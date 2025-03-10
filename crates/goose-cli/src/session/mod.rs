mod builder;
mod completion;
mod input;
mod output;
mod prompt;
mod thinking;

pub use builder::build_session;
pub use goose::session::Identifier;

use anyhow::Result;
use completion::GooseCompleter;
use etcetera::choose_app_strategy;
use etcetera::AppStrategy;
use goose::agents::extension::{Envs, ExtensionConfig};
use goose::agents::{Agent, SessionConfig};
use goose::config::Config;
use goose::message::{Message, MessageContent};
use goose::session;
use mcp_core::handler::ToolError;
use mcp_core::prompt::PromptMessage;

use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio;

use crate::log_usage::log_usage;

pub struct Session {
    agent: Box<dyn Agent>,
    messages: Vec<Message>,
    session_file: PathBuf,
    // Cache for completion data - using std::sync for thread safety without async
    completion_cache: Arc<std::sync::RwLock<CompletionCache>>,
    debug: bool, // New field for debug mode
}

// Cache structure for completion data
struct CompletionCache {
    prompts: HashMap<String, Vec<String>>,
    prompt_info: HashMap<String, output::PromptInfo>,
    last_updated: Instant,
}

impl CompletionCache {
    fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            prompt_info: HashMap::new(),
            last_updated: Instant::now(),
        }
    }
}

impl Session {
    pub fn new(agent: Box<dyn Agent>, session_file: PathBuf, debug: bool) -> Self {
        let messages = match session::read_messages(&session_file) {
            Ok(msgs) => msgs,
            Err(e) => {
                eprintln!("Warning: Failed to load message history: {}", e);
                Vec::new()
            }
        };

        Session {
            agent,
            messages,
            session_file,
            completion_cache: Arc::new(std::sync::RwLock::new(CompletionCache::new())),
            debug,
        }
    }

    /// Add a stdio extension to the session
    ///
    /// # Arguments
    /// * `extension_command` - Full command string including environment variables
    ///   Format: "ENV1=val1 ENV2=val2 command args..."
    pub async fn add_extension(&mut self, extension_command: String) -> Result<()> {
        let mut parts: Vec<&str> = extension_command.split_whitespace().collect();
        let mut envs = std::collections::HashMap::new();

        // Parse environment variables (format: KEY=value)
        while let Some(part) = parts.first() {
            if !part.contains('=') {
                break;
            }
            let env_part = parts.remove(0);
            let (key, value) = env_part.split_once('=').unwrap();
            envs.insert(key.to_string(), value.to_string());
        }

        if parts.is_empty() {
            return Err(anyhow::anyhow!("No command provided in extension string"));
        }

        let cmd = parts.remove(0).to_string();
        // Generate a random name for the ephemeral extension
        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let config = ExtensionConfig::Stdio {
            name,
            cmd,
            args: parts.iter().map(|s| s.to_string()).collect(),
            envs: Envs::new(envs),
            // TODO: should set timeout
            timeout: Some(goose::config::DEFAULT_EXTENSION_TIMEOUT),
        };

        self.agent
            .add_extension(config)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start extension: {}", e))?;

        // Invalidate the completion cache when a new extension is added
        self.invalidate_completion_cache().await;

        Ok(())
    }

    /// Add a builtin extension to the session
    ///
    /// # Arguments
    /// * `builtin_name` - Name of the builtin extension(s), comma separated
    pub async fn add_builtin(&mut self, builtin_name: String) -> Result<()> {
        for name in builtin_name.split(',') {
            let config = ExtensionConfig::Builtin {
                name: name.trim().to_string(),
                // TODO: should set a timeout
                timeout: Some(goose::config::DEFAULT_EXTENSION_TIMEOUT),
            };
            self.agent
                .add_extension(config)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start builtin extension: {}", e))?;
        }

        // Invalidate the completion cache when a new extension is added
        self.invalidate_completion_cache().await;

        Ok(())
    }

    pub async fn list_prompts(
        &mut self,
        extension: Option<String>,
    ) -> Result<HashMap<String, Vec<String>>> {
        let prompts = self.agent.list_extension_prompts().await;

        // Early validation if filtering by extension
        if let Some(filter) = &extension {
            if !prompts.contains_key(filter) {
                return Err(anyhow::anyhow!("Extension '{}' not found", filter));
            }
        }

        // Convert prompts into filtered map of extension names to prompt names
        Ok(prompts
            .into_iter()
            .filter(|(ext, _)| extension.as_ref().is_none_or(|f| f == ext))
            .map(|(extension, prompt_list)| {
                let names = prompt_list.into_iter().map(|p| p.name).collect();
                (extension, names)
            })
            .collect())
    }

    pub async fn get_prompt_info(&mut self, name: &str) -> Result<Option<output::PromptInfo>> {
        let prompts = self.agent.list_extension_prompts().await;

        // Find which extension has this prompt
        for (extension, prompt_list) in prompts {
            if let Some(prompt) = prompt_list.iter().find(|p| p.name == name) {
                return Ok(Some(output::PromptInfo {
                    name: prompt.name.clone(),
                    description: prompt.description.clone(),
                    arguments: prompt.arguments.clone(),
                    extension: Some(extension),
                }));
            }
        }

        Ok(None)
    }

    pub async fn get_prompt(&mut self, name: &str, arguments: Value) -> Result<Vec<PromptMessage>> {
        let result = self.agent.get_prompt(name, arguments).await?;
        Ok(result.messages)
    }

    /// Process a single message and get the response
    async fn process_message(&mut self, message: String) -> Result<()> {
        self.messages.push(Message::user().with_text(&message));
        // Get the provider from the agent for description generation
        let provider = self.agent.provider().await;

        // Persist messages with provider for automatic description generation
        session::persist_messages(&self.session_file, &self.messages, Some(provider)).await?;

        self.process_agent_response(false).await?;
        Ok(())
    }

    /// Start an interactive session, optionally with an initial message
    pub async fn interactive(&mut self, message: Option<String>) -> Result<()> {
        // Process initial message if provided
        if let Some(msg) = message {
            self.process_message(msg).await?;
        }

        // Initialize the completion cache
        self.update_completion_cache().await?;

        // Create a new editor with our custom completer
        let config = rustyline::Config::builder()
            .completion_type(rustyline::CompletionType::Circular)
            .build();
        let mut editor =
            rustyline::Editor::<GooseCompleter, rustyline::history::DefaultHistory>::with_config(
                config,
            )?;

        // Set up the completer with a reference to the completion cache
        let completer = GooseCompleter::new(self.completion_cache.clone());
        editor.set_helper(Some(completer));

        // Create and use a global history file in ~/.config/goose directory
        // This allows command history to persist across different chat sessions
        // instead of being tied to each individual session's messages
        let history_file = choose_app_strategy(crate::APP_STRATEGY.clone())
            .expect("goose requires a home dir")
            .in_config_dir("history.txt");

        // Ensure config directory exists
        if let Some(parent) = history_file.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        // Load history from the global file
        if history_file.exists() {
            if let Err(err) = editor.load_history(&history_file) {
                eprintln!("Warning: Failed to load command history: {}", err);
            }
        }

        // Helper function to save history after commands
        let save_history =
            |editor: &mut rustyline::Editor<GooseCompleter, rustyline::history::DefaultHistory>| {
                if let Err(err) = editor.save_history(&history_file) {
                    eprintln!("Warning: Failed to save command history: {}", err);
                }
            };

        output::display_greeting();
        loop {
            match input::get_input(&mut editor)? {
                input::InputResult::Message(content) => {
                    save_history(&mut editor);

                    self.messages.push(Message::user().with_text(&content));

                    // Get the provider from the agent for description generation
                    let provider = self.agent.provider().await;

                    // Persist messages with provider for automatic description generation
                    session::persist_messages(&self.session_file, &self.messages, Some(provider))
                        .await?;

                    output::show_thinking();
                    self.process_agent_response(true).await?;
                    output::hide_thinking();
                }
                input::InputResult::Exit => break,
                input::InputResult::AddExtension(cmd) => {
                    save_history(&mut editor);

                    match self.add_extension(cmd.clone()).await {
                        Ok(_) => output::render_extension_success(&cmd),
                        Err(e) => output::render_extension_error(&cmd, &e.to_string()),
                    }
                }
                input::InputResult::AddBuiltin(names) => {
                    save_history(&mut editor);

                    match self.add_builtin(names.clone()).await {
                        Ok(_) => output::render_builtin_success(&names),
                        Err(e) => output::render_builtin_error(&names, &e.to_string()),
                    }
                }
                input::InputResult::ToggleTheme => {
                    save_history(&mut editor);

                    let current = output::get_theme();
                    let new_theme = match current {
                        output::Theme::Light => {
                            println!("Switching to Dark theme");
                            output::Theme::Dark
                        }
                        output::Theme::Dark => {
                            println!("Switching to Ansi theme");
                            output::Theme::Ansi
                        }
                        output::Theme::Ansi => {
                            println!("Switching to Light theme");
                            output::Theme::Light
                        }
                    };
                    output::set_theme(new_theme);
                    continue;
                }
                input::InputResult::Retry => continue,
                input::InputResult::ListPrompts(extension) => {
                    save_history(&mut editor);

                    match self.list_prompts(extension).await {
                        Ok(prompts) => output::render_prompts(&prompts),
                        Err(e) => output::render_error(&e.to_string()),
                    }
                }
                input::InputResult::GooseMode(mode) => {
                    save_history(&mut editor);

                    let config = Config::global();
                    let mode = mode.to_lowercase();

                    // Check if mode is valid
                    if !["auto", "approve", "chat"].contains(&mode.as_str()) {
                        output::render_error(&format!(
                            "Invalid mode '{}'. Mode must be one of: auto, approve, chat",
                            mode
                        ));
                        continue;
                    }

                    config
                        .set("GOOSE_MODE", Value::String(mode.to_string()))
                        .unwrap();
                    println!("Goose mode set to '{}'", mode);
                    continue;
                }
                input::InputResult::PromptCommand(opts) => {
                    save_history(&mut editor);

                    // name is required
                    if opts.name.is_empty() {
                        output::render_error("Prompt name argument is required");
                        continue;
                    }

                    if opts.info {
                        match self.get_prompt_info(&opts.name).await? {
                            Some(info) => output::render_prompt_info(&info),
                            None => {
                                output::render_error(&format!("Prompt '{}' not found", opts.name))
                            }
                        }
                    } else {
                        // Convert the arguments HashMap to a Value
                        let arguments = serde_json::to_value(opts.arguments)
                            .map_err(|e| anyhow::anyhow!("Failed to serialize arguments: {}", e))?;

                        match self.get_prompt(&opts.name, arguments).await {
                            Ok(messages) => {
                                let start_len = self.messages.len();
                                let mut valid = true;
                                for (i, prompt_message) in messages.into_iter().enumerate() {
                                    let msg = Message::from(prompt_message);
                                    // ensure we get a User - Assistant - User type pattern
                                    let expected_role = if i % 2 == 0 {
                                        mcp_core::Role::User
                                    } else {
                                        mcp_core::Role::Assistant
                                    };

                                    if msg.role != expected_role {
                                        output::render_error(&format!(
                                            "Expected {:?} message at position {}, but found {:?}",
                                            expected_role, i, msg.role
                                        ));
                                        valid = false;
                                        // get rid of everything we added to messages
                                        self.messages.truncate(start_len);
                                        break;
                                    }

                                    if msg.role == mcp_core::Role::User {
                                        output::render_message(&msg, self.debug);
                                    }
                                    self.messages.push(msg);
                                }

                                if valid {
                                    output::show_thinking();
                                    self.process_agent_response(true).await?;
                                    output::hide_thinking();
                                }
                            }
                            Err(e) => output::render_error(&e.to_string()),
                        }
                    }
                }
            }
        }

        // Log usage and cleanup
        if let Ok(home_dir) = choose_app_strategy(crate::APP_STRATEGY.clone()) {
            let usage = self.agent.usage().await;
            log_usage(
                home_dir,
                self.session_file.to_string_lossy().to_string(),
                usage,
            );
            println!(
                "\nClosing session. Recorded to {}",
                self.session_file.display()
            );
        }
        Ok(())
    }

    /// Process a single message and exit
    pub async fn headless(&mut self, message: String) -> Result<()> {
        self.process_message(message).await
    }

    async fn process_agent_response(&mut self, interactive: bool) -> Result<()> {
        let session_id = session::Identifier::Path(self.session_file.clone());
        let mut stream = self
            .agent
            .reply(
                &self.messages,
                Some(SessionConfig {
                    id: session_id,
                    working_dir: std::env::current_dir()
                        .expect("failed to get current session working directory"),
                }),
            )
            .await?;

        use futures::StreamExt;
        loop {
            tokio::select! {
                result = stream.next() => {
                    match result {
                        Some(Ok(message)) => {
                            // If it's a confirmation request, get approval but otherwise do not render/persist
                            if let Some(MessageContent::ToolConfirmationRequest(confirmation)) = message.content.first() {
                                output::hide_thinking();

                                // Format the confirmation prompt
                                let prompt = "Goose would like to call the above tool, do you approve?".to_string();

                                // Get confirmation from user
                                let confirmed = cliclack::confirm(prompt).initial_value(true).interact()?;
                                self.agent.handle_confirmation(confirmation.id.clone(), confirmed).await;
                            }
                            // otherwise we have a model/tool to render
                            else {
                                self.messages.push(message.clone());

                                // No need to update description on assistant messages
                                session::persist_messages(&self.session_file, &self.messages, None).await?;

                                if interactive {output::hide_thinking()};
                                output::render_message(&message, self.debug);
                                if interactive {output::show_thinking()};
                            }
                        }
                        Some(Err(e)) => {
                            eprintln!("Error: {}", e);
                            drop(stream);
                            if let Err(e) = self.handle_interrupted_messages(false).await {
                                eprintln!("Error handling interruption: {}", e);
                            }
                            output::render_error(
                                "The error above was an exception we were not able to handle.\n\
                                These errors are often related to connection or authentication\n\
                                We've removed the conversation up to the most recent user message\n\
                                - depending on the error you may be able to continue",
                            );
                            break;
                        }
                        None => break,
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    drop(stream);
                    if let Err(e) = self.handle_interrupted_messages(true).await {
                        eprintln!("Error handling interruption: {}", e);
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_interrupted_messages(&mut self, interrupt: bool) -> Result<()> {
        // First, get any tool requests from the last message if it exists
        let tool_requests = self
            .messages
            .last()
            .filter(|msg| msg.role == mcp_core::role::Role::Assistant)
            .map_or(Vec::new(), |msg| {
                msg.content
                    .iter()
                    .filter_map(|content| {
                        if let MessageContent::ToolRequest(req) = content {
                            Some((req.id.clone(), req.tool_call.clone()))
                        } else {
                            None
                        }
                    })
                    .collect()
            });

        if !tool_requests.is_empty() {
            // Interrupted during a tool request
            // Create tool responses for all interrupted tool requests
            let mut response_message = Message::user();
            let last_tool_name = tool_requests
                .last()
                .and_then(|(_, tool_call)| tool_call.as_ref().ok().map(|tool| tool.name.clone()))
                .unwrap_or_else(|| "tool".to_string());

            let notification = if interrupt {
                "Interrupted by the user to make a correction".to_string()
            } else {
                "An uncaught error happened during tool use".to_string()
            };
            for (req_id, _) in &tool_requests {
                response_message.content.push(MessageContent::tool_response(
                    req_id.clone(),
                    Err(ToolError::ExecutionError(notification.clone())),
                ));
            }
            self.messages.push(response_message);

            // No need for description update here
            session::persist_messages(&self.session_file, &self.messages, None).await?;

            let prompt = format!(
                "The existing call to {} was interrupted. How would you like to proceed?",
                last_tool_name
            );
            self.messages.push(Message::assistant().with_text(&prompt));

            // No need for description update here
            session::persist_messages(&self.session_file, &self.messages, None).await?;

            output::render_message(&Message::assistant().with_text(&prompt), self.debug);
        } else {
            // An interruption occurred outside of a tool request-response.
            if let Some(last_msg) = self.messages.last() {
                if last_msg.role == mcp_core::role::Role::User {
                    match last_msg.content.first() {
                        Some(MessageContent::ToolResponse(_)) => {
                            // Interruption occurred after a tool had completed but not assistant reply
                            let prompt = "The tool calling loop was interrupted. How would you like to proceed?";
                            self.messages.push(Message::assistant().with_text(prompt));

                            // No need for description update here
                            session::persist_messages(&self.session_file, &self.messages, None)
                                .await?;

                            output::render_message(
                                &Message::assistant().with_text(prompt),
                                self.debug,
                            );
                        }
                        Some(_) => {
                            // A real users message
                            self.messages.pop();
                            let prompt = "Interrupted before the model replied and removed the last message.";
                            output::render_message(
                                &Message::assistant().with_text(prompt),
                                self.debug,
                            );
                        }
                        None => panic!("No content in last message"),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn session_file(&self) -> PathBuf {
        self.session_file.clone()
    }

    /// Update the completion cache with fresh data
    /// This should be called before the interactive session starts
    pub async fn update_completion_cache(&mut self) -> Result<()> {
        // Get fresh data
        let prompts = self.agent.list_extension_prompts().await;

        // Update the cache with write lock
        let mut cache = self.completion_cache.write().unwrap();
        cache.prompts.clear();
        cache.prompt_info.clear();

        for (extension, prompt_list) in prompts {
            let names: Vec<String> = prompt_list.iter().map(|p| p.name.clone()).collect();
            cache.prompts.insert(extension.clone(), names);

            for prompt in prompt_list {
                cache.prompt_info.insert(
                    prompt.name.clone(),
                    output::PromptInfo {
                        name: prompt.name.clone(),
                        description: prompt.description.clone(),
                        arguments: prompt.arguments.clone(),
                        extension: Some(extension.clone()),
                    },
                );
            }
        }

        cache.last_updated = Instant::now();
        Ok(())
    }

    /// Invalidate the completion cache
    /// This should be called when extensions are added or removed
    async fn invalidate_completion_cache(&self) {
        let mut cache = self.completion_cache.write().unwrap();
        cache.prompts.clear();
        cache.prompt_info.clear();
        cache.last_updated = Instant::now();
    }

    pub fn message_history(&self) -> Vec<Message> {
        self.messages.clone()
    }
}

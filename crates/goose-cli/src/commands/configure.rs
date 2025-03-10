use cliclack::spinner;
use console::style;
use goose::agents::{extension::Envs, ExtensionConfig};
use goose::config::{Config, ConfigError, ExperimentManager, ExtensionEntry, ExtensionManager};
use goose::message::Message;
use goose::providers::{create, providers};
use mcp_core::Tool;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

pub async fn handle_configure() -> Result<(), Box<dyn Error>> {
    let config = Config::global();

    if !config.exists() {
        // First time setup flow
        println!();
        println!(
            "{}",
            style("Welcome to goose! Let's get you set up with a provider.").dim()
        );
        println!(
            "{}",
            style("  you can rerun this command later to update your configuration").dim()
        );
        println!();
        cliclack::intro(style(" goose-configure ").on_cyan().black())?;
        match configure_provider_dialog().await {
            Ok(true) => {
                println!(
                    "\n  {}: Run '{}' again to adjust your config or add extensions",
                    style("Tip").green().italic(),
                    style("goose configure").cyan()
                );
                // Since we are setting up for the first time, we'll also enable the developer system
                // This operation is best-effort and errors are ignored
                ExtensionManager::set(ExtensionEntry {
                    enabled: true,
                    config: ExtensionConfig::Builtin {
                        name: "developer".to_string(),
                        timeout: Some(goose::config::DEFAULT_EXTENSION_TIMEOUT),
                    },
                })?;
            }
            Ok(false) => {
                let _ = config.clear();
                println!(
                    "\n  {}: We did not save your config, inspect your credentials\n   and run '{}' again to ensure goose can connect",
                    style("Warning").yellow().italic(),
                    style("goose configure").cyan()
                );
            }
            Err(e) => {
                let _ = config.clear();

                match e.downcast_ref::<ConfigError>() {
                    Some(ConfigError::NotFound(key)) => {
                        println!(
                            "\n  {} Required configuration key '{}' not found \n  Please provide this value and run '{}' again",
                            style("Error").red().italic(),
                            key,
                            style("goose configure").cyan()
                        );
                    }
                    Some(ConfigError::KeyringError(msg)) => {
                        #[cfg(target_os = "macos")]
                        println!(
                            "\n  {} Failed to access secure storage (keyring): {} \n  Please check your system keychain and run '{}' again. \n  If your system is unable to use the keyring, please try setting secret key(s) via environment variables.",
                            style("Error").red().italic(),
                            msg,
                            style("goose configure").cyan()
                        );

                        #[cfg(target_os = "windows")]
                        println!(
                            "\n  {} Failed to access Windows Credential Manager: {} \n  Please check Windows Credential Manager and run '{}' again. \n  If your system is unable to use the Credential Manager, please try setting secret key(s) via environment variables.",
                            style("Error").red().italic(),
                            msg,
                            style("goose configure").cyan()
                        );

                        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                        println!(
                            "\n  {} Failed to access secure storage: {} \n  Please check your system's secure storage and run '{}' again. \n  If your system is unable to use secure storage, please try setting secret key(s) via environment variables.",
                            style("Error").red().italic(),
                            msg,
                            style("goose configure").cyan()
                        );
                    }
                    Some(ConfigError::DeserializeError(msg)) => {
                        println!(
                            "\n  {} Invalid configuration value: {} \n  Please check your input and run '{}' again",
                            style("Error").red().italic(),
                            msg,
                            style("goose configure").cyan()
                        );
                    }
                    Some(ConfigError::FileError(e)) => {
                        println!(
                            "\n  {} Failed to access config file: {} \n  Please check file permissions and run '{}' again",
                            style("Error").red().italic(),
                            e,
                            style("goose configure").cyan()
                        );
                    }
                    Some(ConfigError::DirectoryError(msg)) => {
                        println!(
                            "\n  {} Failed to access config directory: {} \n  Please check directory permissions and run '{}' again",
                            style("Error").red().italic(),
                            msg,
                            style("goose configure").cyan()
                        );
                    }
                    // handle all other nonspecific errors
                    _ => {
                        println!(
                            "\n  {} {} \n  We did not save your config, inspect your credentials\n   and run '{}' again to ensure goose can connect",
                            style("Error").red().italic(),
                            e,
                            style("goose configure").cyan()
                        );
                    }
                }
            }
        }
        Ok(())
    } else {
        println!();
        println!(
            "{}",
            style("This will update your existing config file").dim()
        );
        println!(
            "{} {}",
            style("  if you prefer, you can edit it directly at").dim(),
            config.path()
        );
        println!();

        cliclack::intro(style(" goose-configure ").on_cyan().black())?;
        let action = cliclack::select("What would you like to configure?")
            .item(
                "providers",
                "Configure Providers",
                "Change provider or update credentials",
            )
            .item("add", "Add Extension", "Connect to a new extension")
            .item(
                "toggle",
                "Toggle Extensions",
                "Enable or disable connected extensions",
            )
            .item("remove", "Remove Extension", "Remove an extension")
            .item(
                "settings",
                "Goose Settings",
                "Set the Goose Mode, Tool Output, Experiment and more",
            )
            .interact()?;

        match action {
            "toggle" => toggle_extensions_dialog(),
            "add" => configure_extensions_dialog(),
            "remove" => remove_extension_dialog(),
            "settings" => configure_settings_dialog(),
            "providers" => configure_provider_dialog().await.and(Ok(())),
            _ => unreachable!(),
        }
    }
}

/// Dialog for configuring the AI provider and model
pub async fn configure_provider_dialog() -> Result<bool, Box<dyn Error>> {
    // Get global config instance
    let config = Config::global();

    // Get all available providers and their metadata
    let available_providers = providers();

    // Create selection items from provider metadata
    let provider_items: Vec<(&String, &str, &str)> = available_providers
        .iter()
        .map(|p| (&p.name, p.display_name.as_str(), p.description.as_str()))
        .collect();

    // Get current default provider if it exists
    let current_provider: Option<String> = config.get("GOOSE_PROVIDER").ok();
    let default_provider = current_provider.unwrap_or_default();

    // Select provider
    let provider_name = cliclack::select("Which model provider should we use?")
        .initial_value(&default_provider)
        .items(&provider_items)
        .interact()?;

    // Get the selected provider's metadata
    let provider_meta = available_providers
        .iter()
        .find(|p| &p.name == provider_name)
        .expect("Selected provider must exist in metadata");

    // Configure required provider keys
    for key in &provider_meta.config_keys {
        if !key.required {
            continue;
        }

        // First check if the value is set via environment variable
        let from_env = std::env::var(&key.name).ok();

        match from_env {
            Some(env_value) => {
                let _ =
                    cliclack::log::info(format!("{} is set via environment variable", key.name));
                if cliclack::confirm("Would you like to save this value to your keyring?")
                    .initial_value(true)
                    .interact()?
                {
                    if key.secret {
                        config.set_secret(&key.name, Value::String(env_value))?;
                    } else {
                        config.set(&key.name, Value::String(env_value))?;
                    }
                    let _ = cliclack::log::info(format!("Saved {} to config file", key.name));
                }
            }
            None => {
                // No env var, check config/secret storage
                let existing: Result<String, _> = if key.secret {
                    config.get_secret(&key.name)
                } else {
                    config.get(&key.name)
                };

                match existing {
                    Ok(_) => {
                        let _ = cliclack::log::info(format!("{} is already configured", key.name));
                        if cliclack::confirm("Would you like to update this value?").interact()? {
                            let new_value: String = if key.secret {
                                cliclack::password(format!("Enter new value for {}", key.name))
                                    .mask('▪')
                                    .interact()?
                            } else {
                                let mut input =
                                    cliclack::input(format!("Enter new value for {}", key.name));
                                if key.default.is_some() {
                                    input = input.default_input(&key.default.clone().unwrap());
                                }
                                input.interact()?
                            };

                            if key.secret {
                                config.set_secret(&key.name, Value::String(new_value))?;
                            } else {
                                config.set(&key.name, Value::String(new_value))?;
                            }
                        }
                    }
                    Err(_) => {
                        let value: String = if key.secret {
                            cliclack::password(format!(
                                "Provider {} requires {}, please enter a value",
                                provider_meta.display_name, key.name
                            ))
                            .mask('▪')
                            .interact()?
                        } else {
                            let mut input = cliclack::input(format!(
                                "Provider {} requires {}, please enter a value",
                                provider_meta.display_name, key.name
                            ));
                            if key.default.is_some() {
                                input = input.default_input(&key.default.clone().unwrap());
                            }
                            input.interact()?
                        };

                        if key.secret {
                            config.set_secret(&key.name, Value::String(value))?;
                        } else {
                            config.set(&key.name, Value::String(value))?;
                        }
                    }
                }
            }
        }
    }

    // Select model, defaulting to the provider's recommended model UNLESS there is an env override
    let default_model = std::env::var("GOOSE_MODEL").unwrap_or(provider_meta.default_model.clone());
    let model: String = cliclack::input("Enter a model from that provider:")
        .default_input(&default_model)
        .interact()?;

    // Test the configuration
    let spin = spinner();
    spin.start("Checking your configuration...");

    // Use max tokens to speed up the provider test.
    let model_config = goose::model::ModelConfig::new(model.clone()).with_max_tokens(Some(50));
    let provider = create(provider_name, model_config)?;

    let messages =
        vec![Message::user().with_text("What is the weather like in San Francisco today?")];
    let sample_tool = Tool::new(
        "get_weather".to_string(),
        "Get current temperature for a given location.".to_string(),
        json!({
            "type": "object",
            "required": ["location"],
            "properties": {
                "location": {"type": "string"}
            }
        }),
    );

    let result = provider
        .complete(
            "You are an AI agent called Goose. You use tools of connected extensions to solve problems.",
            &messages,
            &[sample_tool]
        )
        .await;

    match result {
        Ok((_message, _usage)) => {
            // Update config with new values only if the test succeeds
            config.set("GOOSE_PROVIDER", Value::String(provider_name.to_string()))?;
            config.set("GOOSE_MODEL", Value::String(model.clone()))?;
            cliclack::outro("Configuration saved successfully")?;
            Ok(true)
        }
        Err(e) => {
            spin.stop(style(e.to_string()).red());
            cliclack::outro(style("Failed to configure provider: init chat completion request with tool did not succeed.").on_red().white())?;
            Ok(false)
        }
    }
}

/// Configure extensions that can be used with goose
/// Dialog for toggling which extensions are enabled/disabled
pub fn toggle_extensions_dialog() -> Result<(), Box<dyn Error>> {
    let extensions = ExtensionManager::get_all()?;

    if extensions.is_empty() {
        cliclack::outro(
            "No extensions configured yet. Run configure and add some extensions first.",
        )?;
        return Ok(());
    }

    // Create a list of extension names and their enabled status
    let extension_status: Vec<(String, bool)> = extensions
        .iter()
        .map(|entry| (entry.config.name().to_string(), entry.enabled))
        .collect();

    // Get currently enabled extensions for the selection
    let enabled_extensions: Vec<&String> = extension_status
        .iter()
        .filter(|(_, enabled)| *enabled)
        .map(|(name, _)| name)
        .collect();

    // Let user toggle extensions
    let selected = cliclack::multiselect(
        "enable extensions: (use \"space\" to toggle and \"enter\" to submit)",
    )
    .required(false)
    .items(
        &extension_status
            .iter()
            .map(|(name, _)| (name, name.as_str(), ""))
            .collect::<Vec<_>>(),
    )
    .initial_values(enabled_extensions)
    .interact()?;

    // Update enabled status for each extension
    for name in extension_status.iter().map(|(name, _)| name) {
        ExtensionManager::set_enabled(name, selected.iter().any(|s| s.as_str() == name))?;
    }

    cliclack::outro("Extension settings updated successfully")?;
    Ok(())
}

pub fn configure_extensions_dialog() -> Result<(), Box<dyn Error>> {
    let extension_type = cliclack::select("What type of extension would you like to add?")
        .item(
            "built-in",
            "Built-in Extension",
            "Use an extension that comes with Goose",
        )
        .item(
            "stdio",
            "Command-line Extension",
            "Run a local command or script",
        )
        .item(
            "sse",
            "Remote Extension",
            "Connect to a remote extension via SSE",
        )
        .interact()?;

    match extension_type {
        // TODO we'll want a place to collect all these options, maybe just an enum in goose-mcp
        "built-in" => {
            let extension = cliclack::select("Which built-in extension would you like to enable?")
                .item(
                    "developer",
                    "Developer Tools",
                    "Code editing and shell access",
                )
                .item(
                    "computercontroller",
                    "Computer Controller",
                    "controls for webscraping, file caching, and automations",
                )
                .item(
                    "google_drive",
                    "Google Drive",
                    "Search and read content from google drive - additional config required",
                )
                .item(
                    "memory",
                    "Memory",
                    "Tools to save and retrieve durable memories",
                )
                .item(
                    "tutorial",
                    "Tutorial",
                    "Access interactive tutorials and guides",
                )
                .item("jetbrains", "JetBrains", "Connect to jetbrains IDEs")
                .interact()?
                .to_string();

            let timeout: u64 = cliclack::input("Please set the timeout for this tool (in secs):")
                .placeholder(&goose::config::DEFAULT_EXTENSION_TIMEOUT.to_string())
                .validate(|input: &String| match input.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Please enter a valide timeout"),
                })
                .interact()?;

            ExtensionManager::set(ExtensionEntry {
                enabled: true,
                config: ExtensionConfig::Builtin {
                    name: extension.clone(),
                    timeout: Some(timeout),
                },
            })?;

            cliclack::outro(format!("Enabled {} extension", style(extension).green()))?;
        }
        "stdio" => {
            let extensions = ExtensionManager::get_all_names()?;
            let name: String = cliclack::input("What would you like to call this extension?")
                .placeholder("my-extension")
                .validate(move |input: &String| {
                    if input.is_empty() {
                        Err("Please enter a name")
                    } else if extensions.contains(input) {
                        Err("An extension with this name already exists")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let command_str: String = cliclack::input("What command should be run?")
                .placeholder("npx -y @block/gdrive")
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a command")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let timeout: u64 = cliclack::input("Please set the timeout for this tool (in secs):")
                .placeholder(&goose::config::DEFAULT_EXTENSION_TIMEOUT.to_string())
                .validate(|input: &String| match input.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Please enter a valide timeout"),
                })
                .interact()?;

            // Split the command string into command and args
            let mut parts = command_str.split_whitespace();
            let cmd = parts.next().unwrap_or("").to_string();
            let args: Vec<String> = parts.map(String::from).collect();

            let add_env =
                cliclack::confirm("Would you like to add environment variables?").interact()?;

            let mut envs = HashMap::new();
            if add_env {
                loop {
                    let key: String = cliclack::input("Environment variable name:")
                        .placeholder("API_KEY")
                        .interact()?;

                    let value: String = cliclack::password("Environment variable value:")
                        .mask('▪')
                        .interact()?;

                    envs.insert(key, value);

                    if !cliclack::confirm("Add another environment variable?").interact()? {
                        break;
                    }
                }
            }

            ExtensionManager::set(ExtensionEntry {
                enabled: true,
                config: ExtensionConfig::Stdio {
                    name: name.clone(),
                    cmd,
                    args,
                    envs: Envs::new(envs),
                    timeout: Some(timeout),
                },
            })?;

            cliclack::outro(format!("Added {} extension", style(name).green()))?;
        }
        "sse" => {
            let extensions = ExtensionManager::get_all_names()?;
            let name: String = cliclack::input("What would you like to call this extension?")
                .placeholder("my-remote-extension")
                .validate(move |input: &String| {
                    if input.is_empty() {
                        Err("Please enter a name")
                    } else if extensions.contains(input) {
                        Err("An extension with this name already exists")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let uri: String = cliclack::input("What is the SSE endpoint URI?")
                .placeholder("http://localhost:8000/events")
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a URI")
                    } else if !input.starts_with("http") {
                        Err("URI should start with http:// or https://")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let timeout: u64 = cliclack::input("Please set the timeout for this tool (in secs):")
                .placeholder(&goose::config::DEFAULT_EXTENSION_TIMEOUT.to_string())
                .validate(|input: &String| match input.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Please enter a valide timeout"),
                })
                .interact()?;

            let add_env =
                cliclack::confirm("Would you like to add environment variables?").interact()?;

            let mut envs = HashMap::new();
            if add_env {
                loop {
                    let key: String = cliclack::input("Environment variable name:")
                        .placeholder("API_KEY")
                        .interact()?;

                    let value: String = cliclack::password("Environment variable value:")
                        .mask('▪')
                        .interact()?;

                    envs.insert(key, value);

                    if !cliclack::confirm("Add another environment variable?").interact()? {
                        break;
                    }
                }
            }

            ExtensionManager::set(ExtensionEntry {
                enabled: true,
                config: ExtensionConfig::Sse {
                    name: name.clone(),
                    uri,
                    envs: Envs::new(envs),
                    timeout: Some(timeout),
                },
            })?;

            cliclack::outro(format!("Added {} extension", style(name).green()))?;
        }
        _ => unreachable!(),
    };

    Ok(())
}

pub fn remove_extension_dialog() -> Result<(), Box<dyn Error>> {
    let extensions = ExtensionManager::get_all()?;

    // Create a list of extension names and their enabled status
    let extension_status: Vec<(String, bool)> = extensions
        .iter()
        .map(|entry| (entry.config.name().to_string(), entry.enabled))
        .collect();

    if extensions.is_empty() {
        cliclack::outro(
            "No extensions configured yet. Run configure and add some extensions first.",
        )?;
        return Ok(());
    }

    // Check if all extensions are enabled
    if extension_status.iter().all(|(_, enabled)| *enabled) {
        cliclack::outro(
            "All extensions are currently enabled. You must first disable extensions before removing them.",
        )?;
        return Ok(());
    }

    let selected = cliclack::multiselect("Select extensions to remove (note: you can only remove disabled extensions - use \"space\" to toggle and \"enter\" to submit)")
        .required(false)
        .items(
            &extension_status
                .iter()
                .filter(|(_, enabled)| !enabled)
                .map(|(name, _)| (name, name.as_str(), ""))
                .collect::<Vec<_>>(),
        )
        .interact()?;

    for name in selected {
        ExtensionManager::remove(name)?;
        cliclack::outro(format!("Removed {} extension", style(name).green()))?;
    }

    Ok(())
}

pub fn configure_settings_dialog() -> Result<(), Box<dyn Error>> {
    let setting_type = cliclack::select("What setting would you like to configure?")
        .item("goose_mode", "Goose Mode", "Configure Goose mode")
        .item(
            "tool_output",
            "Tool Output",
            "Show more or less tool output",
        )
        .item(
            "experiment",
            "Toggle Experiment",
            "Enable or disable an experiment feature",
        )
        .interact()?;

    match setting_type {
        "goose_mode" => {
            configure_goose_mode_dialog()?;
        }
        "tool_output" => {
            configure_tool_output_dialog()?;
        }
        "experiment" => {
            toggle_experiments_dialog()?;
        }
        _ => unreachable!(),
    };

    Ok(())
}

pub fn configure_goose_mode_dialog() -> Result<(), Box<dyn Error>> {
    let config = Config::global();

    // Check if GOOSE_MODE is set as an environment variable
    if std::env::var("GOOSE_MODE").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_MODE environment variable is set and will override the configuration here.");
    }

    let mode = cliclack::select("Which Goose mode would you like to configure?")
        .item(
            "auto",
            "Auto Mode", 
            "Full file modification, extension usage, edit, create and delete files freely"
        )
        .item(
            "approve",
            "Approve Mode",
            "Editing, creating, deleting files and using extensions will require human approval"
        )
        .item(
            "chat",
            "Chat Mode",
            "Engage with the selected provider without using tools, extensions, or file modification"
        )
        .interact()?;

    match mode {
        "auto" => {
            config.set("GOOSE_MODE", Value::String("auto".to_string()))?;
            cliclack::outro("Set to Auto Mode - full file modification enabled")?;
        }
        "approve" => {
            config.set("GOOSE_MODE", Value::String("approve".to_string()))?;
            cliclack::outro("Set to Approve Mode - modifications require approval")?;
        }
        "chat" => {
            config.set("GOOSE_MODE", Value::String("chat".to_string()))?;
            cliclack::outro("Set to Chat Mode - no tools or modifications enabled")?;
        }
        _ => unreachable!(),
    };
    Ok(())
}

pub fn configure_tool_output_dialog() -> Result<(), Box<dyn Error>> {
    let config = Config::global();
    // Check if GOOSE_CLI_MIN_PRIORITY is set as an environment variable
    if std::env::var("GOOSE_CLI_MIN_PRIORITY").is_ok() {
        let _ = cliclack::log::info("Notice: GOOSE_CLI_MIN_PRIORITY environment variable is set and will override the configuration here.");
    }
    let tool_log_level = cliclack::select("Which tool output would you like to show?")
        .item("high", "High Importance", "")
        .item("medium", "Medium Importance", "Ex. results of file-writes")
        .item("all", "All (default)", "Ex. shell command output")
        .interact()?;

    match tool_log_level {
        "high" => {
            config.set("GOOSE_CLI_MIN_PRIORITY", Value::from(0.8))?;
            cliclack::outro("Showing tool output of high importance only.")?;
        }
        "medium" => {
            config.set("GOOSE_CLI_MIN_PRIORITY", Value::from(0.2))?;
            cliclack::outro("Showing tool output of medium importance.")?;
        }
        "all" => {
            config.set("GOOSE_CLI_MIN_PRIORITY", Value::from(0.0))?;
            cliclack::outro("Showing all tool output.")?;
        }
        _ => unreachable!(),
    };

    Ok(())
}

/// Configure experiment features that can be used with goose
/// Dialog for toggling which experiments are enabled/disabled
pub fn toggle_experiments_dialog() -> Result<(), Box<dyn Error>> {
    let experiments = ExperimentManager::get_all()?;

    if experiments.is_empty() {
        cliclack::outro("No experiments supported yet.")?;
        return Ok(());
    }

    // Get currently enabled experiments for the selection
    let enabled_experiments: Vec<&String> = experiments
        .iter()
        .filter(|(_, enabled)| *enabled)
        .map(|(name, _)| name)
        .collect();

    // Let user toggle experiments
    let selected = cliclack::multiselect(
        "enable experiments: (use \"space\" to toggle and \"enter\" to submit)",
    )
    .required(false)
    .items(
        &experiments
            .iter()
            .map(|(name, _)| (name, name.as_str(), ""))
            .collect::<Vec<_>>(),
    )
    .initial_values(enabled_experiments)
    .interact()?;

    // Update enabled status for each experiments
    for name in experiments.iter().map(|(name, _)| name) {
        ExperimentManager::set_enabled(name, selected.iter().any(|&s| s.as_str() == name))?;
    }

    cliclack::outro("Experiments settings updated successfully")?;
    Ok(())
}

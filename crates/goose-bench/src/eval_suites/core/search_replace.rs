use crate::bench_work_dir::BenchmarkWorkDir;
use crate::eval_suites::{BenchAgent, Evaluation, EvaluationMetric, ExtensionRequirements};
use crate::register_evaluation;
use async_trait::async_trait;
use std::fs;

#[derive(Debug)]
pub struct DeveloperSearchReplace {}

impl DeveloperSearchReplace {
    pub fn new() -> Self {
        DeveloperSearchReplace {}
    }
}

#[async_trait]
impl Evaluation for DeveloperSearchReplace {
    async fn run(
        &self,
        mut agent: Box<dyn BenchAgent>,
        work_dir: &mut BenchmarkWorkDir,
    ) -> anyhow::Result<Vec<(String, EvaluationMetric)>> {
        let mut metrics = Vec::new();

        let _target_file = match work_dir.fs_get("./assets/kubernetes_swagger.json".to_string()) {
            Ok(file) => file,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Could not find kubernetes_swagger.json file"
                ))
            }
        };
        let mut source_file = work_dir.base_path.clone();
        source_file.push("assets/kubernetes_swagger.json");

        // Send the prompt to modify the file
        let _messages = agent.prompt("Remove the io.k8s.api.admissionregistration.v1.ServiceReference definition block and replace with a new definition for io.k8s.api.admissionregistration.v1.FakeServiceReference. Update the fields in the definition as well to be consistent. Don't change the property names. Don't update any references to the old definition. Only modify the definition and it's description to 'FakeServiceReference simulates a reference to a fake service for testing purposes.'.The file to modify is kubernetes_swagger.json.".to_string()).await?;

        // Get the path to the modified file
        let modified_file_path = std::env::current_dir()
            .unwrap_or_default()
            .join("kubernetes_swagger.json");

        // Read the expected patch file from the assets directory
        let patch_file_path = work_dir.base_path.join("assets").join("kubernetes.patch");
        if !patch_file_path.exists() {
            return Err(anyhow::anyhow!("Could not find patch file"));
        }
        let patch_content = fs::read_to_string(&patch_file_path)?
            .lines()
            .skip(4)
            .collect::<Vec<&str>>()
            .join("\n");

        // Run git diff between modified and source files
        let diff_output = std::process::Command::new("git")
            .args([
                "diff",
                "--no-index",
                source_file.to_str().unwrap(),
                modified_file_path.to_str().unwrap(),
            ])
            .output()?;

        let actual_diff = String::from_utf8_lossy(&diff_output.stdout)
            .to_string()
            .lines()
            .skip(4)
            .collect::<Vec<&str>>()
            .join("\n");

        let mut changes_match = true;

        // Compare the remaining lines
        if actual_diff != patch_content {
            println!("Diffs don't match!");
            println!("Expected patch:\n{}", patch_content);
            println!("Actual diff:\n{}", actual_diff);
            changes_match = false;
        }

        metrics.push((
            "Changes match expected patch".to_string(),
            EvaluationMetric::Boolean(changes_match),
        ));

        Ok(metrics)
    }

    fn name(&self) -> &str {
        "developer_search_replace"
    }

    fn required_extensions(&self) -> ExtensionRequirements {
        ExtensionRequirements {
            builtin: vec!["developer".to_string()],
            external: Vec::new(),
        }
    }
}

register_evaluation!("developer_search_replace", DeveloperSearchReplace);

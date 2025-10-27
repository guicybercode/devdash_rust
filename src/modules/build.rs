use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildStatus {
    Success,
    Failure,
    Running,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub status: BuildStatus,
    pub name: String,
    pub timestamp: String,
    pub commit_sha: String,
    pub message: String,
}

#[derive(Deserialize)]
struct GitHubWorkflowRun {
    status: String,
    conclusion: Option<String>,
    name: String,
    updated_at: String,
    head_sha: String,
    head_commit: Option<GitHubCommit>,
}

#[derive(Deserialize)]
struct GitHubCommit {
    message: String,
}

#[derive(Deserialize)]
struct GitHubResponse {
    workflow_runs: Vec<GitHubWorkflowRun>,
}

pub struct BuildModule;

impl BuildModule {
    pub async fn get_github_status(
        repo: &str,
        token: &str,
    ) -> Result<Vec<BuildInfo>> {
        let url = format!("https://api.github.com/repos/{}/actions/runs?per_page=10", repo);
        let client = reqwest::Client::new();
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;
        
        let data: GitHubResponse = response.json().await?;
        
        let mut builds = Vec::new();
        for run in data.workflow_runs {
            let status = match (&run.status[..], run.conclusion.as_deref()) {
                ("completed", Some("success")) => BuildStatus::Success,
                ("completed", Some("failure")) => BuildStatus::Failure,
                ("in_progress", _) | ("queued", _) => BuildStatus::Running,
                _ => BuildStatus::Unknown,
            };
            
            builds.push(BuildInfo {
                status,
                name: run.name,
                timestamp: run.updated_at,
                commit_sha: run.head_sha.chars().take(7).collect(),
                message: run.head_commit
                    .map(|c| c.message.lines().next().unwrap_or("").to_string())
                    .unwrap_or_default(),
            });
        }
        
        Ok(builds)
    }
    
    pub async fn get_gitlab_status(
        project_id: &str,
        token: &str,
    ) -> Result<Vec<BuildInfo>> {
        let url = format!("https://gitlab.com/api/v4/projects/{}/pipelines?per_page=10", project_id);
        let client = reqwest::Client::new();
        
        let response = client
            .get(&url)
            .header("PRIVATE-TOKEN", token)
            .send()
            .await?;
        
        let pipelines: Vec<serde_json::Value> = response.json().await?;
        
        let mut builds = Vec::new();
        for pipeline in pipelines {
            let status_str = pipeline["status"].as_str().unwrap_or("unknown");
            let status = match status_str {
                "success" => BuildStatus::Success,
                "failed" => BuildStatus::Failure,
                "running" => BuildStatus::Running,
                _ => BuildStatus::Unknown,
            };
            
            builds.push(BuildInfo {
                status,
                name: "Pipeline".to_string(),
                timestamp: pipeline["updated_at"].as_str().unwrap_or("").to_string(),
                commit_sha: pipeline["sha"]
                    .as_str()
                    .map(|s| s.chars().take(7).collect())
                    .unwrap_or_default(),
                message: String::new(),
            });
        }
        
        Ok(builds)
    }
}

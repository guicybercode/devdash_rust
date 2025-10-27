use anyhow::Result;
use git2::Repository;

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub branch: String,
    pub commits_ahead: usize,
    pub commits_behind: usize,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub last_commit_message: String,
    pub last_commit_author: String,
}

pub struct GitModule;

impl GitModule {
    pub fn get_status(repo_path: &str) -> Result<GitStatus> {
        let repo = Repository::open(repo_path)?;
        
        let head = repo.head()?;
        let branch_name = head.shorthand().unwrap_or("detached").to_string();
        
        let mut commits_ahead = 0;
        let mut commits_behind = 0;
        
        if let Ok(upstream) = repo.branch_upstream_name(&head.name().unwrap()) {
            let upstream_ref = format!("refs/remotes/{}", upstream.as_str().unwrap_or(""));
            if let Ok(upstream_oid) = repo.refname_to_id(&upstream_ref) {
                let (ahead, behind) = repo.graph_ahead_behind(head.target().unwrap(), upstream_oid)?;
                commits_ahead = ahead;
                commits_behind = behind;
            }
        }
        
        let mut staged = 0;
        let mut unstaged = 0;
        let mut untracked = 0;
        
        let mut options = git2::StatusOptions::new();
        options.include_untracked(true);
        
        if let Ok(statuses) = repo.statuses(Some(&mut options)) {
            for entry in statuses.iter() {
                let status = entry.status();
                if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                    staged += 1;
                }
                if status.is_wt_new() || status.is_wt_modified() || status.is_wt_deleted() {
                    unstaged += 1;
                }
                if status.is_wt_new() {
                    untracked += 1;
                }
            }
        }
        
        let last_commit_message = if let Ok(commit) = head.peel_to_commit() {
            commit.message().unwrap_or("").lines().next().unwrap_or("").to_string()
        } else {
            String::new()
        };
        
        let last_commit_author = if let Ok(commit) = head.peel_to_commit() {
            commit.author().name().unwrap_or("").to_string()
        } else {
            String::new()
        };
        
        Ok(GitStatus {
            branch: branch_name,
            commits_ahead,
            commits_behind,
            staged,
            unstaged,
            untracked,
            last_commit_message,
            last_commit_author,
        })
    }
}

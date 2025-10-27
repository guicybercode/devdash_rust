use crate::{
    config::Config,
    modules::{
        build::BuildInfo,
        coverage::CoverageInfo,
        git::GitStatus,
        system::SystemStats,
        timer::Timer,
    },
    storage::TimerData,
};

pub struct App {
    pub config: Config,
    pub git_status: GitStatus,
    pub builds: Vec<BuildInfo>,
    pub coverage: CoverageInfo,
    pub system_stats: SystemStats,
    pub timer: Timer,
    pub timer_data: TimerData,
    pub logs: Vec<String>,
    pub should_quit: bool,
    pub current_tab: usize,
    pub focused_panel: usize,
    pub show_help: bool,
    pub last_git_hash: String,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let timer_data = TimerData::load();
        
        Ok(App {
            config,
            git_status: GitStatus::default(),
            builds: Vec::new(),
            coverage: CoverageInfo::default(),
            system_stats: SystemStats::default(),
            timer: Timer::new(25),
            timer_data,
            logs: Vec::new(),
            should_quit: false,
            current_tab: 0,
            focused_panel: 0,
            show_help: false,
            last_git_hash: String::new(),
        })
    }
    
    pub fn project_name(&self) -> String {
        if let Ok(repo) = git2::Repository::open(&self.config.repo_path) {
            if let Ok(remote) = repo.find_remote("origin") {
                if let Some(url) = remote.url() {
                    if let Some(name) = url.split('/').last() {
                        return name.trim_end_matches(".git").to_string();
                    }
                }
            }
        }
        
        if let Ok(repo) = git2::Repository::open(&self.config.repo_path) {
            if let Some(path) = repo.workdir() {
                if let Some(name) = path.file_name() {
                    return name.to_string_lossy().to_string();
                }
            }
        }
        
        "rust-project".to_string()
    }
    
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
    pub fn add_log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 10 {
            self.logs.remove(0);
        }
    }
}

impl Default for GitStatus {
    fn default() -> Self {
        GitStatus {
            branch: "unknown".to_string(),
            commits_ahead: 0,
            commits_behind: 0,
            staged: 0,
            unstaged: 0,
            untracked: 0,
            last_commit_message: String::new(),
            last_commit_author: String::new(),
        }
    }
}

impl Default for CoverageInfo {
    fn default() -> Self {
        CoverageInfo {
            total_coverage: 0.0,
            files: Vec::new(),
        }
    }
}

impl Default for SystemStats {
    fn default() -> Self {
        SystemStats {
            cpu_usage: 0.0,
            ram_used: 0,
            ram_total: 1,
            disk_used: 0,
            disk_total: 1,
            uptime: 0,
        }
    }
}

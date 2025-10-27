use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod app;
mod config;
mod modules;
mod storage;
mod ui;

use app::App;
use config::Config;
use modules::{
    build::BuildModule,
    coverage::CoverageModule,
    git::GitModule,
    system::SystemModule,
};
use ui::{layout, themes::ThemeManager};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load();
    let mut app = App::new(config)?;
    let mut theme_manager = ThemeManager::new();
    let mut system_module = SystemModule::new();
    
    app.add_log("DevDash started".to_string());
    app.add_log("Loading data...".to_string());
    
    let mut terminal = init_terminal()?;
    
    let mut last_update = std::time::Instant::now();
    
    refresh_data(&mut app, &mut system_module).await;
    
    loop {
        terminal.draw(|f| {
            layout::render_dashboard(f, &app, theme_manager.current_theme());
        })?;
        
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Esc => app.quit(),
                        KeyCode::Tab => app.current_tab = (app.current_tab + 1) % 4,
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            refresh_data(&mut app, &mut system_module).await;
                        }
                        KeyCode::Char(' ') => {
                            handle_timer(&mut app);
                        }
                        KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            theme_manager.cycle_theme();
                        }
                        _ => {}
                    }
                }
            }
        }
        
        if last_update.elapsed().as_secs() >= 2 {
            refresh_data(&mut app, &mut system_module).await;
            last_update = std::time::Instant::now();
        }
        
        if app.should_quit {
            break;
        }
    }
    
    restore_terminal(&mut terminal)?;
    Ok(())
}

async fn refresh_data(app: &mut App, system_module: &mut SystemModule) {
    if let Ok(stats) = system_module.get_stats() {
        app.system_stats = stats;
    }
    
    if let Ok(git_status) = GitModule::get_status(&app.config.repo_path) {
        app.git_status = git_status;
    }
    
    if let Ok(coverage) = CoverageModule::get_coverage() {
        app.coverage = coverage;
    }
    
    if let (Some(repo), Some(token)) = (&app.config.github_repo, &app.config.github_token) {
        if let Ok(builds) = BuildModule::get_github_status(repo, token).await {
            app.builds = builds;
        }
    } else if let (Some(project_id), Some(token)) = (&app.config.gitlab_project_id, &app.config.gitlab_token) {
        if let Ok(builds) = BuildModule::get_gitlab_status(project_id, token).await {
            app.builds = builds;
        }
    }
    
    let completed = app.timer.update();
    if completed {
        app.add_log("Timer completed!".to_string());
        
        let session = crate::storage::TimerSession {
            start: chrono::Utc::now(),
            duration_min: app.timer.duration_minutes(),
            tag: app.timer.get_tag().clone(),
            completed: true,
        };
        
        if let Err(e) = app.timer_data.add_session(session) {
            app.add_log(format!("Failed to save session: {}", e));
        }
    }
}

fn handle_timer(app: &mut App) {
    match app.timer.state() {
        crate::modules::timer::TimerState::Idle => {
            app.timer.start();
            app.add_log("Timer started".to_string());
        }
        crate::modules::timer::TimerState::Running => {
            app.timer.pause();
            app.add_log("Timer paused".to_string());
        }
        crate::modules::timer::TimerState::Paused => {
            app.timer.start();
            app.add_log("Timer resumed".to_string());
        }
        crate::modules::timer::TimerState::Completed => {
            app.timer.reset();
            app.add_log("Timer reset".to_string());
        }
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

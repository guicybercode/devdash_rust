use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::ui::themes::Theme;
use super::widgets;

pub fn render_dashboard(frame: &mut Frame, app: &crate::app::App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());
    
    render_header(frame, chunks[0], app, theme);
    render_body(frame, chunks[1], app, theme);
    render_footer(frame, chunks[2], theme);
}

fn render_header(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let title = format!("DevDash - Painel do Projeto: {}", &app.project_name());
    
    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .style(theme.primary())
        .block(Block::default().style(theme.surface()).borders(Borders::NONE));
    
    frame.render_widget(header, area);
}

fn render_body(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(7),
            Constraint::Length(6),
        ])
        .split(area);
    
    render_status_panels(frame, chunks[0], app, theme);
    render_system_stats(frame, chunks[1], app, theme);
    render_logs(frame, chunks[2], app, theme);
}

fn render_system_stats(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let stats = &app.system_stats;
    let ram_percent = if stats.ram_total > 0 {
        (stats.ram_used as f64 / stats.ram_total as f64) * 100.0
    } else {
        0.0
    };
    let disk_percent = if stats.disk_total > 0 {
        (stats.disk_used as f64 / stats.disk_total as f64) * 100.0
    } else {
        0.0
    };
    
    let uptime_minutes = stats.uptime / 60;
    let uptime_hours = uptime_minutes / 60;
    
    let content = format!(
        "CPU: {:.1}%\nRAM: {:.0}MB / {:.0}MB ({:.1}%)\nDisk: {:.0}MB / {:.0}MB ({:.1}%)\nUptime: {}h {}m",
        stats.cpu_usage,
        stats.ram_used / 1024 / 1024,
        stats.ram_total / 1024 / 1024,
        ram_percent,
        stats.disk_used / 1024 / 1024,
        stats.disk_total / 1024 / 1024,
        disk_percent,
        uptime_hours,
        uptime_minutes % 60
    );
    
    let widget = widgets::status_box("System Stats", &content, theme, true);
    frame.render_widget(widget, area);
}

fn render_status_panels(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);
    
    render_git_status(frame, chunks[0], app, theme);
    render_build_status(frame, chunks[1], app, theme);
    render_coverage_status(frame, chunks[2], app, theme);
    render_timer_status(frame, chunks[3], app, theme);
}

fn render_git_status(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let focused = app.focused_panel == 0;
    let git_info = &app.git_status;
    let commit_msg = if git_info.last_commit_message.is_empty() {
        "No commits".to_string()
    } else if git_info.last_commit_message.len() > 40 {
        format!("{}...", &git_info.last_commit_message[..37])
    } else {
        git_info.last_commit_message.clone()
    };
    
    let content = format!(
        "Branch: {}\nCommits ^: {}\nCommits v: {}\nStaged: {}\nUnstaged: {}\nUntracked: {}\nLast: {}",
        git_info.branch, git_info.commits_ahead, git_info.commits_behind,
        git_info.staged, git_info.unstaged, git_info.untracked,
        commit_msg
    );
    
    let widget = widgets::status_box("Git Status", &content, theme, focused);
    frame.render_widget(widget, area);
}

fn render_build_status(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let focused = app.focused_panel == 1;
    let builds = &app.builds;
    let content = if builds.is_empty() {
        "No builds".to_string()
    } else {
        let status_indicator = match builds[0].status {
            crate::modules::build::BuildStatus::Success => "[OK]",
            crate::modules::build::BuildStatus::Failure => "[FAIL]",
            crate::modules::build::BuildStatus::Running => "[RUN]",
            crate::modules::build::BuildStatus::Unknown => "[?]",
        };
        let time_str = if builds[0].timestamp.is_empty() {
            "now".to_string()
        } else if builds[0].timestamp.len() > 19 {
            builds[0].timestamp[..19].to_string()
        } else {
            builds[0].timestamp.clone()
        };
        format!("Status: {}\nName: {}\nTime: {}", status_indicator, builds[0].name, time_str)
    };
    
    let widget = widgets::status_box("Build Status", &content, theme, focused);
    frame.render_widget(widget, area);
}

fn render_coverage_status(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let coverage = app.coverage.total_coverage;
    let files_count = app.coverage.files.len();
    
    let bar_length: usize = 20;
    let filled = ((coverage / 100.0) * bar_length as f64) as usize;
    let bar = format!("[{}{}]", 
        "=".repeat(filled), 
        " ".repeat(bar_length.saturating_sub(filled))
    );
    
    let content = format!("Coverage: {:.1}%\nFiles: {}\n{}", coverage, files_count, bar);
    
    let widget = widgets::status_box("Test Coverage", &content, theme, false);
    frame.render_widget(widget, area);
}

fn render_timer_status(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let timer = &app.timer;
    let remaining = timer.remaining_seconds();
    let minutes = remaining / 60;
    let seconds = remaining % 60;
    let state_text = match timer.state() {
        crate::modules::timer::TimerState::Idle => "Idle",
        crate::modules::timer::TimerState::Running => "Running",
        crate::modules::timer::TimerState::Paused => "Paused",
        crate::modules::timer::TimerState::Completed => "Completed",
    };
    
    let content = format!("Timer: {:02}:{:02}\nState: {}", minutes, seconds, state_text);
    
    let widget = widgets::status_box("Timer", &content, theme, false);
    frame.render_widget(widget, area);
}

fn render_logs(frame: &mut Frame, area: Rect, app: &crate::app::App, theme: &Theme) {
    let items: Vec<ListItem> = app
        .logs
        .iter()
        .map(|log| ListItem::new(log.clone()).style(theme.text()))
        .collect();
    
    let list = List::new(items)
        .block(Block::default().title("Recent Logs").borders(Borders::ALL).border_style(theme.text_secondary()).style(theme.surface()));
    
    frame.render_widget(list, area);
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let widget = widgets::footer(theme);
    frame.render_widget(widget, area);
}

pub fn add_log(app: &mut crate::app::App, message: String) {
    app.add_log(message);
}

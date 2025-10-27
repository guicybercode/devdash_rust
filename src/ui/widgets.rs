use ratatui::{
    style::{Style, Modifier},
    widgets::{Block, Borders, Paragraph, Wrap, BorderType},
};

use super::themes::Theme;

pub fn status_box<'a>(title: &'a str, content: &'a str, theme: &Theme, bold_title: bool) -> Paragraph<'a> {
    let title_style = if bold_title {
        Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
    } else {
        theme.primary()
    };
    
    Paragraph::new(content)
        .block(
            Block::default()
                .title(title)
                .title_style(title_style)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.primary))
                .style(theme.surface()),
        )
        .style(theme.text())
        .wrap(Wrap { trim: true })
}

pub fn footer<'a>(theme: &Theme) -> Paragraph<'a> {
    let shortcuts = "[Q]uit | [Tab]Navigate | [Ctrl+T]Theme | [Space]Timer | [R]efresh";
    let branding = "made by moonguip gui기กีギ";
    let content = format!("{:<70} {:>30}", shortcuts, branding);
    
    Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(theme.accent))
        )
        .style(theme.text_secondary())
}
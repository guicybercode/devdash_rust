use ratatui::style::{Color, Style};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub accent: Color,
}

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        
        themes.insert(
            "moonlight".to_string(),
            Theme {
                name: "Moonlight",
                primary: Color::Rgb(190, 180, 255),
                secondary: Color::Rgb(140, 180, 255),
                background: Color::Rgb(30, 30, 45),
                surface: Color::Rgb(40, 40, 55),
                text: Color::Rgb(255, 255, 255),
                text_secondary: Color::Rgb(170, 170, 170),
                success: Color::Rgb(120, 255, 180),
                warning: Color::Rgb(255, 200, 120),
                error: Color::Rgb(255, 120, 140),
                accent: Color::Rgb(180, 120, 255),
            },
        );

        themes.insert(
            "nord".to_string(),
            Theme {
                name: "Nord",
                primary: Color::Rgb(136, 192, 208),
                secondary: Color::Rgb(143, 188, 187),
                background: Color::Rgb(46, 52, 64),
                surface: Color::Rgb(59, 66, 82),
                text: Color::Rgb(236, 239, 244),
                text_secondary: Color::Rgb(216, 222, 233),
                success: Color::Rgb(163, 190, 140),
                warning: Color::Rgb(235, 203, 139),
                error: Color::Rgb(191, 97, 106),
                accent: Color::Rgb(136, 192, 208),
            },
        );

        themes.insert(
            "dracula".to_string(),
            Theme {
                name: "Dracula",
                primary: Color::Rgb(189, 147, 249),
                secondary: Color::Rgb(139, 233, 253),
                background: Color::Rgb(40, 42, 54),
                surface: Color::Rgb(68, 71, 90),
                text: Color::Rgb(248, 248, 242),
                text_secondary: Color::Rgb(188, 187, 177),
                success: Color::Rgb(80, 250, 123),
                warning: Color::Rgb(255, 184, 108),
                error: Color::Rgb(255, 85, 85),
                accent: Color::Rgb(189, 147, 249),
            },
        );

        themes.insert(
            "gruvbox".to_string(),
            Theme {
                name: "Gruvbox",
                primary: Color::Rgb(211, 134, 155),
                secondary: Color::Rgb(131, 165, 152),
                background: Color::Rgb(40, 40, 40),
                surface: Color::Rgb(60, 56, 54),
                text: Color::Rgb(235, 219, 178),
                text_secondary: Color::Rgb(213, 196, 161),
                success: Color::Rgb(142, 192, 124),
                warning: Color::Rgb(250, 189, 47),
                error: Color::Rgb(251, 73, 52),
                accent: Color::Rgb(211, 134, 155),
            },
        );

        themes.insert(
            "solarized".to_string(),
            Theme {
                name: "Solarized Light",
                primary: Color::Rgb(38, 139, 210),
                secondary: Color::Rgb(108, 113, 196),
                background: Color::Rgb(253, 246, 227),
                surface: Color::Rgb(238, 232, 213),
                text: Color::Rgb(88, 110, 117),
                text_secondary: Color::Rgb(101, 123, 131),
                success: Color::Rgb(133, 153, 0),
                warning: Color::Rgb(181, 137, 0),
                error: Color::Rgb(211, 54, 130),
                accent: Color::Rgb(38, 139, 210),
            },
        );

        themes.insert(
            "tokyonight".to_string(),
            Theme {
                name: "Tokyo Night",
                primary: Color::Rgb(124, 169, 247),
                secondary: Color::Rgb(108, 221, 192),
                background: Color::Rgb(26, 27, 38),
                surface: Color::Rgb(36, 40, 59),
                text: Color::Rgb(192, 202, 245),
                text_secondary: Color::Rgb(124, 135, 173),
                success: Color::Rgb(74, 218, 134),
                warning: Color::Rgb(255, 184, 108),
                error: Color::Rgb(218, 106, 106),
                accent: Color::Rgb(124, 169, 247),
            },
        );

        ThemeManager {
            themes,
            current: "moonlight".to_string(),
        }
    }

    pub fn current_theme(&self) -> &Theme {
        self.themes.get(&self.current).unwrap()
    }

    pub fn set_theme(&mut self, name: String) {
        if self.themes.contains_key(&name) {
            self.current = name;
        }
    }

    pub fn cycle_theme(&mut self) {
        let themes: Vec<String> = self.themes.keys().cloned().collect();
        if let Some(index) = themes.iter().position(|t| t == &self.current) {
            let next_index = (index + 1) % themes.len();
            self.current = themes[next_index].clone();
        }
    }

    pub fn theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
}

impl Theme {
    pub fn bg(&self) -> Style {
        Style::default().bg(self.background)
    }

    pub fn surface(&self) -> Style {
        Style::default().bg(self.surface)
    }

    pub fn text(&self) -> Style {
        Style::default().fg(self.text)
    }

    pub fn text_secondary(&self) -> Style {
        Style::default().fg(self.text_secondary)
    }

    pub fn primary(&self) -> Style {
        Style::default().fg(self.primary)
    }

    pub fn success(&self) -> Style {
        Style::default().fg(self.success)
    }

    pub fn warning(&self) -> Style {
        Style::default().fg(self.warning)
    }

    pub fn error(&self) -> Style {
        Style::default().fg(self.error)
    }
}
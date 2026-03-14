use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub fn dark_theme() -> Theme {
    Theme::custom(
        "Dota Dark".to_string(),
        iced::theme::Palette {
            background: Color::from_rgb(0.08, 0.08, 0.10),
            text: Color::from_rgb(0.92, 0.92, 0.92),
            primary: Color::from_rgb(0.83, 0.18, 0.18),
            success: Color::from_rgb(0.20, 0.78, 0.35),
            danger: Color::from_rgb(0.90, 0.20, 0.20),
        },
    )
}

// Card container styles

pub fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.14, 0.14, 0.18))),
        border: Border {
            color: Color::from_rgb(0.25, 0.25, 0.30),
            width: 1.5,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: None,
    }
}

pub fn selected_card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.18, 0.12, 0.12))),
        border: Border {
            color: Color::from_rgb(0.90, 0.30, 0.20),
            width: 2.5,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.90, 0.20, 0.10, 0.25),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 16.0,
        },
        text_color: None,
    }
}

pub fn placeholder_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.20, 0.20, 0.25))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(Color::from_rgb(0.5, 0.5, 0.55)),
    }
}

pub fn bottom_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.06, 0.06, 0.08))),
        border: Border {
            color: Color::from_rgb(0.20, 0.20, 0.25),
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        text_color: None,
    }
}

pub fn header_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.06, 0.06, 0.08))),
        border: Border {
            color: Color::from_rgb(0.20, 0.20, 0.25),
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        text_color: None,
    }
}

// Button styles

pub fn apply_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::from_rgb(0.75, 0.15, 0.15))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgb(0.85, 0.20, 0.20),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.85, 0.20, 0.20))),
            ..base
        },
        button::Status::Disabled => {
            let mut s = button::primary(theme, status);
            s.background = Some(Background::Color(Color::from_rgb(0.30, 0.30, 0.35)));
            s.text_color = Color::from_rgb(0.50, 0.50, 0.55);
            s
        }
        _ => base,
    }
}

pub fn launch_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::from_rgb(0.10, 0.45, 0.55))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgb(0.15, 0.55, 0.65),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.55, 0.65))),
            ..base
        },
        _ => base,
    }
}

pub fn card_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// Sidebar styles

pub fn sidebar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.10, 0.10, 0.13))),
        border: Border {
            color: Color::from_rgb(0.20, 0.20, 0.25),
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        text_color: None,
    }
}

pub fn tab_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: Color::from_rgb(0.55, 0.55, 0.60),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.20))),
            text_color: Color::from_rgb(0.80, 0.80, 0.85),
            ..base
        },
        _ => base,
    }
}

pub fn active_tab_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.16, 0.12, 0.12))),
        text_color: Color::from_rgb(0.95, 0.95, 0.95),
        border: Border {
            color: Color::from_rgb(0.83, 0.18, 0.18),
            width: 2.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

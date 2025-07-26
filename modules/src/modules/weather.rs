use crate::{Color, Icon, Style};
use crate::modules::ToModule;
use std::{fmt, process::Command, time::{Duration, Instant}};

pub struct Weather {
    temp: String,
    next_update: Instant,
}

impl Weather {
    pub fn new() -> Self {
        let mut w = Self {
            temp: "…".into(),
            next_update: Instant::now(),
        };
        w.update();
        w
    }

    fn fetch_weather() -> Option<String> {
        let url = "https://wttr.in/?format=%t";
        let output = Command::new("curl")
            .args(["-s", url])
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }
}

impl ToModule for Weather {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Manual('󰖔')) // Nerd Font icon (weather-sunny)
    }

    fn style(&self) -> Style {
        Style::new_with_fg(Color::Yellow)
    }

    fn next_render_time(&self) -> Option<Duration> {
        Some(self.next_update.saturating_duration_since(Instant::now()))
    }

    fn update(&mut self) {
        if Instant::now() < self.next_update {
            return;
        }
        if let Some(temp) = Self::fetch_weather() {
            self.temp = temp;
        }
        self.next_update = Instant::now() + Duration::from_secs(600); // 10 min cache
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.temp)
    }
}

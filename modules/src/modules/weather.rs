use crate::{Color, Icon, Style};
use crate::modules::ToModule;
use std::fmt;
use std::process::Command;
use std::time::{Duration, Instant};

pub struct Weather {
    temp: f32,
    next_update: Instant,
}

impl Weather {
    pub fn new() -> Self {
        let mut module = Self {
            temp: 0.0,
            next_update: Instant::now(),
        };
        module.update();
        module
    }

    fn fetch_weather() -> Option<f32> {
        let loc_output = Command::new("curl")
            .args(["-s", "https://ipinfo.io/loc"])
            .output()
            .ok()?;
        let loc = String::from_utf8_lossy(&loc_output.stdout).trim().to_string();
        let mut parts = loc.split(',');
        let lat = parts.next()?;
        let lon = parts.next()?;

        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
            lat, lon
        );

        let weather_output = Command::new("curl")
            .args(["-s", &url])
            .output()
            .ok()?;

        let json = String::from_utf8_lossy(&weather_output.stdout);
        let temp_marker = "\"temperature\":";
        let start = json.find(temp_marker)? + temp_marker.len();
        let end = json[start..].find(',').unwrap_or(json[start..].len()) + start;
        let temp_str = &json[start..end];
        temp_str.trim().parse().ok()
    }
}

impl ToModule for Weather {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Manual('')) // thermometer icon
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
        self.next_update = Instant::now() + Duration::from_secs(600);
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:+.0}°C", self.temp)
    }
}

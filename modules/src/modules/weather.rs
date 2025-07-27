use crate::{Color, Icon, Style};
use crate::modules::ToModule;
use serde_json::Value;
use std::fmt;
use std::process::Command;
use std::time::{Duration, Instant};

pub struct Weather {
    temp: f32,
    next_update: Instant,
    code: u8,
    is_day: bool,
}

impl Weather {
    pub fn new() -> Self {
        let mut module = Self {
            temp: 0.0,
            next_update: Instant::now(),
            code: 0,
            is_day: true,
        };
        module.update();
        module
    }

    fn fetch_weather() -> Option<(f32, u8, bool)> {
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
        let parsed: Value = serde_json::from_str(&json).ok()?;
        let temp = parsed["current_weather"]["temperature"].as_f64()? as f32;
        let code = parsed["current_weather"]["weathercode"].as_u64()? as u8;
        let is_day = parsed["current_weather"]["is_day"].as_u64()? == 1;
        Some((temp, code, is_day))
    }
}

impl ToModule for Weather {
    fn icon(&self) -> Option<Icon> {
        use Icon::Manual;

        // Placeholder: you should have self.code: u8 and self.is_day: bool fields in Weather
        // This assumes these fields exist and are populated.
        let code = self.code;
        let day = self.is_day;

        let ch = match (code, day) {
            (0, true) => '',   // Day sunny (nf-weather-day_sunny)
            (0, false) => '󰖔',  // Night clear (nf-md-weather_night)
            (1..=3, true) => '',   // Day cloudy (nf-weather-day_cloudy)
            (1..=3, false) => '',  // Night cloudy (nf-weather-night_alt_cloudy)
            (45..=48, true) => '', // Day fog (nf-weather-day_fog)
            (45..=48, false) => '', // Night fog (nf-weather-night_fog)
            (51..=67, true) => '',  // Day rain (nf-weather-day_rain)
            (51..=67, false) => '', // Night rain (nf-weather-night_alt_rain)
            (71..=77, true) => '',  // Day snow (nf-weather-day_snow)
            (71..=77, false) => '', // Night snow (nf-weather-night_alt_snow)
            (95..=99, true) => '',  // Day thunderstorm (nf-weather-day_thunderstorm)
            (95..=99, false) => '', // Night thunderstorm (nf-weather-night_alt_thunderstorm)
            _ => '', // Default: thermometer (updatnf-weather-thermometer_exteriored)
        };

        Some(Manual(ch))
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
        if let Some((temp, code, is_day)) = Self::fetch_weather() {
            self.temp = temp;
            self.code = code;
            self.is_day = is_day;
        }
        self.next_update = Instant::now() + Duration::from_secs(600);
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:+.0}°C", self.temp)
    }
}

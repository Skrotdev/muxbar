use crate::{Color, Icon, Style};
use crate::modules::ToModule;
use std::{fmt, process::Command, time::{Duration, Instant}};

pub struct Kube {
    context: String,
    namespace: String,
    next_update: Instant,
}

impl Kube {
    pub fn new() -> Self {
        let mut module = Self {
            context: "…".to_string(),
            namespace: "…".to_string(),
            next_update: Instant::now(),
        };
        module.update();
        module
    }

    fn fetch_context() -> Option<String> {
        Command::new("kubectl")
            .args(["config", "current-context"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    }

    fn fetch_namespace() -> Option<String> {
        Command::new("kubectl")
            .args(["config", "view", "--minify", "--output", "jsonpath={..namespace}"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    }
}

impl ToModule for Kube {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Manual('⎈')) // Kubernetes icon from Nerd Fonts
    }

    fn style(&self) -> Style {
        Style::new_with_fg(Color::Cyan)
    }

    fn next_render_time(&self) -> Option<Duration> {
        Some(self.next_update.saturating_duration_since(Instant::now()))
    }

    fn update(&mut self) {
        if Instant::now() < self.next_update {
            return;
        }
        if let Some(ctx) = Self::fetch_context() {
            self.context = ctx;
        }
        if let Some(ns) = Self::fetch_namespace() {
            self.namespace = ns;
        }
        self.next_update = Instant::now() + Duration::from_secs(10); // update every 10s
    }
}

impl fmt::Display for Kube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.context, self.namespace)
    }
}

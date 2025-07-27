use modules::modules::{Battery, Cpu, DateTime, Memory, Text, TmuxContent, Warning, Weather, Kube};
use modules::{modules::ToModule, Color, Icon, Module, Style};
use std::time::Duration;

pub fn get_modules() -> Vec<Module<Box<dyn ToModule>>> {
    let text = "Muxbar";
    [
        Text::new(text, 4, '.').map(Into::into),
        Some(Kube::new().into()),
        Some(
            Module::from(DateTime::new("%d-%m-%Y"))
                .set_icon(Some(Icon::Calendar))
                .set_style(Style::new_with_fg(Color::Yellow)),
        ),
        Some(DateTime::new("%H:%M:%S").into()),
        Some(Cpu::new(2, 2).into()),
        Some(Memory::new(2, 2).into()),
        Battery::try_new(2).ok().flatten().map(Into::into),
        Some(TmuxContent::SessionName.into()),
        Some(Weather::new().into()),
        Warning::new_battery(20, 20.0)
            .ok()
            .flatten()
            .map(Into::into),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub const PRE_MODULES: &str = "";
pub const BETWEEN_MODULES: &str = " ";
pub const POST_MODULES: &str = " ";

// Tmux can't handle too many rerenders, so we limit it to 1 second
pub const FASTEST_INTERVALL: Duration = Duration::from_secs(1);

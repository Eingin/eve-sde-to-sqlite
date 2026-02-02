//! Terminal UI module using indicatif
//!
//! Provides animated spinners and progress bars for CLI output.

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

/// Application phases shown during operation
#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    Checking,
    Downloading,
    Extracting,
    Converting,
    Complete,
}

impl Phase {
    fn icon(&self) -> &'static str {
        match self {
            Phase::Checking => "◐",
            Phase::Downloading => "↓",
            Phase::Extracting => "⤷",
            Phase::Converting => "⚙",
            Phase::Complete => "✓",
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Checking => write!(f, "Checking for updates"),
            Phase::Downloading => write!(f, "Downloading SDE"),
            Phase::Extracting => write!(f, "Extracting files"),
            Phase::Converting => write!(f, "Converting to SQLite"),
            Phase::Complete => write!(f, "Complete"),
        }
    }
}

/// Trait for UI implementations - allows both real UI and silent/test modes
pub trait Ui {
    fn set_phase(&mut self, phase: Phase);
    fn set_info(&mut self, info: impl Into<String>);
    fn set_progress(&mut self, current: u64, total: u64, label: impl Into<String>);
    fn clear_progress(&mut self);
}

fn spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.cyan} {msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
}

fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{spinner:.cyan} {msg}\n[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({percent}%)",
    )
    .unwrap()
    .progress_chars("█▓░")
}

/// Main UI application using indicatif progress bars
pub struct UiApp {
    bar: ProgressBar,
    phase: Phase,
}

impl UiApp {
    /// Create a new UI with an animated spinner
    pub fn new() -> Result<Self> {
        let bar = ProgressBar::new_spinner();
        bar.set_style(spinner_style());
        bar.enable_steady_tick(std::time::Duration::from_millis(80));
        Ok(Self {
            bar,
            phase: Phase::Checking,
        })
    }

    /// Finish the UI with a summary message
    pub fn finish(self, summary: &str) -> Result<()> {
        self.bar.finish_with_message(format!("✓ {}", summary));
        Ok(())
    }
}

impl Ui for UiApp {
    fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
        self.bar
            .set_message(format!("{} {}", self.phase.icon(), self.phase));
    }

    fn set_info(&mut self, info: impl Into<String>) {
        self.bar.set_message(format!(
            "{} {} - {}",
            self.phase.icon(),
            self.phase,
            info.into()
        ));
    }

    fn set_progress(&mut self, current: u64, total: u64, label: impl Into<String>) {
        if self.bar.length() != Some(total) {
            self.bar.set_length(total);
            self.bar.set_style(progress_style());
        }
        self.bar.set_position(current);
        self.bar.set_message(format!(
            "{} {} {}",
            self.phase.icon(),
            self.phase,
            label.into()
        ));
    }

    fn clear_progress(&mut self) {
        self.bar.set_length(0);
        self.bar.set_style(spinner_style());
    }
}

/// Silent UI implementation for testing and non-interactive use
#[derive(Default)]
pub struct SilentUi;

impl SilentUi {
    pub fn new() -> Self {
        Self
    }
}

impl Ui for SilentUi {
    fn set_phase(&mut self, _phase: Phase) {}
    fn set_info(&mut self, _info: impl Into<String>) {}
    fn set_progress(&mut self, _current: u64, _total: u64, _label: impl Into<String>) {}
    fn clear_progress(&mut self) {}
}

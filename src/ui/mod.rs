//! Terminal UI module using ratatui
//!
//! Provides a simple API for displaying application state:
//! - Current phase (Checking, Downloading, Extracting, Converting)
//! - Progress (current/total with optional details)
//! - Activity log (scrollable history)

mod components;

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::time::Duration;

use components::{LogPanel, ProgressPanel, StatusPanel};

/// Application phases shown in the status panel
#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    Checking,
    Downloading,
    Extracting,
    Converting,
    Complete,
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

/// Progress information for the current operation
#[derive(Debug, Clone, Default)]
pub struct Progress {
    pub current: u64,
    pub total: u64,
    pub label: String,
}

impl Progress {
    pub fn new(current: u64, total: u64, label: impl Into<String>) -> Self {
        Self {
            current,
            total,
            label: label.into(),
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.current as f64 / self.total as f64
        }
    }
}

/// Trait for UI implementations - allows both real TUI and silent/test modes
pub trait Ui {
    fn set_phase(&mut self, phase: Phase);
    fn set_info(&mut self, info: impl Into<String>);
    fn set_progress(&mut self, current: u64, total: u64, label: impl Into<String>);
    fn clear_progress(&mut self);
    fn log(&mut self, message: impl Into<String>);
}

/// Main UI application state - full TUI implementation
pub struct UiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    status: StatusPanel,
    progress: ProgressPanel,
    log: LogPanel,
    should_quit: bool,
}

impl UiApp {
    /// Create a new UI application and enter the alternate screen
    pub fn new() -> Result<Self> {
        // Setup terminal
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            status: StatusPanel::new(),
            progress: ProgressPanel::new(),
            log: LogPanel::new(),
            should_quit: false,
        })
    }

    /// Check for quit signal (Ctrl+C or 'q')
    pub fn check_quit(&mut self) -> bool {
        if event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(CrosstermEvent::Key(KeyEvent { code, .. })) = event::read() {
                if code == KeyCode::Char('q') || code == KeyCode::Char('c') {
                    self.should_quit = true;
                }
            }
        }
        self.should_quit
    }

    /// Draw the UI
    fn draw(&mut self) -> Result<()> {
        let status = &self.status;
        let progress = &self.progress;
        let log = &self.log;

        self.terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5), // Status panel
                    Constraint::Length(3), // Progress bar
                    Constraint::Min(5),    // Log panel
                ])
                .split(area);

            status.render(frame, chunks[0]);
            progress.render(frame, chunks[1]);
            log.render(frame, chunks[2]);
        })?;

        Ok(())
    }

    /// Finish the UI and restore the terminal
    pub fn finish(mut self, summary: &str) -> Result<()> {
        self.set_phase(Phase::Complete);
        self.clear_progress();
        self.log(summary);
        self.draw()?;

        // Wait for keypress before exiting
        self.log("Press any key to exit...");
        self.draw()?;

        loop {
            if event::poll(Duration::from_millis(100))? {
                if let CrosstermEvent::Key(_) = event::read()? {
                    break;
                }
            }
        }

        self.restore()
    }

    /// Restore terminal without waiting
    pub fn restore(mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        self.terminal.backend_mut().execute(LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Ui for UiApp {
    fn set_phase(&mut self, phase: Phase) {
        self.status.set_phase(phase);
        self.draw().ok();
    }

    fn set_info(&mut self, info: impl Into<String>) {
        self.status.set_info(info);
        self.draw().ok();
    }

    fn set_progress(&mut self, current: u64, total: u64, label: impl Into<String>) {
        self.progress
            .set_progress(Progress::new(current, total, label));
        self.draw().ok();
    }

    fn clear_progress(&mut self) {
        self.progress.clear();
        self.draw().ok();
    }

    fn log(&mut self, message: impl Into<String>) {
        self.log.add(message);
        self.draw().ok();
    }
}

impl Drop for UiApp {
    fn drop(&mut self) {
        // Best effort cleanup
        terminal::disable_raw_mode().ok();
        self.terminal
            .backend_mut()
            .execute(LeaveAlternateScreen)
            .ok();
        self.terminal.show_cursor().ok();
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
    fn log(&mut self, _message: impl Into<String>) {}
}

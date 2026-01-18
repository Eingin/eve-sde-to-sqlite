//! UI Components for the terminal interface

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph};
use ratatui::Frame;

use super::{Phase, Progress};

/// Status panel showing current phase and info
pub struct StatusPanel {
    phase: Phase,
    info: String,
}

impl StatusPanel {
    pub fn new() -> Self {
        Self {
            phase: Phase::Checking,
            info: String::new(),
        }
    }

    pub fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    pub fn set_info(&mut self, info: impl Into<String>) {
        self.info = info.into();
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let phase_style = match self.phase {
            Phase::Complete => Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            _ => Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        };

        let phase_indicator = match self.phase {
            Phase::Checking => "◐",
            Phase::Downloading => "↓",
            Phase::Extracting => "⤷",
            Phase::Converting => "⚙",
            Phase::Complete => "✓",
        };

        let lines = vec![
            Line::from(vec![
                Span::styled(format!(" {} ", phase_indicator), phase_style),
                Span::styled(self.phase.to_string(), phase_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("   "),
                Span::styled(&self.info, Style::default().fg(Color::Gray)),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" EVE SDE to SQLite ")
            .border_style(Style::default().fg(Color::Blue));

        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }
}

/// Progress panel showing a progress bar
pub struct ProgressPanel {
    progress: Option<Progress>,
}

impl ProgressPanel {
    pub fn new() -> Self {
        Self { progress: None }
    }

    pub fn set_progress(&mut self, progress: Progress) {
        self.progress = Some(progress);
    }

    pub fn clear(&mut self) {
        self.progress = None;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(Color::Blue));

        match &self.progress {
            Some(progress) => {
                let label = if progress.total > 0 {
                    format!(
                        "{}: {}/{} ({:.0}%)",
                        progress.label,
                        progress.current,
                        progress.total,
                        progress.ratio() * 100.0
                    )
                } else {
                    progress.label.clone()
                };

                let gauge = Gauge::default()
                    .block(block)
                    .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
                    .ratio(progress.ratio().min(1.0))
                    .label(label);

                frame.render_widget(gauge, area);
            }
            None => {
                let paragraph = Paragraph::new("").block(block);
                frame.render_widget(paragraph, area);
            }
        }
    }
}

/// Log panel showing scrollable history
pub struct LogPanel {
    entries: Vec<String>,
    max_entries: usize,
}

impl LogPanel {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }

    pub fn add(&mut self, message: impl Into<String>) {
        self.entries.push(message.into());
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Activity ")
            .border_style(Style::default().fg(Color::Blue));

        // Calculate how many items we can show
        let visible_height = area.height.saturating_sub(2) as usize; // -2 for borders
        let start = self.entries.len().saturating_sub(visible_height);

        let items: Vec<ListItem> = self.entries[start..]
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let style = if i == self.entries.len() - start - 1 {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                ListItem::new(Span::styled(format!(" {}", entry), style))
            })
            .collect();

        let list = List::new(items).block(block);
        frame.render_widget(list, area);
    }
}

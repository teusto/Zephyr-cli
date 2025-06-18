use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    layout::{Alignment, Rect},
    text::{Line, Span},
};
use crossterm::event::KeyCode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mood {
    Angry,
    Sad,
    Neutral,
    Happy,
}

impl Mood {
    fn all() -> [Mood; 4] {
        [Mood::Angry, Mood::Sad, Mood::Neutral, Mood::Happy]
    }

    fn name(self) -> &'static str {
        match self {
            Mood::Angry => "Angry",
            Mood::Sad => "Sad",
            Mood::Neutral => "Neutral",
            Mood::Happy => "Happy",
        }
    }

    fn face(self) -> &'static str {
        match self {
            Mood::Angry => "😠",
            Mood::Sad => "😢",
            Mood::Neutral => "😐",
            Mood::Happy => "😊",
        }
    }

    fn bg(self) -> Color {
        match self {
            Mood::Angry => Color::Red,
            Mood::Sad => Color::Blue,
            Mood::Neutral => Color::Yellow,
            Mood::Happy => Color::Green,
        }
    }
}

pub struct MoodJournal {
    cursor: usize,
    chosen: Option<Mood>,
}

impl MoodJournal {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            chosen: None
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.chosen = None;
    }

    pub fn handle_key(&mut self, code: KeyCode) -> bool {
        use KeyCode::*;
        match code {
            Left => {
                if self.chosen.is_none() {
                    if self.cursor == 0 { self.cursor = 3 } else { self.cursor -= 1; }
                    return true;
                }
            }
            Right => {
                if self.chosen.is_none() {
                    self.cursor = (self.cursor + 1) % 4;
                    return true;
                }
            }
            Char('1') => { self.chosen = Some(Mood::Angry); return true; }
            Char('2') => { self.chosen = Some(Mood::Sad); return true; }
            Char('3') => { self.chosen = Some(Mood::Neutral); return true; }
            Char('4') => { self.chosen = Some(Mood::Happy); return true; }
            Enter => {
                if self.chosen.is_none() {
                    self.chosen = Some(Mood::all()[self.cursor]);
                }
                return true;
            }
            Esc => { self.reset(); return true; }
            _ => {}
        }
        false
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        if let Some(mood) = self.chosen {
            // show chosen mood with colored background
            let block = Block::default().borders(Borders::NONE).style(Style::default().bg(mood.bg()));
            frame.render_widget(block, area);
            let text = vec![Line::from(format!("You feel {} {}", mood.name(), mood.face()).bold())];
            frame.render_widget(
                Paragraph::new(text).alignment(Alignment::Center),
                area,
            );
            return;
        }

        // selection UI
        let block = Block::default().borders(Borders::NONE);
        frame.render_widget(block, area);

        let mut lines = Vec::new();
        lines.push(Line::from("How are you feeling?"));

        // faces line
        let faces: Vec<Span> = Mood::all().iter().enumerate().map(|(i, mood)| {
            let mut span = Span::raw(format!("  {}  ", mood.face()));
            if self.cursor == i {
                span = span.bold().fg(Color::White);
            }
            span
        }).collect();
        lines.push(Line::from(faces));

        lines.push(Line::from("[←][→] select   [Enter] confirm   [Esc] reset"));

        frame.render_widget(
            Paragraph::new(lines).alignment(Alignment::Center),
            area,
        );
    }
}
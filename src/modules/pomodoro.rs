use std::time::{Duration, Instant};
use ratatui::{
    Frame,
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color, Stylize},
    layout::{Alignment, Rect},
    text::Line,
};

#[derive(Clone, Copy)]
pub struct PomodoroConfig {
    pub work: Duration,
    pub break_: Duration,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            work: Duration::from_secs(25 * 60),
            break_: Duration::from_secs(5 * 60),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Work,
    Break,
}

pub struct Pomodoro {
    config: PomodoroConfig,
    phase: Phase,
    remaining: Duration,
    running: bool,
    last_tick: Instant,
}

impl Pomodoro {
    pub fn new(config: PomodoroConfig) -> Self {
        Self {
           remaining: config.work,
           phase: Phase::Work,
           running: false,
           last_tick: Instant::now(),
           config, 
        }
    }

    pub fn start(&mut self){
        self.running = true;
        self.last_tick = Instant::now();
    }

    pub fn reset(&mut self){
        self.phase = Phase::Work;
        self.remaining = self.config.work;
        self.running = false;
    }

    pub fn tick(&mut self){
        if !self.running { return; }
        let now = Instant::now();
        let delta = now.saturating_duration_since(self.last_tick);
        self.last_tick = now;
        if delta >= self.remaining {
            self.switch_phase();
            print!("\x07");
        } else {
            self.remaining -= delta;
        }
    }

    fn switch_phase(&mut self){
        self.phase = match self.phase {
            Phase::Work => {
                self.remaining = self.config.break_;
                Phase::Break
            },
            Phase::Break => {
                self.remaining = self.config.work;
                Phase::Work
            },
        };
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect){
        let mins = self.remaining.as_secs() / 60;
        let secs = self.remaining.as_secs() % 60;
        let timer_text = format!("{:02}:{:02}", mins, secs);
        let phase_text = match self.phase { Phase::Work => "WORK", Phase::Break => "BREAK" };
    
        let block = Block::default().borders(Borders::NONE);
        let text_lines = vec![
            Line::from(timer_text.bold()),
            Line::from(phase_text),
            Line::from("[s]tart [r]eset"),
        ];

        frame.render_widget(
            Paragraph::new(text_lines)
                .block(block)
                .alignment(Alignment::Center)
                .style(match self.phase {
                    Phase::Work => Style::default().fg(Color::LightRed),
                    Phase::Break => Style::default().fg(Color::LightGreen),
                }),
            area
        );
    }
}
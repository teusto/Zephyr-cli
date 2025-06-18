mod store;
mod modules;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Widget, Block, Borders, BorderType, Paragraph, Padding},
    text::Line,
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Style, Stylize}
};
use store::{InstalledModules};
use modules::pomodoro::{Pomodoro, PomodoroConfig};
use modules::mood_journal::MoodJournal;
use std::time::Duration;

fn main() -> Result<()>{
    ratatui::restore();
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

// ────────────────────────────── Card widget ──────────────────────────────
struct Card<'a> {
    title: &'a str,
    ascii: &'a [&'a str],
    focused: bool,
    running: bool,
}

impl<'a> Card<'a> {
    fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        // Outer block styling
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1));

        if self.focused {
            block = block.border_style(Style::new().fg(Color::Rgb(38, 97, 25)));
        } else {
            block = block.border_style(Style::new().fg(Color::Rgb(17, 50, 11)));
        }

        if self.running {
            block = block.style(Style::default().bg(Color::Rgb(117, 255, 84)));
        }

        frame.render_widget(block.clone(), area);
        let inner = block.inner(area);

        // Split inner vertically -> image center, title bottom
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .split(inner);

        let centered = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Percentage(60), Constraint::Fill(1)])
            .split(outer[0]);

        let img_lines: Vec<Line> = self.ascii.iter().map(|&s| Line::from(s)).collect();
        frame.render_widget(Paragraph::new(img_lines).alignment(Alignment::Center), centered[1]);
        frame.render_widget(Paragraph::new(self.title.bold()).alignment(Alignment::Center).style(Style::new().fg(Color::Rgb(117, 255, 84))), outer[1]);
    }
}

// ────────────────────────────── Application state ────────────────────────
struct App {
    modules: Vec<InstalledModules>,
    cursor: usize,
    running: Option<usize>,
    pomodoro: Pomodoro,
    mood: MoodJournal,
}

impl App {
    fn new() -> Self {
        Self {
            modules: vec![
                InstalledModules::Pomodoro,
                InstalledModules::TodoList,
                InstalledModules::HabitTracker,
                InstalledModules::TimeLogger,
                InstalledModules::MoodJournal,
            ],
            cursor: 0,
            running: None,
            pomodoro: Pomodoro::new(PomodoroConfig::default()),
            mood: MoodJournal::new(),
        }
    }

    fn pomodoro_idx(&self) -> usize {
        self.modules.iter().position(|m| matches! (m, InstalledModules::Pomodoro)).unwrap_or(0)
    }

    fn mood_idx(&self) -> usize {
        self.modules.iter().position(|m| matches!(m, InstalledModules::MoodJournal)).unwrap_or(0)
    }

    fn tick(&mut self) {
        if self.running == Some(self.pomodoro_idx()) {
            self.pomodoro.tick();
        }
    }

    fn on_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char(',') => {
                if self.cursor == 0 { self.cursor = self.modules.len() - 1; } else { self.cursor -= 1; }
            }
            KeyCode::Char('.') => {
                self.cursor = (self.cursor + 1) % self.modules.len();
            }
            KeyCode::Char('p') => {
                self.running = Some(self.cursor);
            }
            KeyCode::Char('s') => {
                if self.running == Some(self.pomodoro_idx()) {
                    self.pomodoro.start();
                }
            }
            KeyCode::Char('r') => {
                if self.running == Some(self.pomodoro_idx()) {
                    self.pomodoro.reset();
                }
            }
            KeyCode::Char('q') => return true, // quit
            _ => {}
        }
        // delegate to mood journal if running
        if self.running == Some(self.mood_idx()) {
            if self.mood.handle_key(code) { return false; }
        }
        false
    }
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| render(f, &app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if app.on_key(key.code) {
                    break Ok(());
                }
            }   
        }

        app.tick();
    }
}

fn render(frame: &mut Frame, app: &App) {
    let installed = &app.modules;

    // build Fill | card × n | Fill
    let per = 100 / installed.len() as u16;
    let mut constraints = Vec::with_capacity(installed.len() + 2);
    constraints.push(Constraint::Fill(1));
    constraints.extend(std::iter::repeat(Constraint::Percentage(per)).take(installed.len()));
    constraints.push(Constraint::Fill(1));

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .vertical_margin(2)
        .split(frame.area());

    for (idx, module) in installed.iter().enumerate() {
        let card = Card {
            title: module.title(),
            ascii: module.ascii(),
            focused: idx == app.cursor,
            running: app.running == Some(idx),
        };
        card.render(frame, cols[idx + 1]);

        if app.running == Some(idx) {
            match module {
                InstalledModules::Pomodoro => app.pomodoro.render(frame, cols[idx + 1]),
                InstalledModules::MoodJournal => app.mood.render(frame, cols[idx + 1]),
                _ => {},
            }
        }
    }
}

fn install_module_on_zephyr(module: InstalledModules){
}
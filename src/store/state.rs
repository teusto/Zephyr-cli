pub struct State {
    pub active_module: InstalledModules,
}

pub enum InstalledModules {
    Pomodoro,
    TodoList,
    HabitTracker,
    TimeLogger,
    MoodJournal,
}

impl InstalledModules {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Pomodoro => "Pomodoro",
            Self::TodoList => "Todo List",
            Self::HabitTracker => "Habit Tracker",
            Self::TimeLogger => "Time Logger",
            Self::MoodJournal => "Mood Journal",
        }
    }

    pub fn ascii(&self) -> &'static [&'static str] {
        match self {
            Self::Pomodoro => &["  ____  ", " | __ ) "],
            Self::TodoList => &[" [ ]", " [ ]"],
            Self::HabitTracker => &["  /‾‾\\  ", " (•_•)"],
            Self::TimeLogger => &["  ⏱ ", "  ⌛ "],
            Self::MoodJournal => &[" (ᵔᴥᵔ) "],
        }
    }
}
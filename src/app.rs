#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CurrentScreen {
    Title(TitleSelection),
    Setting,
    Rules,
    GameSetting,
    Game,
    ReturnToTitle,
    FivePawn,
    HandRank,
    RoundOver,
    GameOver,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TitleSelection {
    Start,
    Settings,
    Rules,
    Exit,
}

impl TitleSelection {
    pub fn next(&self) -> Self {
        match self {
            TitleSelection::Start => TitleSelection::Settings,
            TitleSelection::Settings => TitleSelection::Rules,
            TitleSelection::Rules => TitleSelection::Exit,
            TitleSelection::Exit => TitleSelection::Start,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            TitleSelection::Start => TitleSelection::Exit,
            TitleSelection::Settings => TitleSelection::Start,
            TitleSelection::Rules => TitleSelection::Settings,
            TitleSelection::Exit => TitleSelection::Rules,
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Title(TitleSelection::Start),
        }
    }
}

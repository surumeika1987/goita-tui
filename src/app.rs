use goita::BoardDirection;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CurrentScreen {
    Title(TitleSelection),
    Setting,
    Rules,
    GameSettings(GameSettingSelection),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameSettingSelection {
    Player1,
    Player2,
    Player3,
    Player4,
    InitialTurnPlayer,
    WinningScore,
    Start,
}

impl GameSettingSelection {
    pub fn next(&self) -> Self {
        match self {
            GameSettingSelection::Player1 => GameSettingSelection::Player2,
            GameSettingSelection::Player2 => GameSettingSelection::Player3,
            GameSettingSelection::Player3 => GameSettingSelection::Player4,
            GameSettingSelection::Player4 => GameSettingSelection::InitialTurnPlayer,
            GameSettingSelection::InitialTurnPlayer => GameSettingSelection::WinningScore,
            GameSettingSelection::WinningScore => GameSettingSelection::Start,
            GameSettingSelection::Start => GameSettingSelection::Player1,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            GameSettingSelection::Player1 => GameSettingSelection::Start,
            GameSettingSelection::Player2 => GameSettingSelection::Player1,
            GameSettingSelection::Player3 => GameSettingSelection::Player2,
            GameSettingSelection::Player4 => GameSettingSelection::Player3,
            GameSettingSelection::InitialTurnPlayer => GameSettingSelection::Player4,
            GameSettingSelection::WinningScore => GameSettingSelection::InitialTurnPlayer,
            GameSettingSelection::Start => GameSettingSelection::WinningScore,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerSetting {
    Player,
    CPU,
}

impl PlayerSetting {
    pub fn next(&self) -> Self {
        match self {
            PlayerSetting::Player => PlayerSetting::CPU,
            PlayerSetting::CPU => PlayerSetting::Player,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            PlayerSetting::Player => PlayerSetting::CPU,
            PlayerSetting::CPU => PlayerSetting::Player,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GameSetting {
    pub player1: PlayerSetting,
    pub player2: PlayerSetting,
    pub player3: PlayerSetting,
    pub player4: PlayerSetting,
    pub initial_turn_player: BoardDirection,
    pub winning_score: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct App {
    pub current_screen: CurrentScreen,

    pub game_setting: GameSetting,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Title(TitleSelection::Start),
            game_setting: GameSetting {
                player1: PlayerSetting::Player,
                player2: PlayerSetting::CPU,
                player3: PlayerSetting::CPU,
                player4: PlayerSetting::CPU,
                initial_turn_player: BoardDirection::North,
                winning_score: 150,
            },
        }
    }
}

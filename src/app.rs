use goita::{
    ApplyResult, BoardDirection, DealEvent, GoitaGame, GoitaRule, Piece, PieceWithFacing,
    PlayerAction, Team,
};
use std::iter;

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

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,

    pub view_board: [Option<Vec<PieceWithFacing>>; 4],
    pub view_hand: Option<Vec<Piece>>,
    pub temp_place_piece: Option<Piece>,
    pub piece_selection: u8,

    pub game_setting: GameSetting,
    pub game_state: Option<GoitaGame>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Title(TitleSelection::Start),
            view_board: [None, None, None, None],
            view_hand: None,
            temp_place_piece: None,
            piece_selection: 0,
            game_setting: GameSetting {
                player1: PlayerSetting::Player,
                player2: PlayerSetting::CPU,
                player3: PlayerSetting::CPU,
                player4: PlayerSetting::CPU,
                initial_turn_player: BoardDirection::North,
                winning_score: 150,
            },
            game_state: None,
        }
    }

    pub fn start_new_game(&mut self) {
        self.game_state = Some(GoitaGame::new(
            GoitaRule::new(self.game_setting.winning_score as u32),
            self.game_setting.initial_turn_player,
        ));
    }

    pub fn start_new_round(&mut self) -> Option<DealEvent> {
        if let Some(game) = self.game_state.as_mut() {
            let deal_event = game.start_new_round();
            if let Ok(deal_event) = deal_event {
                Some(deal_event)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn team_score(&self, team: Team) -> Option<u32> {
        if let Some(game) = self.game_state.as_ref() {
            Some(game.score(team))
        } else {
            None
        }
    }

    pub fn winning_score(&self) -> Option<u32> {
        if let Some(game) = self.game_state.as_ref() {
            Some(game.winning_score())
        } else {
            None
        }
    }

    pub fn current_turn_player(&self) -> Option<BoardDirection> {
        if let Some(game) = self.game_state.as_ref() {
            game.current_turn_player()
        } else {
            None
        }
    }

    pub fn view_player_board(&self, player: BoardDirection) -> Option<Vec<PieceWithFacing>> {
        self.view_board
            .get(player as usize)
            .and_then(|board| board.clone())
    }

    pub fn game_player_board(&self, player: BoardDirection) -> Option<Vec<PieceWithFacing>> {
        if let Some(game) = self.game_state.as_ref() {
            game.player_board(player)
        } else {
            None
        }
    }

    pub fn set_fivepawn_player_board(&mut self) {
        if let Some(game) = self.game_state.as_ref() {
            for i in 0..4 {
                let player = BoardDirection::from(i as u8);
                if let Some(hand) = game.player_hand(player) {
                    let five_pawn = hand.iter().filter(|piece| **piece == Piece::Pawn).count() == 5;
                    if five_pawn {
                        self.view_board[i] = Some(
                            iter::repeat_n(PieceWithFacing::FaceUp(Piece::Pawn), 5)
                                .collect::<Vec<PieceWithFacing>>(),
                        );
                    }
                }
            }
        }
    }

    pub fn set_hand_rank_player_board(&mut self, player: BoardDirection) {
        if let Some(game) = self.game_state.as_ref() {
            if let Some(hand) = game.player_hand(player) {
                let pawn_count = hand.iter().filter(|piece| **piece == Piece::Pawn).count();
                match pawn_count {
                    6 => {
                        let mut remain_pieces = hand
                            .iter()
                            .filter(|piece| **piece != Piece::Pawn)
                            .cloned()
                            .collect::<Vec<Piece>>();
                        remain_pieces.sort();
                        let mut temp_board =
                            iter::repeat_n(PieceWithFacing::FaceUp(Piece::Pawn), 6)
                                .collect::<Vec<PieceWithFacing>>();
                        temp_board.append(
                            &mut remain_pieces
                                .into_iter()
                                .map(PieceWithFacing::FaceUp)
                                .collect(),
                        );
                        self.view_board[usize::from(player)] = Some(temp_board);
                    }
                    7 => {
                        let remain_piece = hand
                            .iter()
                            .filter(|piece| **piece != Piece::Pawn)
                            .cloned()
                            .collect::<Vec<Piece>>();
                        let mut temp_board =
                            iter::repeat_n(PieceWithFacing::FaceUp(Piece::Pawn), 6)
                                .collect::<Vec<PieceWithFacing>>();
                        temp_board.push(PieceWithFacing::FaceUp(*remain_piece.first().unwrap()));
                        self.view_board[usize::from(player)] = Some(temp_board);
                    }
                    8 => {
                        self.view_board[usize::from(player)] =
                            Some(iter::repeat_n(PieceWithFacing::FaceUp(Piece::Pawn), 8).collect())
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn clear_view_board(&mut self) {
        self.view_board = [None, None, None, None];
    }

    pub fn sync_board(&mut self) {
        if let Some(game) = self.game_state.as_ref() {
            for i in 0..4 {
                let player = BoardDirection::from(i as u8);
                self.view_board[i] = game.player_board(player);
            }
        } else {
            self.view_board = [None, None, None, None];
        }
    }

    pub fn clear_view_hand(&mut self) {
        self.view_hand = None;
    }

    pub fn sync_hand(&mut self, player: BoardDirection) {
        if let Some(game) = self.game_state.as_ref() {
            self.view_hand = game.player_hand(player);
        } else {
            self.view_hand = None;
        }
    }

    pub fn remove_from_view_hand(&mut self, piece: Piece) {
        if let Some(view_hand) = self.view_hand.as_mut() {
            if let Some(pos) = view_hand.iter().position(|p| *p == piece) {
                view_hand.remove(pos);
            }
        }
    }

    pub fn place_piece(&mut self) -> Option<ApplyResult> {
        if let Some(game) = self.game_state.as_mut()
            && let Some(current_turn_player) = game.current_turn_player()
            && let Some(view_hand) = self.view_hand.as_ref()
            && let Some(selection_piece) = view_hand.get(self.piece_selection as usize)
        {
            if let Some(temp_place_piece) = self.temp_place_piece {
                let result = game.play_turn(
                    current_turn_player,
                    PlayerAction::Place {
                        top: temp_place_piece,
                        bottom: *selection_piece,
                    },
                );
                if let Ok(result) = result {
                    self.temp_place_piece = None;
                    self.sync_board();
                    self.sync_hand(current_turn_player.next());
                    Some(result)
                } else {
                    None
                }
            } else {
                self.temp_place_piece = Some(*selection_piece);
                self.remove_from_view_hand(*selection_piece);
                None
            }
        } else {
            None
        }
    }
}

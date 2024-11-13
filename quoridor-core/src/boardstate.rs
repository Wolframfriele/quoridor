use bitmaps::Bitmap;

use crate::actions::{Action, PossibleActions};
use crate::gamestate::{GameStatus, VictoryReason};
use crate::locations::{Direction, PawnLocation, WallLocation, WallOrientation};
use crate::visualize::print_board_state;

#[derive(Clone, Hash, Debug)]
/// The boardstate is responsible for keeping track of all the pawns and walls placed on the board.
///
/// It is also the place that holds the basic game rules related logic, since it is the place that
/// can check if move's keep the board in a legal state. This will require some implementation off
/// a path finding algorithm, since so much off the game revolves around not blocking the path off
/// for the pawns.
///
/// Still need to figure out the rules around jumping over pawns and how to check that efficiently
pub struct Boardstate {
    active_player: Player,
    white_position: PawnLocation,
    black_position: PawnLocation,
    white_available_walls: u8,
    black_available_walls: u8,
    wall_positions: [Option<WallOrientation>; 71],
    horizontal_blocks: Bitmap::<72>,
    vertical_blocks: Bitmap::<80>,
}

impl Default for Boardstate {
    fn default() -> Boardstate {
        Boardstate {
            active_player: Player::White,
            white_position: PawnLocation::build(4)
                .expect("White player starting location on square 5 should be a valid location."),
            black_position: PawnLocation::build(76)
                .expect("Black player starting location on square 77 should be a valid location."),
            white_available_walls: 10,
            black_available_walls: 10,
            wall_positions: [const { None }; 71],
            horizontal_blocks: Bitmap::<72>::new(),
            vertical_blocks: Bitmap::<80>::new(),
        }
    }
}

impl Boardstate {
    pub fn new() -> Boardstate {
        Boardstate::default()
    }

    pub fn start_from(
        white: PawnLocation,
        black: PawnLocation,
        walls: Vec<WallLocation>,
    ) -> Result<Boardstate, String> {
        let mut boardstate = Boardstate::new();
        for wall_location in walls {
            boardstate.insert_wall_at_location(wall_location)?;
        }
        boardstate.white_position = white;
        boardstate.black_position = black;

        Ok(boardstate)
    }

    pub fn get_active_player(&self) -> &Player {
        &self.active_player
    }

    pub fn get_position_white_pawn(&self) -> &PawnLocation {
        &self.white_position
    }

    pub fn get_position_black_pawn(&self) -> &PawnLocation {
        &self.black_position
    }

    pub fn get_available_walls_white_player(&self) -> &u8 {
        &self.white_available_walls
    }

    pub fn get_available_walls_black_player(&self) -> &u8 {
        &self.black_available_walls
    }

    pub fn get_wall_positions(&self) -> &[Option<WallOrientation>; 71] {
        &self.wall_positions
    }

    /// Method to get all the legal moves for the currently active player in the
    /// current boadstate.
    ///
    /// For the pawn moves this means checking all the move directions and the weird
    /// rules around hoping over other players (need to figure out clean way to do that)
    /// for the wall moves it needs to check all the open wall positions, see what direction would
    /// be available (not blocked by existing wall) and check if it doesn't block all paths to the
    /// opposite side for either player.
    pub fn get_legal_actions(&self) -> PossibleActions {
        PossibleActions::build(self.get_possible_pawn_moves(), Vec::new())
    }

    /// The play action takes an action as input and attempts to play that move on the current
    /// board, if the action is illegal an error is returned. When the action is legal, the
    /// boardstate is updated.
    /// A check is executed if the game is won. When the game is won an enum with the Won result is
    /// returned, otherwise the active player is swapped and an InProgress enum is returned
    pub fn apply_action(&mut self, action: Action) -> Result<GameStatus, String> {
        match action {
            Action::Pawn(pawn_location) => self.move_pawn_to_location(pawn_location),
            Action::Wall(wall_location) => self.insert_wall_at_location(wall_location),
        }
    }

    pub fn is_blocked_horizontal(&self, location: PawnLocation) -> bool {
        let coordinate = location.get_coordinate();

        let mut current_square_wall = false;
        if coordinate.x < 8 && coordinate.y < 8 {
            if let Some(WallOrientation::Horizontal) =
                self.wall_positions[usize::from(location.get_square())]
            {
                //println!("{:?}, {}", coordinate, location.get_square());
                current_square_wall = true;
            }
        }

        let mut left_square_wall = false;
        if coordinate.x > 0 && coordinate.y < 8 {
            if let Some(WallOrientation::Horizontal) =
                self.wall_positions[usize::from(location.get_square() - 1)]
            {
                //println!("{:?}, {} - 1", coordinate, location.get_square());
                left_square_wall = true;
            }
        }
        current_square_wall | left_square_wall
    }

    pub fn is_blocked_vertical(&self, location: PawnLocation) -> bool {
        let coordinate = location.get_coordinate();

        let mut current_square_wall = false;
        if coordinate.x < 8 && coordinate.y < 8 {
            if let Some(WallOrientation::Vertical) =
                self.wall_positions[usize::from(location.get_square())]
            {
                //println!("{:?}, {}", coordinate, location.get_square());
                current_square_wall = true;
            }
        }

        let mut lower_square_wall = false;
        if coordinate.x < 7 && coordinate.y > 0 {
            if let Some(WallOrientation::Vertical) =
                self.wall_positions[usize::from(location.get_square() - 9)]
            {
                //println!("{:?}, {} - 9", coordinate, location.get_square());
                lower_square_wall = true;
            }
        }
        current_square_wall | lower_square_wall
    }

    /// Attempts to insert a wall for the currently active player.
    ///
    /// Checks if the player has walls available, if the location is not blocked by another wall
    /// and if the wall doesn't completly block of the opponent from reaching the other side.
    ///
    /// At a successfull insert the active player is swapped.
    fn insert_wall_at_location(&mut self, location: WallLocation) -> Result<GameStatus, String> {
        self.check_if_walls_available()?;
        if self.wall_positions[usize::from(location.get_square())].is_some() {
            return Err(format!(
                "Can't insert wall at location: {}. A wall already exists",
                location.get_square()
            ));
        }

        match location.get_orientation() {
            WallOrientation::Horizontal => {
                if self
                    .horizontal_blocks
                    .get(location.get_square().into())
                    || self
                        .horizontal_blocks
                        .get((location.get_square() + 1).into())
                {
                    return Err(format!(
                        "Can't insert a {:?} wall at location {}. Because it would overlaps an existing wall",
                        location.get_orientation(),
                        location.get_square()
                    ));
                }
                self.wall_positions[usize::from(location.get_square())] =
                    Some(location.get_orientation());
                self.horizontal_blocks
                    .set(location.get_square().into(), true);
                self.horizontal_blocks
                    .set((location.get_square() + 1).into(), true);

                self.decrease_available_walls();

                self.active_player = match self.active_player {
                    Player::White => Player::Black,
                    Player::Black => Player::White,
                };
            }
            WallOrientation::Vertical => {
                if self.vertical_blocks.get(location.get_square().into())
                    || self
                        .vertical_blocks
                        .get((location.get_square() + 9).into())
                {
                    return Err(format!(
                        "Can't insert a {:?} wall at location {}. Because it would overlaps an existing wall",
                        location.get_orientation(),
                        location.get_square()
                    ));
                }
                self.wall_positions[usize::from(location.get_square())] =
                    Some(location.get_orientation());
                self.vertical_blocks.set(location.get_square().into(), true);
                self.vertical_blocks
                    .set((location.get_square() + 9).into(), true);
                self.decrease_available_walls();
                self.active_player = match self.active_player {
                    Player::White => Player::Black,
                    Player::Black => Player::White,
                };
            }
        }

        Ok(GameStatus::InProgress)
    }

    fn check_if_walls_available(&self) -> Result<(), String> {
        match self.active_player {
            Player::White => {
                if self.white_available_walls >= 1 {
                    return Ok(());
                }
            }
            Player::Black => {
                if self.black_available_walls >= 1 {
                    return Ok(());
                }
            }
        }
        Err(String::from(
            "Can't insert wall, the active player has no more walls left.",
        ))
    }

    fn decrease_available_walls(&mut self) {
        match self.active_player {
            Player::White => self.white_available_walls -= 1,
            Player::Black => self.black_available_walls -= 1,
        }
    }

    fn move_pawn_to_location(&mut self, location: PawnLocation) -> Result<GameStatus, String> {
        let possible_pawn_moves = self.get_possible_pawn_moves();

        if possible_pawn_moves.contains(&location) {
            match self.active_player {
                Player::White => {
                    self.white_position = location;
                    if self.state_is_won() {
                        // Need to figure out if I still need to switch the game state is won
                        // it seems sort of important to be able for normal undo behavior in the
                        // case of playing the computer?
                        self.active_player = Player::Black;
                        return Ok(GameStatus::Finished {
                            won_by: Player::White,
                            reason: VictoryReason::ReachedOppositeSide,
                        });
                    }
                    self.active_player = Player::Black;
                    return Ok(GameStatus::InProgress);
                }
                Player::Black => {
                    self.black_position = location;
                    if self.state_is_won() {
                        self.active_player = Player::White;
                        return Ok(GameStatus::Finished {
                            won_by: Player::Black,
                            reason: VictoryReason::ReachedOppositeSide,
                        });
                    }
                    self.active_player = Player::White;
                    return Ok(GameStatus::InProgress);
                }
            }
        }

        Err(format!(
            "The move to square {} is not legal.",
            location.get_square()
        ))
    }

    fn get_possible_pawn_moves(&self) -> Vec<PawnLocation> {
        let current_location = match self.active_player {
            Player::White => &self.white_position,
            Player::Black => &self.black_position,
        };

        let mut possible_pawn_moves: Vec<PawnLocation> = Vec::with_capacity(4);
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if let Ok(new_location) = self.check_direction(current_location, &direction) {
                possible_pawn_moves.push(new_location)
            }
        }

        possible_pawn_moves
    }

    /// Check if moving in a direction is possible, either returns the new position or an error.
    ///
    /// At some point this should probably also handle the jump
    fn check_direction(
        &self,
        location: &PawnLocation,
        direction: &Direction,
    ) -> Result<PawnLocation, String> {
        match direction {
            Direction::North => {
                if location.get_coordinate().y == 8 {
                    return Err(format!("Going North from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self
                    .horizontal_blocks
                    .get(location.get_square().into())
                {
                    return Err(format!(
                        "Going North from square {} is blocked by a wall",
                        location.get_square()
                    ));
                };

                self.check_new_location_for_other_player(PawnLocation::build(
                    location.get_square() + 9,
                )?)
            }
            Direction::East => {
                if location.get_coordinate().x == 8 {
                    return Err(format!("Going East from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self.vertical_blocks.get(location.get_square().into()) {
                    return Err(format!(
                        "Going East from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                }

                self.check_new_location_for_other_player(PawnLocation::build(
                    location.get_square() + 1,
                )?)
            }
            Direction::South => {
                if location.get_coordinate().y == 0 {
                    return Err(format!("Going West from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self
                    .horizontal_blocks
                    .get((location.get_square() - 1).into())
                {
                    return Err(format!(
                        "Going West from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                }

                self.check_new_location_for_other_player(PawnLocation::build(
                    location.get_square() - 9,
                )?)
            }
            Direction::West => {
                if location.get_coordinate().x == 0 {
                    return Err(format!("Going West from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self
                    .vertical_blocks
                    .get((location.get_square() - 1).into())
                {
                    return Err(format!(
                        "Going West from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                }

                self.check_new_location_for_other_player(PawnLocation::build(
                    location.get_square() - 1,
                )?)
            }
        }
    }

    fn check_new_location_for_other_player(
        &self,
        location: PawnLocation,
    ) -> Result<PawnLocation, String> {
        let occupied = match self.active_player {
            Player::White => location.get_square() == self.black_position.get_square(),
            Player::Black => location.get_square() == self.white_position.get_square(),
        };
        if occupied {
            return Err(format!(
                "Going to square {} is blocked by the other player",
                location.get_square()
            ));
        }
        Ok(location)
    }

    fn state_is_won(&self) -> bool {
        match self.active_player {
            Player::White => self.white_position.get_coordinate().y == 8,
            Player::Black => self.black_position.get_coordinate().y == 1,
        }
    }
}

#[derive(Clone, Hash, Debug)]
/// An enum with the two player options
pub enum Player {
    White,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_possible_pawn_moves_starting_position() {
        let boardstate = Boardstate::new();
        let result = boardstate.get_possible_pawn_moves();
        let expected = vec![
            PawnLocation::build(13).unwrap(),
            PawnLocation::build(5).unwrap(),
            PawnLocation::build(3).unwrap(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn move_pawn_up_from_starting_position() {
        let mut boardstate = Boardstate::new();
        boardstate
            .move_pawn_to_location(PawnLocation::build(13).unwrap())
            .unwrap();

        assert_eq!(boardstate.get_position_white_pawn().get_square(), 13);
    }

    #[test]
    #[should_panic]
    fn move_pawn_to_center_from_starting_position() {
        let mut boardstate = Boardstate::new();
        boardstate
            .move_pawn_to_location(PawnLocation::build(40).unwrap())
            .unwrap();
    }

    #[test]
    fn insert_wall_successfull() {
        let mut boardstate = Boardstate::new();
        boardstate
            .insert_wall_at_location(WallLocation::build(41, WallOrientation::Horizontal).unwrap())
            .unwrap();
        boardstate
            .insert_wall_at_location(WallLocation::build(0, WallOrientation::Vertical).unwrap())
            .unwrap();
        boardstate
            .insert_wall_at_location(WallLocation::build(70, WallOrientation::Horizontal).unwrap())
            .unwrap();

        assert_eq!(
            boardstate.get_wall_positions()[41],
            Some(WallOrientation::Horizontal)
        );
        assert_eq!(
            boardstate.get_wall_positions()[0],
            Some(WallOrientation::Vertical)
        );
        assert_eq!(
            boardstate.get_wall_positions()[70],
            Some(WallOrientation::Horizontal)
        );
    }
}

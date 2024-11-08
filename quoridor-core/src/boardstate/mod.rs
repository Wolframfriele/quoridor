#![allow(dead_code)]
#![allow(unused_variables)]

mod locations;

use fixedbitset::FixedBitSet;
use locations::{PawnLocation, WallLocation, WallOrientation};
use strum::IntoEnumIterator;

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
    horizontal_blocks: FixedBitSet,
    vertical_blocks: FixedBitSet,
}

impl Default for Boardstate {
    fn default() -> Boardstate {
        Boardstate {
            active_player: Player::White,
            white_position: PawnLocation::build(5)
                .expect("White player starting location on square 5 should be a valid location."),
            black_position: PawnLocation::build(77)
                .expect("Black player starting location on square 77 should be a valid location."),
            white_available_walls: 10,
            black_available_walls: 10,
            wall_positions: [const { None }; 71],
            horizontal_blocks: FixedBitSet::with_capacity(72),
            vertical_blocks: FixedBitSet::with_capacity(80),
        }
    }
}

impl Boardstate {
    pub fn new() -> Boardstate {
        Boardstate::default()
    }

    // problably will need some sort off method to start the board off in a certain position,
    // either from notation or from some other thing.

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
    pub fn get_legal_moves(&self) -> PossibleActions {
        let mut possible_moves = PossibleActions::default();

        let possible_pawn_moves = self.get_possible_pawn_moves();

        for new_location in possible_pawn_moves {
            possible_moves.add_pawn_action(new_location);
        }

        possible_moves
    }

    /// The play action takes an action as input and attempts to play that move on the current
    /// board, if the action is illegal an error is returned. When the action is legal, the
    /// boardstate is updated.
    /// A check is executed if the game is won. When the game is won an enum with the Won result is
    /// returned, otherwise the active player is swapped and an InProgress enum is returned
    pub fn play_action(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Pawn(location) => self.move_pawn_to_location(location),
            Action::Wall(wall_location) => Ok(()),
        }
    }

    /// When playing against the computer a player should be able to take back moves, also usefull
    /// for stepping through a game history. Keeping the history of the game is the responsability
    /// of the gamestate.
    pub fn undo_action(&self, action: Action) -> Result<(), String> {
        Ok(())
    }

    fn insert_wall_at_location(&self, location: WallLocation) -> Result<(), String> {
        // Check if no wall exists at the location.
        // Check if none off the edges for the WallLocation are set already.
        // If location is clear, set wall else return error.
        //
        // Run path finding from both pawns to check if it is still possible to reach the other
        // side.
        // If this is impossible undo the wall placement and return error.

        Ok(())
    }

    fn move_pawn_to_location(&mut self, location: PawnLocation) -> Result<(), String> {
        let possible_pawn_moves = self.get_possible_pawn_moves();

        if possible_pawn_moves.contains(&location) {
            match self.active_player {
                Player::White => {
                    self.white_position = location;
                    return Ok(());
                }
                Player::Black => {
                    self.black_position = location;
                    return Ok(());
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
        for direction in locations::Direction::iter() {
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
        location: &locations::PawnLocation,
        direction: &locations::Direction,
    ) -> Result<locations::PawnLocation, String> {
        match direction {
            locations::Direction::North => {
                if location.get_coordinate().y == 9 {
                    return Err(format!("Going North from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self
                    .horizontal_blocks
                    .contains(location.get_square().into())
                {
                    return Err(format!(
                        "Going North from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                };

                PawnLocation::build(location.get_square() + 9)
            }
            locations::Direction::East => {
                if location.get_coordinate().x == 9 {
                    return Err(format!("Going East from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self.vertical_blocks.contains(location.get_square().into()) {
                    return Err(format!(
                        "Going East from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                }

                PawnLocation::build(location.get_square() + 1)
            }
            locations::Direction::South => {
                if location.get_coordinate().y == 1 {
                    return Err(format!("Going West from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self
                    .horizontal_blocks
                    .contains((location.get_square() - 1).into())
                {
                    return Err(format!(
                        "Going West from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                }

                PawnLocation::build(location.get_square() - 9)
            }
            locations::Direction::West => {
                if location.get_coordinate().x == 1 {
                    return Err(format!("Going West from square {0} is impossible since it is on the edge of the board", location.get_square()));
                }
                if self
                    .vertical_blocks
                    .contains((location.get_square() - 1).into())
                {
                    return Err(format!(
                        "Going West from square {0} is blocked by a wall",
                        location.get_square()
                    ));
                }

                PawnLocation::build(location.get_square() - 1)
            }
        }
    }
}

#[derive(Clone, Hash, Debug)]
/// An enum with the two players
pub enum Player {
    White,
    Black,
}

pub enum Action {
    Pawn(PawnLocation),
    Wall(WallLocation),
}

impl Action {
    pub fn from_notation(coordinate_notation: &str) -> Result<Self, String> {
        // WIP Ipmlementations since it made more sense to have this method on the Action struct,
        // but I won't need this until much later
        match coordinate_notation.len() {
            0..=1 => Err(
                "Trying to create an action from a notation string that has less than 2 characters"
                    .to_string(),
            ),
            2 => Ok(Action::Pawn(PawnLocation::build(5).unwrap())),
            3 => Ok(Action::Wall(
                WallLocation::build(5, WallOrientation::Horizontal).unwrap(),
            )),
            _ => Err(
                "Trying to create an action from a notation string that has more than 3 characters"
                    .to_string(),
            ),
        }

        //     if let Some(x) =['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'].iter().position(|&i| i == x) {
    }
}

#[derive(Default)]
pub struct PossibleActions {
    pawn_actions: Vec<PawnLocation>,
    wall_actions: Vec<WallLocation>,
}

impl PossibleActions {
    fn add_pawn_action(&mut self, coordinate: PawnLocation) {
        self.pawn_actions.push(coordinate);
    }

    fn add_wall_action(&mut self, wall_move: WallLocation) {
        self.wall_actions.push(wall_move);
    }

    pub fn get_pawn_actions(&self) -> &Vec<PawnLocation> {
        &self.pawn_actions
    }

    pub fn get_wall_actions(&self) -> &Vec<WallLocation> {
        &self.wall_actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_possible_pawn_moves_starting_position() {
        let boardstate = Boardstate::new();
        let result = boardstate.get_possible_pawn_moves();
        let expected = vec![
            locations::PawnLocation::build(14).unwrap(),
            locations::PawnLocation::build(6).unwrap(),
            locations::PawnLocation::build(4).unwrap(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn move_pawn_up_from_starting_position() {
        let mut boardstate = Boardstate::new();
        boardstate.move_pawn_to_location(PawnLocation::build(14).unwrap()).unwrap();
        
        assert_eq!(boardstate
            .get_position_white_pawn().get_square(), 14);
    }

    #[test]
    #[should_panic]
    fn move_pawn_to_center_from_starting_position() {
        let mut boardstate = Boardstate::new();
        boardstate.move_pawn_to_location(PawnLocation::build(41).unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_action_from_too_short_notation_failed() {
        Action::from_notation("a").unwrap();
    }

    #[test]
    #[should_panic]
    fn new_action_from_too_long_notation_failed() {
        Action::from_notation("B1vx").unwrap();
    }

    #[test]
    #[should_panic]
    fn new_action_from_incorrect_notation_failed() {
        Action::from_notation("x1").unwrap();
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]

mod locations;

use fixedbitset::FixedBitSet;
use locations::{Location, WallLocation, WallOrientation};

#[derive(Clone, Hash)]
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
    white_position: Location,
    black_position: Location,
    white_available_walls: u8,
    black_available_walls: u8,
    wall_positions: [Option<WallOrientation>; 71],
    horizontal_edges: FixedBitSet,
    vertical_edges: FixedBitSet,
}

impl Boardstate {
    pub fn new() -> Boardstate {
        Boardstate {
            active_player: Player::White,
            white_position: Location::build(5)
                .expect("White player starting location on square 5 should be a valid location."),
            black_position: Location::build(77)
                .expect("Black player starting location on square 77 should be a valid location."),
            white_available_walls: 10,
            black_available_walls: 10,
            wall_positions: [const { None }; 71],
            horizontal_edges: FixedBitSet::with_capacity(72),
            vertical_edges: FixedBitSet::with_capacity(80),
        }
    }

    // problably will need some sort off method to start the board off in a certain position,
    // either from notation or from some other thing.

    pub fn get_position_white_pawn(&self) -> &Location {
        &self.white_position
    }

    pub fn get_position_black_pawn(&self) -> &Location {
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

        possible_moves.add_pawn_action(Location::build(2).unwrap());
        possible_moves
    }

    /// The play action takes an action as input and attempts to play that move on the current
    /// board, if the action is illegal an error is returned. When the action is legal, the
    /// boardstate is updated.
    /// A check is executed if the game is won. When the game is won an enum with the Won result is
    /// returned, otherwise the active player is swapped and an InProgress enum is returned
    pub fn play_action(&self, action: Action) -> Result<(), String> {
        match action {
            Action::Pawn(location) => (),
            Action::Wall(wall_location) => (),
        }
        Ok(())
    }

    /// When playing against the computer a player should be able to take back moves, also usefull
    /// for stepping through a game history. Keeping the history of the game is the responsability
    /// of the gamestate.
    pub fn undo_action(&self, action: Action) -> Result<(), String> {
        Ok(())
    }

    fn move_pawn_to_location(&self, location: Location) -> Result<(), String> {
        // Get possible pawn moves from current position and check if the new location is one of
        // the legal options

        // If legal, set the pawn of the active player to the new location else return error

        Ok(())
    }

    fn get_possible_pawn_moves(&self) {
        let current_location = match self.active_player {
            Player::White => &self.white_position,
            Player::Black => &self.black_position,
        };
    }
}

#[derive(Clone, Hash)]
/// An enum with the two players
pub enum Player {
    White,
    Black,
}

pub enum Action {
    Pawn(Location),
    Wall(WallLocation),
}

#[derive(Default)]
pub struct PossibleActions {
    pawn_actions: Vec<Location>,
    wall_actions: Vec<WallLocation>,
}

impl PossibleActions {
    fn add_pawn_action(&mut self, coordinate: Location) {
        self.pawn_actions.push(coordinate);
    }

    fn add_wall_action(&mut self, wall_move: WallLocation) {
        self.wall_actions.push(wall_move);
    }

    pub fn get_pawn_actions(&self) -> &Vec<Location> {
        &self.pawn_actions
    }

    pub fn get_wall_actions(&self) -> &Vec<WallLocation> {
        &self.wall_actions
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_new_location() {
//         // keep it for later
//     }
// }

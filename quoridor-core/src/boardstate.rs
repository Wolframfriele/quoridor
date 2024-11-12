use fixedbitset::FixedBitSet;

use crate::locations::{PawnLocation, WallLocation, WallOrientation, Direction};
use crate::actions::{Action, PossibleActions};

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
            white_position: PawnLocation::build(4)
                .expect("White player starting location on square 5 should be a valid location."),
            black_position: PawnLocation::build(76)
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
    ///
    /// At some point it is prabably nice to get an unchecked version of this method that returns a
    /// new boardstate. When doing the MCTS simulations you will only use random moves from
    /// get_legal_moves, there is no need to execute a bunch of the validation logic around played
    /// actions. And that could save a lot of compute.
    pub fn apply_action(&mut self, action: Action) -> Result<Status, String> {
        match action {
            Action::Pawn(pawn_location) => self.move_pawn_to_location(pawn_location),
            Action::Wall(wall_location) => self.insert_wall_at_location(wall_location),
        }
    }

    /// Attempts to insert a wall for the currently active player.
    ///
    /// Checks if the player has walls available, if the location is not blocked by another wall
    /// and if the wall doesn't completly block of the opponent from reaching the other side.
    ///
    /// At a successfull insert the active player is swapped.
    fn insert_wall_at_location(&mut self, location: WallLocation) -> Result<Status, String> {
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
                    .contains(location.get_square().into())
                    || self
                        .horizontal_blocks
                        .contains((location.get_square() + 1).into())
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
                if self.vertical_blocks.contains(location.get_square().into())
                    || self
                        .vertical_blocks
                        .contains((location.get_square() + 9).into())
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

        Ok(Status::InProgress)
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

    fn move_pawn_to_location(&mut self, location: PawnLocation) -> Result<Status, String> {
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
                        return Ok(Status::Finished(Player::White));
                    }
                    self.active_player = Player::Black;
                    return Ok(Status::InProgress);
                }
                Player::Black => {
                    self.black_position = location;
                    if self.state_is_won() {
                        self.active_player = Player::White;
                        return Ok(Status::Finished(Player::Black));
                    }
                    self.active_player = Player::White;
                    return Ok(Status::InProgress);
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
        for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
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
                    .contains(location.get_square().into())
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
                if self.vertical_blocks.contains(location.get_square().into()) {
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
                    .contains((location.get_square() - 1).into())
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
                    .contains((location.get_square() - 1).into())
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

    pub fn print_board_state(&self) {
        for y in (0..=8u8).rev() {
            let mut horizontal_walls = String::from("  |");
            for x in 0..=8u8 {
                let square = ((y) * 9) + x;
                horizontal_walls.push_str(&self.format_horizontal_wall(square));
            }
            println!("{horizontal_walls}");

            let mut vertical_walls_and_paws = format!("{} |", y + 1);
            for x in 0..=8u8 {
                let square = ((y) * 9) + x;
                vertical_walls_and_paws.push_str(
                    format!(
                        "  {}  {}",
                        self.format_pawn(square),
                        self.format_vertical_wall(square)
                    )
                    .as_str(),
                );
            }
            println!("{vertical_walls_and_paws}");
        }
        println!("  |-----|-----|-----|-----|-----|-----|-----|-----|-----|");
        println!("     A     B     C     D     E     F     G     H     I   \n\n");
    }

    fn format_pawn(&self, square: u8) -> char {
        if self.white_position.get_square() == square {
            return 'O';
        }
        if self.black_position.get_square() == square {
            return 'X';
        }
        ' '
    }

    fn format_horizontal_wall(&self, square: u8) -> String {
        let mut horizontal_line = String::from("--");
        if self.horizontal_blocks.contains(square.into()) {
            horizontal_line.push('#')
        } else {
            horizontal_line.push('-')
        }
        if square < 70 && self.wall_positions[usize::from(square)].is_some() {
            horizontal_line.push_str("--#");
        } else {
            horizontal_line.push_str("--|");
        }
        horizontal_line
    }

    fn format_vertical_wall(&self, square: u8) -> char {
        if self.vertical_blocks.contains(square.into()) {
            return '#';
        }
        '|'
    }
}

#[derive(Clone, Hash, Debug)]
/// An enum with the two player options
pub enum Player {
    White,
    Black,
}

pub enum Status {
    InProgress,
    Finished(Player),
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

use strum::EnumIter;

#[derive(Debug)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub struct PawnLocation {
    square: u8,
}

impl PawnLocation {
    /// Create a new Location, the number is the square on the board where counting starts in the
    /// bottom left corner and goes right. So coordinate A1 is square 1, B1 is square 2 and A2 is
    /// square 10 etc.
    ///
    /// The square number must be between 1 and 81, or else an error will be returned.
    pub fn build(square: u8) -> Result<Self, String> {
        if (1..=81).contains(&square) {
            Ok(PawnLocation { square })
        } else {
            Err(format!(
                "The square should be in range 1..=81, but was: {square}"
            ))
        }
    }

    pub fn get_square(&self) -> u8 {
        assert!((1..=81).contains(&self.square));
        self.square
    }

    pub fn get_coordinate(&self) -> Coordinate {
        let remainder = &self.square % 9;
        if remainder == 0 {
            Coordinate {
                x: 9,
                y: &self.square / 9,
            }
        } else {
            Coordinate {
                x: remainder,
                y: (&self.square / 9) + 1,
            }
        }
    }

    /// I don't think I love where this logic is now, maybe it could be nice to add some off the
    /// max values etc to direction and than do all the checking in the boardstate, than I only
    /// have to have this logic once
    fn new_location_from_direction(&self, direction: Direction) -> Result<PawnLocation, String> {
        println!("{}", 21 % 9);
        match direction {
            Direction::North => PawnLocation::build(&self.square + 9),
            Direction::East => {
                if self.get_coordinate().x != 9 {
                    PawnLocation::build(&self.square + 1)
                } else {
                    Err(format!("Going East from square {0} is impossible since it is on the edge of the board", &self.square))
                }
            }
            Direction::South => PawnLocation::build(&self.square - 9),
            Direction::West => {
                if self.get_coordinate().x != 1 {
                    PawnLocation::build(&self.square - 1)
                } else {
                    Err(format!("Going West from square {0} is impossible since it is on the edge of the board", &self.square))
                }
            }
        }
    }
}

#[derive(EnumIter, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Hash, Debug)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct WallLocation {
    square: u8,
    orientation: WallOrientation,
}

impl WallLocation {
    /// Create a new WallCoordinate. There are 64 unique positions that a wall can be placed in,
    /// but the node value can be between 1 and 71. This is because the coordinate of the squares
    /// is
    pub fn build(square: u8, orientation: WallOrientation) -> Result<Self, String> {
        if (1..=71).contains(&square) && &square % 9 != 0 {
            Ok(WallLocation {
                square,
                orientation,
            })
        } else {
            Err(format!("The square should be in the range 1..=71 and should not be divisible by 9, but was {square}"))
        }
    }

    pub fn get_square(&self) -> u8 {
        assert!((1..=71).contains(&self.square));
        self.square
    }

    pub fn get_orientation(&self) -> WallOrientation {
        self.orientation.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_from_location() {
        let combinations = [
            (PawnLocation::build(1).unwrap(), Coordinate { x: 1, y: 1 }),
            (PawnLocation::build(19).unwrap(), Coordinate { x: 1, y: 3 }),
            (PawnLocation::build(21).unwrap(), Coordinate { x: 3, y: 3 }),
            (PawnLocation::build(36).unwrap(), Coordinate { x: 9, y: 4 }),
            (PawnLocation::build(45).unwrap(), Coordinate { x: 9, y: 5 }),
            (PawnLocation::build(77).unwrap(), Coordinate { x: 5, y: 9 }),
            (PawnLocation::build(81).unwrap(), Coordinate { x: 9, y: 9 }),
        ];
        for (input, expected) in combinations {
            assert_eq!(input.get_coordinate().x, expected.x);
            assert_eq!(input.get_coordinate().y, expected.y);
        }
    }

    #[test]
    fn new_pawn_location_from_direction_east_succesfull() {
        let starting_location = PawnLocation::build(5).unwrap();
        let result = starting_location.new_location_from_direction(Direction::East);
        assert_eq!(result.unwrap().get_square(), 6)
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_north_failed() {
        let starting_location = PawnLocation::build(77).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::North)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_east_failed() {
        let starting_location = PawnLocation::build(9).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::East)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_south_failed() {
        let starting_location = PawnLocation::build(5).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::South)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn new_pawn_location_from_direction_west_failed() {
        let starting_location = PawnLocation::build(46).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::West)
            .unwrap();
    }

    #[test]
    fn new_walllocation_successfull() {
        let invalid_square: Vec<u8> = vec![9, 18, 27, 36, 45, 54, 63];
        for square in 1..=71 {
            if !invalid_square.contains(&square) {
                let location = WallLocation::build(square, WallOrientation::Vertical).unwrap();
                assert_eq!(location.get_square(), square);
            }
        }
    }

    #[test]
    #[should_panic]
    fn new_walllocation_failed() {
        let location = WallLocation::build(36, WallOrientation::Horizontal).unwrap();
    }
}

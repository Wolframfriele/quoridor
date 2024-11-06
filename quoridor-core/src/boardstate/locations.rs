#[derive(Clone, Hash)]
pub struct Location {
    square: u8,
}

impl Location {
    /// Create a new Location, the number is the square on the board where counting starts in the
    /// bottom left corner and goes right. So coordinate A1 is square 1, B1 is square 2 and A2 is
    /// square 10 etc.
    ///
    /// The square number must be between 1 and 81, or else an error will be returned.
    pub fn build(square: u8) -> Result<Self, String> {
        if (1..=81).contains(&square) {
            Ok(Location { square })
        } else {
            Err(format!(
                "The square should be in range 1..=81, but was: {square}"
            ))
        }
    }

    pub fn from_notation(coordinate_notation: &str) -> Result<Self, String> {
        // Convert quoridor notation string to coordinate; for example C2 is 12

        Ok(Location { square: 12 })
    }

    pub fn get_square(&self) -> u8 {
        assert!((1..=81).contains(&self.square));
        self.square
    }

    fn new_location_from_direction(&self, direction: Direction) -> Result<Location, String> {
        match direction {
            Direction::North => Location::build(&self.square + 9),
            Direction::East => {
                if &self.square % 9 != 0 {
                    Location::build(&self.square + 1)
                } else {
                    Err(format!("Going East from square {0} is impossible since it is on the edge of the board", &self.square))
                }
            }
            Direction::South => Location::build(&self.square - 9),
            Direction::West => {
                if ![1, 10, 19, 28, 37, 46, 55, 64, 73].contains(&self.square) {
                    Location::build(&self.square - 1)
                } else {
                    Err(format!("Going West from square {0} is impossible since it is on the edge of the board", &self.square))
                }
            }
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Hash)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_location_from_direction_east_succesfull() {
        let starting_location = Location::build(5).unwrap();
        let result = starting_location.new_location_from_direction(Direction::East);
        assert_eq!(result.unwrap().get_square(), 6)
    }

    #[test]
    #[should_panic]
    fn test_new_location_from_direction_north_failed() {
        let starting_location = Location::build(77).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::North)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_location_from_direction_east_failed() {
        let starting_location = Location::build(9).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::East)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_location_from_direction_south_failed() {
        let starting_location = Location::build(5).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::South)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_location_from_direction_west_failed() {
        let starting_location = Location::build(46).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::West)
            .unwrap();
    }

    #[test]
    fn test_new_walllocation_successfull() {
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
    fn test_new_walllocation_failed() {
        let location = WallLocation::build(36, WallOrientation::Horizontal).unwrap();
    }
}

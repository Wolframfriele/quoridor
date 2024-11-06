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
            Direction::East => Location::build(&self.square + 1),
            Direction::South => Location::build(&self.square - 9),
            Direction::West => Location::build(&self.square - 1),
        }
        // Err("Going East from square 9 is impossible since it is on the edge of the board")
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
    fn test_new_location_from_direction_east_failed() {
        let starting_location = Location::build(9).unwrap();
        let result = starting_location
            .new_location_from_direction(Direction::East)
            .unwrap();
    }
}

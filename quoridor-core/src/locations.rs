pub trait Location {
    fn get_square(&self) -> u8;

    fn get_coordinate(&self) -> Coordinate {
        Coordinate::from_square(self.get_square())
    }

    // Maybe it could be cool to implement the new location from direction here
    // fn new_location_from_direction(&self, direction: Direction) -> impl Location;
}

/// The location of a pawn on the board, represented by the number of the square that the pawn
/// occupies.
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
        if (0..=80).contains(&square) {
            Ok(PawnLocation { square })
        } else {
            Err(format!(
                "The square should be in range 0..=80, but was: {square}"
            ))
        }
    }

    pub fn from_coordinate(coordinate: Coordinate) -> Result<Self, String> {
        if coordinate.x < 9 && coordinate.y < 9 {
            Ok(PawnLocation {
                square: coordinate.to_square(),
            })
        } else {
            Err(format!(
                "The coordinate x and y should be in the range 0..=8, but where x: {} y: {}",
                coordinate.x, coordinate.y
            ))
        }
    }

    pub fn from_direction(&self, direction: Direction) -> Result<Self, String> {
        if let Some(new_coordinate) = self.get_coordinate().from_direction(direction) {
            return PawnLocation::from_coordinate(new_coordinate);
        }
        Err(String::from("Can't create new location from direction"))
    }
}

impl Location for PawnLocation {
    fn get_square(&self) -> u8 {
        self.square
    }
}

/// The location of a wall on the board, represented by the number of the square that touches the
/// center off the wall with the top right corner of the square.
#[derive(Clone, Debug, PartialEq)]
pub struct WallLocation {
    square: u8,
    orientation: WallOrientation,
}

impl WallLocation {
    /// Create a new WallCoordinate. There are 64 unique positions that a wall can be placed in,
    /// but the node value can be between 1 and 71. This is because the coordinate of the squares
    /// is
    pub fn build(square: u8, orientation: WallOrientation) -> Result<Self, String> {
        if (0..=70).contains(&square) && &square % 9 != 8 {
            Ok(WallLocation {
                square,
                orientation,
            })
        } else {
            Err(format!("The square should be in the range 0..=70 and should not be divisible by 9, but was {square}"))
        }
    }

    pub fn from_coordinate(
        coordinate: Coordinate,
        orientation: WallOrientation,
    ) -> Result<Self, String> {
        if coordinate.x < 8 && coordinate.y < 8 {
            Ok(WallLocation {
                square: coordinate.to_square(),
                orientation,
            })
        } else {
            Err(format!(
                "The coordinate x and y should be in the range 0..=7, but where x: {} y: {}",
                coordinate.x, coordinate.y
            ))
        }
    }

    pub fn get_orientation(&self) -> WallOrientation {
        self.orientation.clone()
    }
}

impl Location for WallLocation {
    fn get_square(&self) -> u8 {
        self.square
    }
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    pub fn from_square(square: u8) -> Coordinate {
        Coordinate {
            x: square % 9,
            y: square / 9,
        }
    }

    pub fn to_square(&self) -> u8 {
        (self.y) * 9 + self.x
    }

    pub fn from_direction(&self, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::North => {
                if self.y + 1 < 8 {
                    return Some(Coordinate {
                        x: self.x,
                        y: self.y + 1,
                    });
                }
            }
            Direction::East => {
                if self.x + 1 < 8 {
                    return Some(Coordinate {
                        x: self.x + 1,
                        y: self.y,
                    });
                }
            }
            Direction::South => {
                if self.y - 1 > 0 {
                    return Some(Coordinate {
                        x: self.x,
                        y: self.y - 1,
                    });
                }
            }
            Direction::West => {
                if self.x > 0 {
                    return Some(Coordinate {
                        x: self.x - 1,
                        y: self.y,
                    });
                }
            }
        }
        None
    }

    pub fn from_calculation(&self, x: i8, y: i8) -> Option<Coordinate> {
        let new_x = self.x.checked_add_signed(x)?;
        let new_y = self.y.checked_add_signed(y)?;

        if (0..=8).contains(&new_x) && (0..=8).contains(&new_y) {
            return Some(Coordinate { x: new_x, y: new_y });
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_from_location() {
        let combinations = [
            (PawnLocation::build(0).unwrap(), Coordinate { x: 0, y: 0 }),
            (PawnLocation::build(18).unwrap(), Coordinate { x: 0, y: 2 }),
            (PawnLocation::build(20).unwrap(), Coordinate { x: 2, y: 2 }),
            (PawnLocation::build(35).unwrap(), Coordinate { x: 8, y: 3 }),
            (PawnLocation::build(44).unwrap(), Coordinate { x: 8, y: 4 }),
            (PawnLocation::build(76).unwrap(), Coordinate { x: 4, y: 8 }),
            (PawnLocation::build(80).unwrap(), Coordinate { x: 8, y: 8 }),
        ];
        for (input, expected) in combinations {
            assert_eq!(input.get_coordinate().x, expected.x);
            assert_eq!(input.get_coordinate().y, expected.y);
        }
    }

    #[test]
    fn new_walllocation_successfull() {
        let invalid_square: Vec<u8> = vec![8, 17, 26, 35, 44, 53, 62];
        for square in 0..=70 {
            if !invalid_square.contains(&square) {
                let location = WallLocation::build(square, WallOrientation::Vertical).unwrap();
                assert_eq!(location.get_square(), square);
            }
        }
    }

    #[test]
    #[should_panic]
    fn new_walllocation_failed() {
        let _ = WallLocation::build(35, WallOrientation::Horizontal).unwrap();
    }
}

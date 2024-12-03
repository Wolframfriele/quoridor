use uuid::Uuid;

// TODO Define a trait to handle playing a game with a session and use that for all the player
// types. Then create an Anon that implements the trait
// I probably need some sort of id, and maybe a session token storage? Maybe a rating if I want to
// allow rated games in the case of anon players.
pub trait PlayerInfo {
    fn get_id(&self) -> &Uuid;

    fn get_rating(&self) -> u16;
}

pub type PlayerInfo 

pub struct AnonUser {
    id: Uuid,
    rating: u16,
}

impl AnonUser {
    pub fn new() -> Self {
        AnonUser {
            id: Uuid::new_v4(),
            rating: 1500,
        }
    }
}
impl PlayerInfo for AnonUser {
    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_rating(&self) -> u16 {
        self.rating
    }
}

pub struct User {
    id: Uuid,
    name: String,
    rating: u16,
    email: String,
    password: String,
}

pub struct Engine {
    id: Uuid,
    name: String,
    rating: u16,
}

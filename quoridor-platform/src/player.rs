use uuid::Uuid;

pub enum PlayerType {
    Human(User),
    Engine(Engine),
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

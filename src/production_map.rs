use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left, Right
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Left => "L",
            Direction::Right => "R",
        })
    }
}

#[derive(Clone, Debug)]
pub struct Transformation {
    pub new_state: String,
    pub write: char,
    pub direction: Direction,
}
impl Transformation {
    pub fn new(new_state: String, write: char, direction: Direction) -> Self {
        Transformation { new_state, write, direction }
    }
}

#[derive(Debug, Default)]
pub struct ProductionMap {
    pub map: HashMap<(String, char), Transformation>,
    pub initial_state: String,
    pub final_state: String,
}
impl ProductionMap {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

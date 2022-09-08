#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "top" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => Err(anyhow::anyhow!("Wrong direction")),
        }
    }
}

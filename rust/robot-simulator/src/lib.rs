// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    facing: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            facing: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Robot {
            facing,
            position: self.position,
        }
    }

    pub fn turn_left(self) -> Self {
        let facing = match self.facing {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        Robot {
            facing,
            position: self.position,
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.position;
        let position = match self.facing {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };

        Robot {
            position,
            facing: self.facing,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |bot, i| match i {
            'A' => bot.advance(),
            'R' => bot.turn_right(),
            'L' => bot.turn_left(),
            _ => bot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}

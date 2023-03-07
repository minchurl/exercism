// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Debug)]
pub struct Robot {
    x: i32, 
    y: i32, 
    d: Direction
}
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            d: d, 
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North =>  Direction::East, 
            Direction::East => Direction::South, 
            Direction::South => Direction::West, 
            Direction::West => Direction::North,
        };
        Robot {
            x: self.x,
            y: self.y, 
            d: d,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North =>  Direction::West, 
            Direction::East => Direction::North, 
            Direction::South => Direction::East, 
            Direction::West => Direction::South,
        };
        Robot {
            x: self.x,
            y: self.y, 
            d: d, 
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y, d) = (self.x, self.y, self.d);
        match d {
            Direction::North => Robot{x: x, y: y + 1, d: d}, 
            Direction::East => Robot{x: x + 1, y: y, d: d}, 
            Direction::South => Robot{x: x, y: y - 1, d: d}, 
            Direction::West => Robot{x: x - 1, y: y, d: d}, 
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, instruction| {
                match instruction {
                    'L' => robot.turn_left(), 
                    'R' => robot.turn_right(), 
                    'A' => robot.advance(), 
                    _ => panic!("invaild instruction {}", instruction), 
                }
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
impl From<i32> for Direction {
    fn from(x: i32) -> Self {
        match x {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}
#[derive(Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl std::fmt::Debug for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ROBOBI {{ x: {}, y: {}, d: {} }}",
            self.x, self.y, self.d
        )
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        let x = self.d as i32 + 1;
        let x = if x > 3 { 0 } else { x };
        self.d = Direction::from(x);
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        let x = self.d as i32 - 1;
        let x = if x < 0 { 3 } else { x };
        self.d = Direction::from(x);
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        self = match self.d {
            Direction::North => Robot {
                x: self.x,
                y: self.y + 1,
                d: self.d,
            },
            Direction::East => Robot {
                x: self.x + 1,
                y: self.y,
                d: self.d,
            },
            Direction::South => Robot {
                x: self.x,
                y: self.y - 1,
                d: self.d,
            },
            Direction::West => Robot {
                x: self.x - 1,
                y: self.y,
                d: self.d,
            },
        };
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'R' => {
                    self = self.turn_right();
                }
                'L' => {
                    self = self.turn_left();
                }
                'A' => {
                    self = self.advance();
                }
                _ => unreachable!(),
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

fn main() {
    let robot = Robot::new(2, -7, Direction::East).instructions("RRAAAAALA");
    println!("POS: {:?}", robot);
}

#[test]
fn robots_are_created_with_position_and_direction() {
    let robot = Robot::new(0, 0, Direction::North);
    assert_eq!((0, 0), robot.position());
    assert_eq!(&Direction::North, robot.direction());
}
#[test]

fn positions_can_be_negative() {
    let robot = Robot::new(-1, -1, Direction::South);
    assert_eq!((-1, -1), robot.position());
    assert_eq!(&Direction::South, robot.direction());
}
#[test]

fn turning_right_does_not_change_position() {
    let robot = Robot::new(0, 0, Direction::North).turn_right();
    assert_eq!((0, 0), robot.position());
}
#[test]

fn turning_right_from_north_points_the_robot_east() {
    let robot = Robot::new(0, 0, Direction::North).turn_right();
    assert_eq!(&Direction::East, robot.direction());
}
#[test]

fn turning_right_from_east_points_the_robot_south() {
    let robot = Robot::new(0, 0, Direction::East).turn_right();
    assert_eq!(&Direction::South, robot.direction());
}
#[test]

fn turning_right_from_south_points_the_robot_west() {
    let robot = Robot::new(0, 0, Direction::South).turn_right();
    assert_eq!(&Direction::West, robot.direction());
}
#[test]

fn turning_right_from_west_points_the_robot_north() {
    let robot = Robot::new(0, 0, Direction::West).turn_right();
    assert_eq!(&Direction::North, robot.direction());
}
#[test]

fn turning_left_does_not_change_position() {
    let robot = Robot::new(0, 0, Direction::North).turn_left();
    assert_eq!((0, 0), robot.position());
}
#[test]

fn turning_left_from_north_points_the_robot_west() {
    let robot = Robot::new(0, 0, Direction::North).turn_left();
    assert_eq!(&Direction::West, robot.direction());
}
#[test]

fn turning_left_from_west_points_the_robot_south() {
    let robot = Robot::new(0, 0, Direction::West).turn_left();
    assert_eq!(&Direction::South, robot.direction());
}
#[test]

fn turning_left_from_south_points_the_robot_east() {
    let robot = Robot::new(0, 0, Direction::South).turn_left();
    assert_eq!(&Direction::East, robot.direction());
}
#[test]

fn turning_left_from_east_points_the_robot_north() {
    let robot = Robot::new(0, 0, Direction::East).turn_left();
    assert_eq!(&Direction::North, robot.direction());
}
#[test]

fn advance_does_not_change_the_direction() {
    let robot = Robot::new(0, 0, Direction::North).advance();
    assert_eq!(&Direction::North, robot.direction());
}
#[test]

fn advance_increases_the_y_coordinate_by_one_when_facing_north() {
    let robot = Robot::new(0, 0, Direction::North).advance();
    assert_eq!((0, 1), robot.position());
}
#[test]

fn advance_decreases_the_y_coordinate_by_one_when_facing_south() {
    let robot = Robot::new(0, 0, Direction::South).advance();
    assert_eq!((0, -1), robot.position());
}
#[test]

fn advance_increases_the_x_coordinate_by_one_when_facing_east() {
    let robot = Robot::new(0, 0, Direction::East).advance();
    assert_eq!((1, 0), robot.position());
}
#[test]

fn advance_decreases_the_x_coordinate_by_one_when_facing_west() {
    let robot = Robot::new(0, 0, Direction::West).advance();
    assert_eq!((-1, 0), robot.position());
}
#[test]

fn follow_instructions_to_move_west_and_north() {
    let robot = Robot::new(0, 0, Direction::North).instructions("LAAARALA");
    assert_eq!((-4, 1), robot.position());
    assert_eq!(&Direction::West, robot.direction());
}
#[test]

fn follow_instructions_to_move_west_and_south() {
    let robot = Robot::new(2, -7, Direction::East).instructions("RRAAAAALA");
    assert_eq!((-3, -8), robot.position());
    assert_eq!(&Direction::South, robot.direction());
}
#[test]

fn follow_instructions_to_move_east_and_north() {
    let robot = Robot::new(8, 4, Direction::South).instructions("LAAARRRALLLL");
    assert_eq!((11, 5), robot.position());
    assert_eq!(&Direction::North, robot.direction());
}

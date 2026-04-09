#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn conflicts(&self, other: &Self) -> bool {
        match self {
            Self::Up => other == &Self::Down,
            Self::Down => other == &Self::Up,
            Self::Left => other == &Self::Right,
            Self::Right => other == &Self::Left,
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    pub positions: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut starting_position = Vec::new();

        for i in 20..25 {
            starting_position.push(Point { x: i, y: 25 });
        }

        Snake {
            positions: starting_position,
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self) {
        let _ = self.positions.remove(0);
        let current_head = self.positions.last().unwrap();

        let new_head = match self.direction {
            Direction::Up => Point {
                x: current_head.x,
                y: current_head.y + 1,
            },
            Direction::Down => Point {
                x: current_head.x,
                y: current_head.y - 1,
            },
            Direction::Left => Point {
                x: current_head.x - 1,
                y: current_head.y,
            },
            Direction::Right => Point {
                x: current_head.x + 1,
                y: current_head.y,
            },
        };

        self.positions.push(new_head);
    }

    pub fn grow(&mut self) {
        let tail = self.positions[0].clone();
        self.positions.insert(0, tail);
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !self.direction.conflicts(&new_direction) {
            self.direction = new_direction;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_conflicts_up() {
        assert!(Direction::Up.conflicts(&Direction::Down));

        assert!(!Direction::Up.conflicts(&Direction::Up));
        assert!(!Direction::Up.conflicts(&Direction::Left));
        assert!(!Direction::Up.conflicts(&Direction::Right));
    }

    #[test]
    fn direction_conflicts_down() {
        assert!(Direction::Down.conflicts(&Direction::Up));

        assert!(!Direction::Down.conflicts(&Direction::Down));
        assert!(!Direction::Down.conflicts(&Direction::Left));
        assert!(!Direction::Down.conflicts(&Direction::Right));
    }

    #[test]
    fn direction_conflicts_left() {
        assert!(Direction::Left.conflicts(&Direction::Right));

        assert!(!Direction::Left.conflicts(&Direction::Up));
        assert!(!Direction::Left.conflicts(&Direction::Down));
        assert!(!Direction::Left.conflicts(&Direction::Left));
    }

    #[test]
    fn direction_conflicts_right() {
        assert!(Direction::Right.conflicts(&Direction::Left));

        assert!(!Direction::Right.conflicts(&Direction::Up));
        assert!(!Direction::Right.conflicts(&Direction::Down));
        assert!(!Direction::Right.conflicts(&Direction::Right));
    }

    #[test]
    fn update_snake_going_right() {
        let initial_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Right,
        };

        let expected_positions = vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ];

        snake.update();

        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn update_snake_going_left() {
        let initial_positions = vec![
            Point { x: 10, y: 0 },
            Point { x: 9, y: 0 },
            Point { x: 8, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Left,
        };

        let expected_positions = vec![
            Point { x: 9, y: 0 },
            Point { x: 8, y: 0 },
            Point { x: 7, y: 0 },
        ];

        snake.update();

        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn update_snake_goes_past_min_x() {
        let initial_positions = vec![
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Left,
        };

        let expected_positions = vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: -1, y: 0 },
        ];

        snake.update();

        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn update_snake_goes_past_min_y() {
        let initial_positions = vec![
            Point { x: 1, y: 2 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Down,
        };

        let expected_positions = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: -1 },
        ];

        snake.update();

        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn update_snake_going_up() {
        let initial_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Up,
        };

        let expected_positions = vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ];

        snake.update();

        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn update_snake_going_down() {
        let initial_positions = vec![
            Point { x: 0, y: 10 },
            Point { x: 0, y: 9 },
            Point { x: 0, y: 8 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Down,
        };

        let expected_positions = vec![
            Point { x: 0, y: 9 },
            Point { x: 0, y: 8 },
            Point { x: 0, y: 7 },
        ];

        snake.update();

        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_to_up_if_going_right() {
        let initial_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Right,
        };

        snake.change_direction(Direction::Up);

        assert_eq!(snake.direction, Direction::Up);

        let expected_positions = vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_to_down_if_going_right() {
        let initial_positions = vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Right,
        };

        snake.change_direction(Direction::Down);

        assert_eq!(snake.direction, Direction::Down);

        let expected_positions = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 0 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_to_right_if_going_right_does_nothing() {
        let initial_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Right,
        };

        snake.change_direction(Direction::Right);

        assert_eq!(snake.direction, Direction::Right);

        let expected_positions = vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_left_if_going_right_does_nothing() {
        let initial_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Right,
        };

        snake.change_direction(Direction::Left);

        assert_eq!(snake.direction, Direction::Right);

        let expected_positions = vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_to_up_if_going_left() {
        let initial_positions = vec![
            Point { x: 3, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Left,
        };

        snake.change_direction(Direction::Up);

        assert_eq!(snake.direction, Direction::Up);

        let expected_positions = vec![
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_to_down_if_going_left() {
        let initial_positions = vec![
            Point { x: 3, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 1 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Left,
        };

        snake.change_direction(Direction::Down);

        assert_eq!(snake.direction, Direction::Down);

        let expected_positions = vec![
            Point { x: 2, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 0 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_left_if_going_left_does_nothing() {
        let initial_positions = vec![
            Point { x: 4, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Left,
        };

        snake.change_direction(Direction::Left);

        assert_eq!(snake.direction, Direction::Left);

        let expected_positions = vec![
            Point { x: 3, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn change_direction_right_if_going_left_does_nothing() {
        let initial_positions = vec![
            Point { x: 4, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Left,
        };

        snake.change_direction(Direction::Right);

        assert_eq!(snake.direction, Direction::Left);

        let expected_positions = vec![
            Point { x: 3, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 },
        ];

        snake.update();
        assert_eq!(snake.positions, expected_positions);
    }

    #[test]
    fn grow_snake() {

        let initial_positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];

        let mut snake = Snake {
            positions: initial_positions,
            direction: Direction::Right,
        };

        snake.grow();

        assert_eq!(snake.positions[0], Point {x: 0, y: 0});
        assert_eq!(snake.positions[1], Point {x: 0, y: 0});

        snake.update();
        assert_eq!(snake.positions[0], Point {x: 0, y: 0});
        assert_eq!(snake.positions[1], Point {x: 1, y: 0});
    }
}

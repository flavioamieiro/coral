#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    pub positions: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut starting_position = Vec::new();

        for i in 100..102 {
            starting_position.push(Point { x: i, y: 100 });
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

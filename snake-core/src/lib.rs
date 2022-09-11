use rand::distributions::Uniform;
use rand::prelude::Distribution;

use std::cmp::PartialEq;
use std::collections::{HashSet, LinkedList};
use std::vec::Vec;

const SNAKE_INITIAL_LENGTH: i32 = 3;
const SNAKE_INITIAL_POSITION_X: u32 = 10;
const SNAKE_INITIAL_POSITION_Y: u32 = 5;

const PLANE_WIDTH: u32 = 20;
const PLANE_HEIGHT: u32 = 10;

const APPLE_GENERATION_CHANCE_PER_MOVE: f32 = 1. / 20.;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaneField {
    Empty,
    Body,
    Apple,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionChangeError;

#[derive(Debug, Clone, PartialEq)]
struct AppleInsertError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(u32, u32);

#[derive(Debug)]
pub struct SnakeContext {
    plane: [[PlaneField; PLANE_WIDTH as usize]; PLANE_HEIGHT as usize],
    snake: LinkedList<Point>,
    apples: HashSet<Point>,
    direction: Direction,
}

impl SnakeContext {
    pub fn new() -> Self {
        Self {
            plane: [[PlaneField::Empty; PLANE_WIDTH as usize]; PLANE_HEIGHT as usize],
            snake: LinkedList::new(),
            apples: HashSet::new(),
            direction: Direction::Left,
        }
    }

    pub fn get_plane_string(&self) -> String {
        let mut str_buff = String::with_capacity(PLANE_WIDTH as usize * PLANE_HEIGHT as usize);
        for i in self.plane {
            for j in i {
                let char_to_display = match j {
                    PlaneField::Empty => '-',
                    PlaneField::Body => 'X',
                    PlaneField::Apple => '0',
                };
                str_buff.push(char_to_display);
            }
            str_buff.push('\n');
        }
        str_buff
    }

    pub fn new_game(&mut self) {
        self.reset_game();
        self.generate_snake();
        self.clear_plane();
        self.update_plane();
    }

    fn reset_game(&mut self) {
        self.snake.clear();
        self.apples.clear();
        self.direction = Direction::Left;
    }

    fn generate_snake(&mut self) {
        let values = (0..SNAKE_INITIAL_LENGTH)
            .into_iter()
            .map(|value| value as u32)
            .map(|offset| Point(SNAKE_INITIAL_POSITION_X + offset, SNAKE_INITIAL_POSITION_Y));
        self.snake.extend(values);
    }

    pub fn change_direction(&mut self, direction: Direction) -> Result<(), DirectionChangeError> {
        if self.direction == direction {
            return Ok(());
        }

        let elements = self
            .snake
            .iter()
            .take(2)
            .map(|point| point.1)
            .collect::<Vec<u32>>();
        let is_faced_horizontally = elements[0] == elements[1];
        if is_faced_horizontally && (direction == Direction::Left || direction == Direction::Right)
        {
            return Err(DirectionChangeError);
        } else if !is_faced_horizontally
            && (direction == Direction::Up || direction == Direction::Down)
        {
            return Err(DirectionChangeError);
        }

        self.direction = direction;
        Ok(())
    }

    pub fn current_direction(&self) -> Direction {
        self.direction
    }

    pub fn update_position(&mut self) {
        self.snake.pop_back();
        let Point(snake_head_x, snake_head_y) =
            *self.snake.front().expect("Snake should not be empty");
        let new_head = match self.direction {
            Direction::Up if snake_head_y == 0 => Point(snake_head_x, PLANE_HEIGHT - 1),
            Direction::Up => Point(snake_head_x, snake_head_y - 1),
            Direction::Down if snake_head_y >= PLANE_HEIGHT - 1 => Point(snake_head_x, 0),
            Direction::Down => Point(snake_head_x, snake_head_y + 1),
            Direction::Left if snake_head_x == 0 => Point(PLANE_WIDTH - 1, snake_head_y),
            Direction::Left => Point(snake_head_x - 1, snake_head_y),
            Direction::Right if snake_head_x >= PLANE_WIDTH - 1 => Point(0, snake_head_y),
            Direction::Right => Point(snake_head_x + 1, snake_head_y),
        };
        if self.check_for_game_over(new_head) {
            self.new_game();
            return;
        }

        self.handle_collisions_with_apples(new_head);
        self.snake.push_front(new_head);

        self.insert_apple_by_random();
        self.update_plane();
    }

    fn handle_collisions_with_apples(&mut self, head: Point) {
        let collides_with_apple = self.apples.iter().find(|&&apple| apple == head).is_some();
        if collides_with_apple {
            self.apples.remove(&head.clone());
            self.append_snake();
        }
    }

    fn check_for_game_over(&mut self, head: Point) -> bool {
        self.snake.iter().find(|&&body| body == head).is_some()
    }

    fn insert_apple_by_random(&mut self) {
        let mut rng = rand::thread_rng();
        let random_x = Uniform::from(0..PLANE_WIDTH - 1);
        let random_y = Uniform::from(0..PLANE_HEIGHT - 1);

        let random_apple = Uniform::from(0..(1. / APPLE_GENERATION_CHANCE_PER_MOVE) as i32);

        if random_apple.sample(&mut rng) == 0 {
            let _ = self.insert_apple(Point(random_x.sample(&mut rng), random_y.sample(&mut rng)));
        }
    }

    fn insert_apple(&mut self, apple_position: Point) -> Result<(), AppleInsertError> {
        match apple_position {
            Point(x, _) if x >= PLANE_WIDTH => return Err(AppleInsertError),
            Point(_, y) if y >= PLANE_HEIGHT => return Err(AppleInsertError),
            Point(_, _) => (),
        };

        let collides_with_snake = self
            .snake
            .iter()
            .any(|&snake_point| snake_point == apple_position);

        let collides_with_other_apples = self
            .apples
            .iter()
            .any(|&existing_apple| existing_apple == apple_position);

        if collides_with_snake || collides_with_other_apples {
            return Err(AppleInsertError);
        }

        self.apples.insert(apple_position);
        Ok(())
    }

    fn update_plane(&mut self) {
        self.clear_plane();
        for point in &self.snake {
            self.plane[point.1 as usize][point.0 as usize] = PlaneField::Body;
        }

        for apple in &self.apples {
            self.plane[apple.1 as usize][apple.0 as usize] = PlaneField::Apple;
        }
    }

    fn clear_plane(&mut self) {
        self.plane = [[PlaneField::Empty; PLANE_WIDTH as usize]; PLANE_HEIGHT as usize];
    }

    fn append_snake(&mut self) {
        let last_two = self
            .snake
            .iter()
            .rev()
            .take(2)
            .cloned()
            .rev()
            .collect::<Vec<Point>>();

        let last_two_elements_are_placed_horizontally = last_two[1].1 == last_two[0].1;
        if last_two_elements_are_placed_horizontally {
            let y = last_two[1].1;
            let should_append_right = last_two[1].0 as i32 > last_two[0].0 as i32;
            if should_append_right {
                let is_on_right_wall = last_two[1].0 == PLANE_WIDTH - 1;
                let x = if is_on_right_wall {
                    0
                } else {
                    last_two[1].0 + 1
                };
                self.snake.push_back(Point(x, y))
            } else {
                let is_on_left_wall = last_two[1].0 == 0;
                let x = if is_on_left_wall {
                    PLANE_WIDTH - 1
                } else {
                    last_two[1].0 - 1
                };
                self.snake.push_back(Point(x, y))
            }
        } else {
            let x = last_two[1].0;
            let should_append_down = last_two[1].1 as i32 > last_two[0].1 as i32;
            if should_append_down {
                let is_on_bottom_wall = last_two[1].1 == PLANE_HEIGHT - 1;
                let y = if is_on_bottom_wall {
                    0
                } else {
                    last_two[1].1 + 1
                };
                self.snake.push_back(Point(x, y))
            } else {
                let is_on_top_wall = last_two[1].1 == 0;
                let y = if is_on_top_wall {
                    PLANE_HEIGHT - 1
                } else {
                    last_two[1].1 - 1
                };
                self.snake.push_back(Point(x, y))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AppleInsertError, Direction, DirectionChangeError, PlaneField, Point, SnakeContext,
    };

    const PLANE_WIDTH: usize = 20;
    const PLANE_HEIGHT: usize = 10;

    #[test]
    fn test_should_create_empty_20_10_plane_after_initialization() {
        let game = SnakeContext::new();
        assert_eq!([[PlaneField::Empty; PLANE_WIDTH]; PLANE_HEIGHT], game.plane);
    }

    #[test]
    fn test_should_generate_snake_with_length_3_at_point_10_5_faced_to_the_left_when_new_game_call()
    {
        let mut game = SnakeContext::new();
        game.new_game();
        game.update_plane();
        let initial_pos = (10usize, 5usize);
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0 + 1]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0 + 2]
        );
        assert_eq!(Direction::Left, game.direction);
    }

    #[test]
    fn test_should_move_snake_body_to_the_left_when_update_position_call_after_new_game_call() {
        let mut game = SnakeContext::new();
        game.new_game();
        let initial_pos = (10usize, 5usize);
        game.update_position();
        game.update_plane();
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0 - 1]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0 + 1]
        );
    }

    #[test]
    fn test_should_return_direction_error_when_illegal_direction_change_horizontal_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        let result = game.change_direction(Direction::Right);
        assert_eq!(DirectionChangeError, result.unwrap_err());
    }

    #[test]
    fn test_should_change_direction_successfully_when_correct_direction_change_horizontal_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        let result = game.change_direction(Direction::Up);
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn test_should_return_direction_error_when_illegal_direction_change_vertical_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.change_direction(Direction::Up)
            .expect("Directions should be ok in this scenario");
        for _ in 0..3 {
            game.update_position();
        }
        let result = game.change_direction(Direction::Down);
        assert_eq!(DirectionChangeError, result.unwrap_err());
    }

    #[test]
    fn test_should_change_direction_successfully_when_correct_direction_change_vertical_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.change_direction(Direction::Up)
            .expect("Directions should be ok in this scenario");
        for _ in 0..3 {
            game.update_position();
        }
        let result = game.change_direction(Direction::Left);
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn test_should_move_snake_up_when_update_position_after_direction_change_to_up() {
        let mut game = SnakeContext::new();
        game.new_game();
        let initial_pos = (10usize, 5usize);
        game.change_direction(Direction::Up)
            .expect("Directions should be ok in this scenario");
        game.update_position();
        game.update_plane();
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize - 1][initial_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0 + 1]
        );
    }

    #[test]
    fn test_should_move_snake_to_opposite_wall_when_wall_reached_left_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        for _ in 0..10 {
            game.update_position();
        }
        game.update_position();
        game.update_plane();
        let expected_pos = (0usize, 5usize);
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 as usize][PLANE_WIDTH - 1]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 as usize][expected_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 as usize][expected_pos.0 + 1]
        );
    }

    #[test]
    fn test_should_move_snake_to_opposite_wall_when_wall_reached_up_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.change_direction(Direction::Up)
            .expect("Directions should be ok in this scenario");
        for _ in 0..5 {
            game.update_position();
        }
        game.update_position();
        game.update_plane();
        let expected_pos = (10usize, 0usize);
        assert_eq!(
            PlaneField::Body,
            game.plane[PLANE_HEIGHT - 1][expected_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 as usize][expected_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 + 1 as usize][expected_pos.0]
        );
    }

    #[test]
    fn test_should_move_snake_in_direction_when_not_collides_with_body() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.update_position();
        game.change_direction(Direction::Up)
            .expect("Directions should be ok in this scenario");
        game.update_position();
        game.change_direction(Direction::Right)
            .expect("Directions should be ok in this scenario");
        game.update_position();
        game.update_plane();
        let expected_pos = (10usize, 5usize);
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 - 1][expected_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 - 1][expected_pos.0 - 1]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1][expected_pos.0 - 1]
        );
    }

    #[test]
    fn test_should_not_change_to_opposite_direction_when_snake_second_node_is_on_the_same_axis() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.update_position();
        game.change_direction(Direction::Up)
            .expect("Directions should be ok in this scenario");
        game.update_position();
        game.change_direction(Direction::Right)
            .expect("Directions should be ok in this scenario");
        game.update_position();
        game.update_plane();
        let expected_pos = (10usize, 5usize);
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 - 1][expected_pos.0]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1 - 1][expected_pos.0 - 1]
        );
        assert_eq!(
            PlaneField::Body,
            game.plane[expected_pos.1][expected_pos.0 - 1]
        );
    }

    #[test]
    fn test_should_insert_apple_successfully_when_not_colliding_with_snake() {
        let mut game = SnakeContext::new();
        game.new_game();
        let result = game.insert_apple(Point(0, 0));
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn test_should_insert_apple_result_in_error_when_colliding_with_snake() {
        let mut game = SnakeContext::new();
        game.new_game();
        let initial_pos = Point(10, 5);
        let result = game.insert_apple(initial_pos);
        assert_eq!(AppleInsertError, result.unwrap_err());
    }

    #[test]
    fn test_should_insert_apple_result_in_error_when_colliding_with_other_apple() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.insert_apple(Point(0, 0))
            .expect("it should be ok in this case");
        let result = game.insert_apple(Point(0, 0));
        assert_eq!(AppleInsertError, result.unwrap_err());
    }

    #[test]
    fn test_should_append_snake_when_encounter_apple_left_case() {
        let mut game = SnakeContext::new();
        game.new_game();
        game.insert_apple(Point(9, 5))
            .expect("it should be ok in this case");
        game.update_position();
        game.update_plane();
        let initial_pos = Point(10, 5);
        assert_eq!(
            PlaneField::Body,
            game.plane[initial_pos.1 as usize][initial_pos.0 as usize + 2]
        );
    }
}

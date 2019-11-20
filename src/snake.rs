use std::collections::LinkedList;
use rand::Rng;/*::{
  distributions::{Distribution, Standard},
  Rng,
};*/

use super::tile::{Tile, random as random_tile};


#[derive(Copy, Clone)]
pub enum Direction {
  NORTH,
  SOUTH,
  EAST,
  WEST,
}

impl Direction {
  pub fn opposite(&self) -> Direction {
    match *self {
      Direction::NORTH => Direction::SOUTH,
      Direction::SOUTH => Direction::NORTH,
      Direction::EAST => Direction::WEST,
      Direction::WEST => Direction::EAST,
    }
  }
  pub fn x_cmp(&self) -> i8 {
    match *self {
      Direction::EAST => 1,
      Direction::WEST => -1,
      _ => 0,
    }
  }
  pub fn y_cmp(&self) -> i8 {
    match *self {
      Direction::NORTH => -1,
      Direction::SOUTH => 1,
      _ => 0,
    }
  }
}

/*impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut Rng) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::NORTH,
            1 => Direction::SOUTH,
            2 => Direction::EAST,
            3 => Direction::WEST,
        }
    }
}*/

pub struct Snake {
  pub direction: Direction,
  pub body: LinkedList<Tile>,
  pub tail: Option<Tile>,
}

impl Snake {
  pub fn new(t: Tile, d: Direction) -> Snake {
    let mut body: LinkedList<Tile> = LinkedList::new();
    body.push_back(t);
    Snake {
      direction: d,
      body: body,
      tail: None,
    }
  }
  /*pub fn new(x: u32, y: u32, d: Direction) -> Snake {
    let mut body = LinkedList::new();
    body.push_back(Tile {x, y})
  }*/
  pub fn random(grid_size: u32) -> Snake {
    return Snake::new(random_tile(grid_size - 2, grid_size - 2), match rand::thread_rng().gen_range(0, 4) {
      0 => Direction::SOUTH,
      1 => Direction::NORTH,
      2 => Direction::WEST,
      _ => Direction::EAST,
    });
  }
}

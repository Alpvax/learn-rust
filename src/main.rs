extern crate rand;

mod snake;
mod tile;

const GAME_SIZE: u32 = 20;
const TILE_SIZE: u32 = 1;

fn main() {
  let mut grid: [[char; (GAME_SIZE) as usize]; (GAME_SIZE) as usize] = [[' '; GAME_SIZE as usize]; GAME_SIZE as usize];
  let snake = snake::Snake::random(GAME_SIZE);
  for i in 0..GAME_SIZE {
    grid[i as usize][0] = 'x';
    grid[i as usize][(GAME_SIZE - 1) as usize] = 'x';
    grid[0][i as usize] = 'x';
    grid[(GAME_SIZE - 1) as usize][i as usize] = 'x';
  }
  for seg in snake.body.iter() {
    grid[seg.y as usize][seg.x as usize] = '#';
  }
  for row in grid.iter() {
    for _i in 0..TILE_SIZE {
      for col in row.iter() {
        for _j in 0..TILE_SIZE {
          print!("{}", col)
        }
      }
      println!("");
    }
  }
}

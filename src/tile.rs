use rand::Rng;

#[derive(Clone)]
pub struct Tile {
  pub x: u32,
  pub y: u32,
}

pub fn random(x_size: u32, y_size: u32) -> Tile {
  let mut rng = rand::thread_rng();
  let x = rng.gen_range(0, x_size);
  let y = rng.gen_range(0, y_size);
  Tile {
    x,
    y,
  }
}

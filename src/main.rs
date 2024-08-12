#![allow(dead_code)]

use std::{env, fs};

#[derive(Clone)]
struct Grid {
  vec:    Vec<bool>,
  width:  usize,
  height: usize,
}

impl Grid {
  // A live cell dies if it has fewer than two live neighbors.
  // A live cell with two or three live neighbors lives on to the next generation.
  // A live cell with more than three live neighbors dies.
  // A dead cell will be brought back to live if it has exactly three live neighbors.

  fn compute_cell_next_state(&self, x: usize, y: usize) -> bool {
    let mut counter = 0;

    for dy in -1..=1 {
      for dx in -1..=1 {
        if dx == 0 && dy == 0 {
          continue;
        }

        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
          let index = ny as usize * self.width + nx as usize;
          if self.vec.get(index) == Some(&true) {
            counter += 1;
          }
        }
      }
    }

    match (self.vec[x + (y * self.width)], counter) {
      (true, 2) | (_, 3) => return true,
      _ => return false,
    }
  }

  fn compute_next_gen(&mut self) {
    let mut i = 0;
    let mut new_vec: Vec<bool> = Vec::with_capacity(self.vec.len());
    for _ in 0..self.vec.len() {
      new_vec.push(false);
    }
    while i < self.vec.len() {
      let x = i % self.width;
      let y = i / self.width;

      new_vec[i] = self.compute_cell_next_state(x, y);

      i += 1;
    }
    self.vec = new_vec;
  }
}

fn display_grid(gen: &Grid) {
  let y = gen.vec.len() / &gen.width;
  let mut buffer = String::new();
  for i in 0..y {
    for j in 0..gen.width {
      if *gen.vec.get(j + (i * gen.width)).unwrap_or(&false) == true {
        buffer += " # ";
      } else {
        buffer += " . ";
      }
    }
    buffer += &format!("{}, {}", i * gen.width, i * gen.width + gen.width - 1).to_owned();
    buffer += "\n";
  }

  let width = gen.width * 3;
  println!("{:=^width$}", "");
  print!("{buffer}");
  println!("{:=^width$}", "");
}

fn main() {
  let mut arg = env::args();
  arg.next().unwrap();

  let size: (usize, usize);
  if arg.len() != 3 {
    panic!("please input the size you want the grid to be: aotomata <width> <hight>");
  }
  size = (
    arg.next().unwrap().parse().unwrap(),
    arg.next().unwrap().parse().unwrap(),
  );
  println!("size: {size:?}");
  let iterations: usize = arg.next().unwrap().parse().unwrap();
  println!("running {iterations:?} iteration.");

  let mut grid: Vec<bool> = Vec::with_capacity(size.0 * size.1);
  let input = fs::read_to_string("input").unwrap();
  let mut input = input.bytes();
  for _ in 0..(size.0 * size.1) {
    grid.push(input.next().unwrap_or(46) == 35);
  }

  let mut gen = Grid {
    vec:    grid,
    width:  size.0,
    height: size.1,
  };
  display_grid(&gen);
  for i in 0..iterations {
    gen.compute_next_gen();
    println!("gen: {:0>2}", i + 1);
    display_grid(&gen);
  }
}

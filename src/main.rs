#![allow(dead_code)]

use std::{env, fs};

struct Grid {
  vec: Vec<bool>,
  x:   usize,
}

impl Grid {
  // A live cell dies if it has fewer than two live neighbors.
  // A live cell with two or three live neighbors lives on to the next generation.
  // A live cell with more than three live neighbors dies.
  // A dead cell will be brought back to live if it has exactly three live neighbors.

  fn compute_cell_state(cell: Vec<bool>) -> bool {
    let _ = cell;
    true
  }

  fn compute_next_gen(&mut self) -> Self {
    let new_vec: Vec<bool> = Vec::with_capacity(self.vec.len());
    for (i, cell) in self.vec.iter().enumerate() {
      // println!("{cell}");
      let mut cell_vec:[bool; 8];
      
      
      
    }
    Grid { vec:new_vec, x:self.x }
  }

}

fn display_grid(gen: &Grid) {
  let y = gen.vec.len() / &gen.x;
  let mut buffer = String::new();
  for i in 0..y {
    for j in 0..gen.x {
      if *gen.vec.get(j + (i * gen.x)).unwrap_or_else(|| &false) == true {
        buffer += " # ";
      } else {
        buffer += " . ";
      }
    }
    buffer += "\n";
  }

  let width = gen.x * 3;
  println!("{:=^width$}", "");
  print!("{buffer}");
  println!("{:=^width$}", "");
}

fn main() {
  let mut arg = env::args();
  arg.next().unwrap();

  let size: (usize, usize);
  if arg.len() != 2 {
    panic!("please input the size you want the grid to be: aotomata <width> <hight>");
  }
  size = (
    arg.next().unwrap().parse().unwrap(),
    arg.next().unwrap().parse().unwrap(),
  );
  println!("size: {size:?}");

  let mut grid: Vec<bool> = Vec::with_capacity(size.0 * size.1);
  let input = fs::read_to_string("input").unwrap();
  for i in 0..size.0 * size.1 {
    grid.push(input.get(i..i + 1).unwrap_or(&".") == "#");
  }
  // println!("{grid:?}");

  let mut gen = Grid {
    vec: grid,
    x:   size.0,
  };
  gen.vec[5] = true;
  // compute_next_gen(gen);
  // println!("{:=^50}", "");
  display_grid(&gen);
  gen.compute_next_gen();
}

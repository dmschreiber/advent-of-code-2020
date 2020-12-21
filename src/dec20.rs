#[cfg(test)]
mod tests {

  #[test]
  pub fn dec20_prod() {
    assert!(30425930368573==super::solve_part1("./inputs/dec20.txt".to_string()));
    super::solve_part2("./inputs/dec20.txt".to_string());

  }
  #[test]
  pub fn dec20_test() {
      assert!(20899048083289==super::solve_part1("./inputs/dec20-test.txt".to_string()));
      super::solve_part2("./inputs/dec20-test.txt".to_string());
  }  
}

use std::fs;
use regex::Regex;
use std::time::{Instant};
use std::collections::HashMap;

lazy_static! {
  static ref TILE_ID_REGEX: Regex = Regex::new(r"^Tile (\d+):$").unwrap();

}

#[derive(Debug,Clone,PartialEq)]
pub struct Tile {
  id : u32,
  lines : Vec<String>,
  sides : Vec<u32>,
  rev_sides : Vec<u32>,
}

impl Tile {
  fn all_rotations(&self) -> Vec<Vec<u32>> {
    let mut v = Vec::new();

    v.push(vec![self.sides[0],self.sides[1],self.sides[2],self.sides[3]]);
    v.push(vec![self.rev_sides[3],self.sides[0],self.rev_sides[1],self.sides[2]]);
    v.push(vec![self.rev_sides[2],self.rev_sides[3],self.rev_sides[0], self.rev_sides[1]]);
    v.push(vec![self.sides[1],self.rev_sides[2],self.sides[3], self.rev_sides[0]]);

    v.push(vec![self.sides[0],self.sides[3],self.sides[2],self.sides[1]]);
    v.push(vec![self.rev_sides[1],self.sides[0],self.rev_sides[3],self.sides[2]]);
    v.push(vec![self.rev_sides[2],self.rev_sides[1],self.rev_sides[0], self.rev_sides[3]]);
    v.push(vec![self.sides[3],self.rev_sides[2],self.sides[1], self.rev_sides[0]]);

    v.push(vec![self.sides[2],self.rev_sides[1],self.sides[0],self.rev_sides[3]]);
    v.push(vec![self.sides[3],self.sides[2],self.sides[1],self.sides[0]]);
    v.push(vec![self.rev_sides[0],self.sides[3],self.rev_sides[2], self.sides[1]]);
    v.push(vec![self.rev_sides[1],self.rev_sides[0],self.rev_sides[3], self.rev_sides[2]]);

    v
  }
}

pub fn make_tile(lines : Vec<String>) -> Tile {
  let mut left_string = "".to_string();
  let mut right_string = "".to_string();

  let mut tile_id= 0;
  let mut top : u32 = 0;
  let mut top_rev : u32 = 0;
  let bottom : u32;
  let bottom_rev : u32;
  let left : u32;
  let left_rev : u32;
  let right : u32;
  let right_rev : u32;

  for (index,line) in lines.iter().enumerate() {
    if line.len() == 0 { break; }

    if let Some(args) = TILE_ID_REGEX.captures(line) {
      tile_id = args[1].parse::<u32>().unwrap();
    } else {
      if index == 1 {
        top = u32::from_str_radix(&line.replace("#","1").replace(".","0"),2).unwrap();
        top_rev = u32::from_str_radix(&line.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        left_string = line[..1].to_string();
        right_string = line[9..=9].to_string();
      } else if index == 10 {
        bottom = u32::from_str_radix(&line.replace("#","1").replace(".","0"),2).unwrap();
        bottom_rev = u32::from_str_radix(&line.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        left_string = format!("{}{}", left_string, &line[..1].to_string());
        right_string = format!("{}{}", right_string.clone(), &line[9..=9].to_string());

        left = u32::from_str_radix(&left_string.replace("#","1").replace(".","0"),2).unwrap();
        left_rev = u32::from_str_radix(&left_string.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        right = u32::from_str_radix(&right_string.replace("#","1").replace(".","0"),2).unwrap();
        right_rev = u32::from_str_radix(&right_string.replace("#","1").replace(".","0").chars().rev().collect::<String>(),2).unwrap();

        return Tile{ id : tile_id, 
            lines: lines[1..].to_vec(),
            sides : vec![top, right, bottom, left] , 
            rev_sides : vec![top_rev, right_rev, bottom_rev, left_rev] };

      } else {
        left_string = format!("{}{}", left_string, &line[..1].to_string());
        right_string = format!("{}{}", right_string.clone(), &line[9..=9].to_string());

      }
    }
  }
  panic!("never returned a tile");
}


pub fn solve_part1(filename : String) -> u64 {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  
    let tiles : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
    
    let mut things = Vec::new();

    for tile in tiles {
      // println!("{}", tile);
      let lines : Vec<String> = tile.lines().map(|s| (&*s).to_string()).collect();
      things.push(make_tile(lines));
    }

    // println!("{:?}", things);
    let start = Instant::now();

    let mut corners = std::collections::HashMap::<u32,Tile>::new();
    let mut borders = std::collections::HashMap::<u32,Tile>::new();
    let mut inner = std::collections::HashMap::<u32,Tile>::new();
    for t in &things {
      // println!("Trying {}", t.id);
      for (_r_index,rotation) in t.all_rotations().iter().enumerate() {
        let mut unique_sides = 0;
        for (_index,s) in rotation.iter().enumerate() {
          let c = count_matches(&things,t.id,*s);
          // println!("Tile {} rotation {:?} {} side index {} with value {} is {} others", t.id, rotation, _r_index, _index, *s, c);
          if c == 0 {
            unique_sides += 1;
          }
        }
        if unique_sides == 2 {
          // println!("{:?} rotation {:?} has {} unique sides", t, rotation, unique_sides);
          corners.insert(t.id, t.clone());
        } else if unique_sides == 1 {
          borders.insert(t.id, t.clone());
        } else {
          inner.insert(t.id, t.clone());
        }
      }
    }
    println!("{} Border tiles {}", borders.len(), borders.keys().map(|id| format!("{}",id) ).collect::<Vec<String>>().join(","));
    let mut retval : u64 = 1;
    for c in corners.values() {
      println!("{}", c.id );
      retval = retval * c.id as u64;
    }
    println!("{}={} in {:?}", corners.keys().map(|id| format!("{}",id) ).collect::<Vec<String>>().join("x"), retval, start.elapsed());
    retval
}

pub fn build_border(corners : &Vec<Tile>, borders : &Vec<Tile>, WIDTH : u32) -> HashMap<(u32,u32),Tile> {
  let mut retval = std::collections::HashMap::<(u32,u32),Tile>::new();
  let mut borders_vec : Vec<Tile> = borders.clone(); // borders.values().cloned().collect();
  let mut corners_vec : Vec<Tile> = corners.clone(); // values().cloned().collect();

  // let WIDTH = 3;

  // println!("borders {:?}", borders_vec);

  if WIDTH == 2 {
    let t = corners_vec[0].clone();
    let index = corners_vec.iter().position(|x| *x == t).unwrap();
    corners_vec.remove(index);
    retval.insert((0,0),t.clone());

    for rotation_sides in t.all_rotations() {
      let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
      if v == vec![0,1,1,0] {

        let t1 = get_match(&corners_vec, t.id, rotation_sides[1]).unwrap().clone();
        let index = corners_vec.iter().position(|x| *x == t1).unwrap();
        corners_vec.remove(index);
        retval.insert((0,1),t1);

        let t2 = get_match(&corners_vec, t.id, rotation_sides[2]).unwrap().clone();
        let index = corners_vec.iter().position(|x| *x == t2).unwrap();
        corners_vec.remove(index);
        retval.insert((1,0),t2);

        break;
      }
    }

    retval.insert((1,1),corners_vec[0].clone());

    return retval;
  }

  for row in 0..WIDTH {
    for col in 0..WIDTH {
      // println!("{} {}", row, col);
      if row == 0 && col == 0 {
        let t = corners_vec[0].clone();
        let index = corners_vec.iter().position(|x| *x == t).unwrap();
        corners_vec.remove(index);

        let t_id = t.id;
        // println!("Tile: {}", t_id);
        retval.insert((row,col),t.clone());
        for rotation_sides in t.all_rotations() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();

          if v == vec![0,1,1,0] {
            let t1 = get_match(&borders_vec, t_id, rotation_sides[1]).unwrap().clone();
            let index = borders_vec.iter().position(|x| *x == t1).unwrap();
            borders_vec.remove(index);
            retval.insert((0,1),t1);

            let t2 = get_match(&borders_vec, t_id, rotation_sides[2]).unwrap().clone();
            let index = borders_vec.iter().position(|x| *x == t2).unwrap();
            borders_vec.remove(index);
            retval.insert((1,0),t2);
    
            break;
          }
        }

      } else if row == 0 && col == WIDTH-2 {
        let t = retval.get(&(row,col)).unwrap().clone();
        for rotation_sides in t.all_rotations() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
          if v == vec![0,1,0,0] {

            let t1 = get_match(&corners_vec, t.id, rotation_sides[1]).unwrap().clone();
            let index = corners_vec.iter().position(|x| *x == t1).unwrap();
            corners_vec.remove(index);
            retval.insert((row,col+1),t1);

            break;
          }
        }

      } else if row == 0 && col == WIDTH-1 {
        let t = retval.get(&(row,col)).unwrap().clone();
        for rotation_sides in t.all_rotations() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
          if v == vec![0,0,1,0] {

            let t1 = get_match(&borders_vec, t.id, rotation_sides[2]).unwrap().clone();
            let index = borders_vec.iter().position(|x| *x == t1).unwrap();
            borders_vec.remove(index);
            retval.insert((row+1,col),t1);

            break;
          }
        }

      } else if row == WIDTH-2 && col == 0 {
        let t = retval.get(&(row,col)).unwrap().clone();
        for rotation_sides in t.all_rotations() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
          if v == vec![0,0,1,0] {

            let t1 = get_match(&corners_vec, t.id, rotation_sides[2]).unwrap().clone();
            let index = corners_vec.iter().position(|x| *x == t1).unwrap();
            corners_vec.remove(index);
            retval.insert((row+1,col),t1);

            break;
          }
        }

      } else if row == WIDTH-1 && col == 0 { // LL corner
        let t = retval.get(&(row,col)).unwrap().clone();
        for rotation_sides in t.all_rotations() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
          if v == vec![0,1,0,0] {

            let t1 = get_match(&borders_vec, t.id, rotation_sides[1]).unwrap().clone();
            let index = borders_vec.iter().position(|x| *x == t1).unwrap();
            borders_vec.remove(index);
            retval.insert((row,col+1),t1);

            break;
          }
        }

      } else if row == WIDTH-1 && col == WIDTH-2 { // second from lower right corner
        let t = retval.get(&(row,col)).unwrap().clone();
        for rotation_sides in t.all_rotations() {
          let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&corners_vec, t.id, *s)).collect();
          if v == vec![0,1,0,0] {

            let t1 = get_match(&corners_vec, t.id, rotation_sides[1]).unwrap().clone();
            let index = corners_vec.iter().position(|x| *x == t1).unwrap();
            corners_vec.remove(index);
            retval.insert((row,col+1),t1);

            break;
          }
        }

      } else if col == 0 || col == WIDTH-1 {
        let t = retval.get(&(row,col)).unwrap();
        if retval.get(&(row+1,col)) == None {
          for rotation_sides in t.all_rotations() {
            let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
            if v == vec![0,0,1,0] {
  
              let t1 = get_match(&borders_vec, t.id, rotation_sides[2]).unwrap().clone();
              let index = borders_vec.iter().position(|x| *x == t1).unwrap();
              borders_vec.remove(index);
              retval.insert((row+1,col),t1);
  
              break;
            }
          }
  
        }
      }  else if col > 0 && row > 0 && col < WIDTH-1 && row < WIDTH-1 {
        // println!("NO LOGIC - INSIDE");
        // retval.get(&(row,col)).unwrap();

      } else if row == 0 || row == WIDTH-1 {
        let t = retval.get(&(row,col)).unwrap();
        if retval.get(&(row,col+1)) == None {
          for rotation_sides in t.all_rotations() {
            let v : Vec<u32> = rotation_sides.iter().map(|s| count_matches(&borders_vec, t.id, *s)).collect();
            if v == vec![0,1,0,0] {
  
              let t1 = get_match(&borders_vec, t.id, rotation_sides[1]).unwrap().clone();
              let index = borders_vec.iter().position(|x| *x == t1).unwrap();
              borders_vec.remove(index);
              retval.insert((row,col+1),t1);
  
              break;
            }
          }
  
        }

      } else {
        println!("SHOULD BE DOING SOMETHING");
      }
    }     
  }

  retval
}

pub fn check_fit(t : Tile, pattern : Vec<u32>) -> bool {

  for rotation_sides in t.all_rotations() {
    let mut pattern_match = true;
    for (index,s) in rotation_sides.iter().enumerate() {
      if pattern[index] > 0 {
        pattern_match = pattern_match && pattern[index] == *s;
      }
    }
    if pattern_match { return pattern_match; }
  }
  
  return false;

}

pub fn count_matches(things : &Vec<Tile>, skip_id : u32, side : u32) -> u32 {
  let mut retval = 0;

  for t in things {
    if t.id != skip_id {
      for rotation_sides in t.all_rotations() {
        let mut side_count = 0;
        for s in rotation_sides {
          if s == side { side_count += 1; }
        }
        retval = std::cmp::max(retval,side_count);
      }
    }
  }
  
  retval
}

pub fn get_match(things : &Vec<Tile>, skip_id : u32, side : u32) -> Option<Tile> {

  for t in things {
  if t.id != skip_id {
    for rotation_sides in t.all_rotations() {
      for s in rotation_sides {
        if s == side {
          return Some(t.clone());
        }
      }
    }
  }
}
None
}

pub fn border (things : &Vec<Tile>, corner_hint : Option<u32>) -> HashMap<(u32,u32),Tile> {
  let mut corners = vec![]; // std::collections::HashMap::<u32,Tile>::new();
  let mut borders = vec![]; // std::collections::HashMap::<u32,Tile>::new();
  let mut inner = vec![];

  for t in things {
    for (_r_index,rotation) in t.all_rotations().iter().enumerate() {
      let mut unique_sides = 0;
      for (_index,s) in rotation.iter().enumerate() {
        let c = count_matches(&things,t.id,*s);
        if c == 0 {
          unique_sides += 1;
        }
      }
      if unique_sides == 2 {
        if let Some(id) = corner_hint {
          if id == t.id {
            corners.insert(0,t.clone());
          } else {
            corners.push(t.clone());
          }
        } else {
          corners.push(t.clone());
        }
        corners.dedup();
      } else if unique_sides == 1 {
        borders.push(t.clone());
        borders.dedup();
      } else {
        inner.push(t.clone());
        inner.dedup();
      }
    }
  }
  // println!("{} Border tiles {}", borders.len(), borders.keys().map(|id| format!("{}",id) ).collect::<Vec<String>>().join(","));
  println!("{} Tiles", things.len());

  let size = (things.len() as f64).sqrt() as u32;
  let mut grid = build_border(&corners, &borders, size);
  let mut next_corner_hint = None;

  if size > 2 {
    let mut upper_left_corner = vec![];
    upper_left_corner.push(grid.get(&(0,1)).unwrap().clone());
    upper_left_corner.push(grid.get(&(1,0)).unwrap().clone());

    for t in &inner {
      for (_r_index,rotation) in t.all_rotations().iter().enumerate() {
        let mut unique_matches = 0;
        for (_index,s) in rotation.iter().enumerate() {
          let c = count_matches(&upper_left_corner, t.id, *s);
          if c == 1 {
            unique_matches += 1;
          }
        }
        if unique_matches == 2 {
            next_corner_hint = Some(t.id);
            break;
        }
      }
    }
  }

  if inner.len() > 1 {
    let sub_grid = border(&inner,next_corner_hint);
    for row in 0..size-2 {
      for col in 0..size-2 {
        // println!("Getting {} {} from subgrid", row, col);
        if let Some(t) = sub_grid.get(&(row,col)) {
          grid.insert((row+1,col+1),t.clone());

        }
      }
    }
    grid
  } else {
    grid
  }
}

pub fn solve_part2(filename : String) {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let tiles : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  
  let mut things = Vec::new();
  let size = (tiles.len() as f64).sqrt() as u32;

  for tile in tiles {
    let lines : Vec<String> = tile.lines().map(|s| (&*s).to_string()).collect();
    things.push(make_tile(lines));
  }


  let grid = border(&things,None);

  for row in 0..size {
    for col in 0..size {
      if let Some(t) = grid.get(&(row,col)) {
        print!(" {:4} ", t.id);
      } else {
        print!(" ???? ");
      }
    }
    println!();
  }

}
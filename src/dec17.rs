#[cfg(test)]
mod tests {

  #[test]
  pub fn dec17_prod() {
      assert!(213==super::solve("./inputs/dec17.txt".to_string()));
      super::solve_part2("./inputs/dec17.txt".to_string());
  }
  #[test]
  pub fn dec17_test() {
      assert!(112==super::solve("./inputs/dec17-test.txt".to_string()));
      assert!(848==super::solve_part2("./inputs/dec17-test.txt".to_string()));
  }
}

use std::fs;

#[derive(Debug,Clone)]
pub struct ConwayMap {
  map : std::collections::HashMap<(i32,i32,i32),u8>,
}

#[derive(Debug,Clone)]
pub struct ConwayMap4D {
  map : std::collections::HashMap<(i32,i32,i32,i32),u8>,
}

pub fn how_many_neighbors(map : &ConwayMap, position : (i32,i32,i32)) -> i32 {
  let mut retval = 0;

  for z_diff in [-1,0,1].iter() {
    for r_diff in [-1,0,1].iter() {
      for c_diff in [-1,0,1].iter() {
        if !(*z_diff == 0 && *r_diff == 0 && *c_diff == 0) {
          if let Some(v) = map.map.get(&(position.0+*r_diff,position.1+*c_diff,position.2+*z_diff)) {
            // println!("found {} at {:?}", *v, (position.0+*r_diff,position.1+*c_diff,position.2+*z_diff));
            if *v == b'#' {
              retval += 1;
            }
          }
        }
    }
  }
}
// println!("returning {:?} - {}", position, retval);

  retval
}
pub fn get_max(map : &std::collections::HashMap<(i32,i32,i32),u8>) -> (i32,i32,i32) {

  let max_col = map.keys().map(|(_a,b,_c)| *b ).max().unwrap();
  let max_row = map.keys().map(|(a,_b, _c)| *a ).max().unwrap();
  let max_z = map.keys().map(|(_a,_b,c) | *c ).max().unwrap();

  (max_row,max_col,max_z)
}

pub fn get_min(map : &std::collections::HashMap<(i32,i32,i32),u8>) -> (i32,i32,i32) {
  let min_col = map.keys().map(|(_a,b,_c)| *b ).min().unwrap();
  let min_row = map.keys().map(|(a,_b, _c)| *a ).min().unwrap();
  let min_z = map.keys().map(|(_a,_b,c) | *c ).min().unwrap();

  (min_row,min_col,min_z)
}

#[allow(dead_code)]
pub fn print(c_map : &ConwayMap) {
  let maxes = get_max(&c_map.map);
  let mins = get_min(&c_map.map);

  for z in mins.2-1..=maxes.2+1 {
    println!("z={}",z);
    for r in mins.0-1..=maxes.0+1 {
      for c in mins.1-1..maxes.1+1 {
        if let Some(v) = c_map.map.get(&(r,c,z)) {
          print!("{}",*v as char)
        } else {
          print!(".")
        }
      }
      println!();
    }
  }

}

pub fn solve(filename : String) -> i64 {
  let mut retval;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();
  // let mut map = std::collections::HashMap::new();

  let mut c_map = ConwayMap{ map: std::collections::HashMap::new() };

  let mut row : i32 = 0;
  let mut col : i32 = 0;
  let z : i32 = 0;

  for line in &lines {
    for b in line.as_bytes() {
      c_map.map.insert((row,col,z),*b);
      col +=1;
    }
    row += 1;
    col = 0;
  }

  for _index in 0..6 {
  // println!("==> ITeration {:?}", index);
  let maxes = get_max(&c_map.map);
  let mins = get_min(&c_map.map);
  let old_map = c_map.clone();

  for z in mins.2-2..=maxes.2+2 {
    for r in mins.0-2..=maxes.0+2 {
      for c in mins.1-2..maxes.1+2 {
        // println!("{} {} {}", r, c, z);
        let n = how_many_neighbors(&old_map, (r,c,z));
        match old_map.map.get(&(r,c,z)) {
          Some(v) => {
            if *v == b'#' && n >= 2 && n <= 3 {
              // println!("stays");
            } else if *v == b'.' && n == 3 {
              c_map.map.entry((r,c,z)).and_modify(|n| *n=b'#');
            } else {
              c_map.map.entry((r,c,z)).and_modify(|n| *n=b'.');

            }
          }
          None => { 
            if n == 3 { 
              c_map.map.insert((r,c,z),b'#');
            } 
          }
        }
      }
    }
  }
  }

  let maxes = get_max(&c_map.map);
  let mins = get_min(&c_map.map);

  retval = 0;
  for z in mins.2-1..=maxes.2+1 {
    for r in mins.0-1..=maxes.0+1 {
      for c in mins.1-1..maxes.1+1 {
        if let Some(v) = c_map.map.get(&(r,c,z)) {
          if *v == b'#' {
            retval += 1;
          }
        }
      }
    }
  }
  // println!("retval {}", retval);
  retval
}


pub fn how_many_neighbors_4d(map : &ConwayMap4D, position : (i32,i32,i32,i32)) -> i32 {
  let mut retval = 0;
  for w_diff in [-1,0,1].iter() {
    for z_diff in [-1,0,1].iter() {
      for r_diff in [-1,0,1].iter() {
        for c_diff in [-1,0,1].iter() {
          if !(*z_diff == 0 && *r_diff == 0 && *c_diff == 0 && *w_diff == 0) {
            if let Some(v) = map.map.get(&(position.0+*r_diff,position.1+*c_diff,position.2+*z_diff,position.3+w_diff)) {
              if *v == b'#' {
                retval += 1;
              }
            }
          }
      }
    }
  }
}
  retval
}

#[allow(dead_code)]
pub fn print_4d(c_map : &ConwayMap4D) {
  let max_col = c_map.map.keys().map(|(_a,b,_c, _d)| *b ).max().unwrap();
  let max_row = c_map.map.keys().map(|(a,_b, _c, _d)| *a ).max().unwrap();
  let max_z = c_map.map.keys().map(|(_a,_b,c, _d) | *c ).max().unwrap();
  let max_w = c_map.map.keys().map(|(_a,_b,_c, d) | *d ).max().unwrap();

  let min_col = c_map.map.keys().map(|(_a,b,_c, _d)| *b ).min().unwrap();
  let min_row = c_map.map.keys().map(|(a,_b, _c, _d)| *a ).min().unwrap();
  let min_z = c_map.map.keys().map(|(_a,_b,c, _d) | *c ).min().unwrap();
  let min_w = c_map.map.keys().map(|(_a,_b,_c, d) | *d ).min().unwrap();

  for w in min_w..max_w+1 {
    for z in min_z-1..=max_z+1 {
      println!("z={}, w={}",z,w);
      for r in min_row-1..=max_row+1 {
        for c in min_col-1..max_col+1 {
          if let Some(v) = c_map.map.get(&(r,c,z,w)) {
            print!("{}",*v as char)
          } else {
            print!(".")
          }
        }
        println!();
      }
    }
  }
}


pub fn solve_part2(filename : String) -> i64 {
  let mut retval;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();
  // let mut map = std::collections::HashMap::new();

  let mut c_map = ConwayMap4D{ map: std::collections::HashMap::new() };

  let mut row : i32 = 0;
  let mut col : i32 = 0;
  let z : i32 = 0;
  let w : i32 = 0;

  for line in &lines {
    for b in line.as_bytes() {
      c_map.map.insert((row,col,z,w),*b);
      col +=1;
    }
    row += 1;
    col = 0;
  }

  for _index in 0..6 {

    let max_col = c_map.map.keys().map(|(_a,b,_c, _d)| *b ).max().unwrap();
    let max_row = c_map.map.keys().map(|(a,_b, _c, _d)| *a ).max().unwrap();
    let max_z = c_map.map.keys().map(|(_a,_b,c, _d) | *c ).max().unwrap();
    let max_w = c_map.map.keys().map(|(_a,_b,_c, d) | *d ).max().unwrap();

    let min_col = c_map.map.keys().map(|(_a,b,_c, _d)| *b ).min().unwrap();
    let min_row = c_map.map.keys().map(|(a,_b, _c, _d)| *a ).min().unwrap();
    let min_z = c_map.map.keys().map(|(_a,_b,c, _d) | *c ).min().unwrap();
    let min_w = c_map.map.keys().map(|(_a,_b,_c, d) | *d ).min().unwrap();

    let old_map = c_map.clone();

    for w in min_w-2..=max_w+2 {
      for z in min_z-2..=max_z+2 {
        for r in min_row-2..=max_row+2 {
          for c in min_col-2..max_col+2 {
            // println!("{} {} {}", r, c, z);
            let n = how_many_neighbors_4d(&old_map, (r,c,z,w));
            match old_map.map.get(&(r,c,z,w)) {
              Some(v) => {
                if *v == b'#' && n >= 2 && n <= 3 {
                  // println!("stays");
                } else if *v == b'.' && n == 3 {
                  c_map.map.entry((r,c,z,w)).and_modify(|n| *n=b'#');
                } else {
                  c_map.map.entry((r,c,z,w)).and_modify(|n| *n=b'.');

                }
              }
              None => { 
                if n == 3 { 
                  c_map.map.insert((r,c,z,w),b'#');
                } 
              }
            }
          }
        }
      }
    }
    // println!("==> ITeration {:?}", index);
    // print_4d(&c_map);

  }

  let max_col = c_map.map.keys().map(|(_a,b,_c, _d)| *b ).max().unwrap();
  let max_row = c_map.map.keys().map(|(a,_b, _c, _d)| *a ).max().unwrap();
  let max_z = c_map.map.keys().map(|(_a,_b,c, _d) | *c ).max().unwrap();
  let max_w = c_map.map.keys().map(|(_a,_b,_c, d) | *d ).max().unwrap();

  let min_col = c_map.map.keys().map(|(_a,b,_c, _d)| *b ).min().unwrap();
  let min_row = c_map.map.keys().map(|(a,_b, _c, _d)| *a ).min().unwrap();
  let min_z = c_map.map.keys().map(|(_a,_b,c, _d) | *c ).min().unwrap();
  let min_w = c_map.map.keys().map(|(_a,_b,_c, d) | *d ).min().unwrap();

  // println!("{}", how_many_neighbors(&old_map, (1,0,-1)));
  // return -1;
  retval = 0;
  for w in min_w-3..=max_w+3 {
    for z in min_z-3..=max_z+3 {
      for r in min_row-3..=max_row+3 {
        for c in min_col-3..max_col+3 {
          if let Some(v) = c_map.map.get(&(r,c,z,w)) {
            if *v == b'#' {
              retval += 1;
            }
          }
        }
      }
    }
  }
  // println!("retval {}", retval);
  retval
}

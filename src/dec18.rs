use std::collections::HashMap;
use std::fs;

pub fn display_map(map: HashMap<(u32,u32),u8>) {
  let mut max_row = 0;
  let mut max_col = 0;

  for k in map.keys() {
    let (a,b) = k;
    if *a > max_row { max_row = *a; }
    if *b > max_col { max_col = *b; }
  }
  for row in 0..=max_row {
    for col in 0..=max_col {
      let b = map.get(&(row,col));
      if let Some(i) = b {
        print!("{}", *i as char);
      }
    }
    println!();
  }
}

pub fn display_map_with_keys(map: HashMap<(u32,u32),u8>, keys: HashMap<char,(u32,u32)>, doors: HashMap<char,(u32,u32)>) {
  let mut max_row = 0;
  let mut max_col = 0;

  for k in map.keys() {
    let (a,b) = k;
    if *a > max_row { max_row = *a; }
    if *b > max_col { max_col = *b; }
  }
  for row in 0..=max_row {
    for col in 0..=max_col {
      let b = map.get(&(row,col));
      if let Some(i) = b {
        if *i == b'.' {
          for k in keys.values() {
            if (row,col) == *k { print!("X"); }
          }           
        }
        print!("{}", *i as char);
      }
    }
    println!();
  }
}

pub fn read_map(filename: String) -> HashMap<(u32,u32),u8> {
  let mut map = HashMap::new();
  let mut keys = HashMap::new();
  let mut doors = HashMap::new();

  let mut col = 0;
  let mut row = 0;

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");
  
  let bytes = contents.as_bytes();
  for &item in bytes {
      if item == 10 {
        row += 1;
        col = 0;
      } else if "abcdefghijklmnopqrstuvwxyz".contains(item as char)
      {
        println!("new key");
        map.entry((row,col)).or_insert(b'.');
        keys.entry(item as char).or_insert((row,col));
        col += 1;
      } else if "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(item as char)
      {
        println!("new key");
        map.entry((row,col)).or_insert(b'.');
        doors.entry(item as char).or_insert((row,col));
        col += 1;
      } else {
        map.entry((row,col)).or_insert(item);
        col += 1;
      }
  }

  map
}  


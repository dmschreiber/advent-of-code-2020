use std::collections::HashMap;
use std::cmp;

fn is_seat_empty(map: &HashMap<(i32,i32),u8>, row: i32, col: i32) -> bool {
  if map.get(&(row,col)) == None || *map.get(&(row,col)).unwrap() == b'L' || *map.get(&(row,col)).unwrap() == b'.' {
    // println!("found one mpty {} {}", row, col);

    return true;
  }
  else {
    // println!("found one full {} {}", row, col);
    return false;
  }

}

fn is_empty_seat(map: &HashMap<(i32,i32),u8>, row: i32, col: i32) -> bool {
  if map.get(&(row,col)) != None && *map.get(&(row,col)).unwrap() == b'L' {
    return true;
  }
  else {
    return false;
  }

}

fn is_seat_full(map: &HashMap<(i32,i32),u8>, row: i32, col: i32) -> i32 {
  if map.get(&(row,col)) != None && *map.get(&(row,col)).unwrap() == b'#' {
    return 1;
  }
  else {
    let t = map.get(&(row,col));
    // println!("found {:?} vs {} at {} {}", t, b'#', row, col);
    return 0;
  }

}

fn check_adjacent_seats(map: &HashMap<(i32,i32),u8>, row: i32, col: i32) -> bool {

  if is_seat_empty(map,row+1,col) && is_seat_empty(map,row,col+1) && is_seat_empty(map, row+1, col+1) &&
    is_seat_empty(map,row-1,col) && is_seat_empty(map, row, col-1) && is_seat_empty(map, row-1, col-1) &&
    is_seat_empty(map, row-1,col+1) && is_seat_empty(map, row+1,col-1)  {
      // println!("found all empty");
      return true;
  } else {
    return false;
  }

}

fn count_adjacent_seats(map: &HashMap<(i32,i32),u8>, row: i32, col: i32) -> i32 {
  let count = is_seat_full(map,row+1,col) + is_seat_full(map,row,col+1) + is_seat_full(map, row+1, col+1) +
    is_seat_full(map,row-1,col) + is_seat_full(map, row, col-1) + is_seat_full(map, row-1, col-1) +
    is_seat_full(map, row-1,col+1) + is_seat_full(map, row+1,col-1);
    // println!("checking adjuacent {} {} {}" ,row, col,count);
    return count;

}

pub struct SeatMap {
  map: HashMap<(i32,i32),u8>,
  max_row: i32,
  max_col: i32,
}

fn count_visible_seats(seat_map: &SeatMap, row: i32, col: i32, short_cut: bool) -> i32 {

  let mut visible_count = 0;

  for n in row+1..=seat_map.max_row {
    let visible = is_seat_full(&seat_map.map,n,col);
    if visible > 0 {
      visible_count += visible;
      break;
    } else if is_empty_seat(&seat_map.map,n,col) {
      break;
    }
  }

  if short_cut && visible_count > 0 {
    return visible_count;
  }

  for n in (0..=row-1).rev() {
    let visible = is_seat_full(&seat_map.map,n,col);
    if visible > 0 {
      visible_count += visible;
      break;
    } else if is_empty_seat(&seat_map.map,n,col) {
      break;
    }
  }
  if short_cut && visible_count > 0 {
    return visible_count;
  }

  for m in col+1..=seat_map.max_col {
    let visible = is_seat_full(&seat_map.map,row,m);
    if visible > 0 {
      visible_count += visible;
      break;
    } else if is_empty_seat(&seat_map.map,row,m) {
      break;
    }
  }
  if short_cut && visible_count > 0 {
    return visible_count;
  }

  for m in (0..=col-1).rev() {
    let visible = is_seat_full(&seat_map.map,row,m);
    if visible > 0 {
      visible_count += visible;
      break;
    } else if is_empty_seat(&seat_map.map, row,m) {
      break;
    }
  }
  if short_cut && visible_count > 0 {
    return visible_count;
  }

  let i_max = cmp::max(&seat_map.max_row-row-1,&seat_map.max_col-col-1);
  for i in 1..=i_max {
      let visible = is_seat_full(&seat_map.map,row+i,col+i);
      if visible > 0 {
        visible_count += visible;
        break;
      } else if is_empty_seat(&seat_map.map,row+i,col+i) {
        break;
      }
  }
  if short_cut && visible_count > 0 {
    return visible_count;
  }

  let i_max = cmp::max(row,col);
  for i in 1..=i_max {
      let visible = is_seat_full(&seat_map.map,row-i,col-i);
      if visible > 0 {
        visible_count += visible;
        break;
      } else if is_empty_seat(&seat_map.map,row-i,col-i) {
        break;
      }
    
  }
  if short_cut && visible_count > 0 {
    return visible_count;
  }

  let i_max = cmp::max(row,&seat_map.max_col-col-1);
  for i in 1..=i_max {
      let visible = is_seat_full(&seat_map.map,row-i,col+i);
      if visible > 0 {
        visible_count += visible;
        break;
      } else if is_empty_seat(&seat_map.map,row-i,col+i) {
        break;
      }
  }
  if short_cut && visible_count > 0 {
    return visible_count;
  }


  let i_max = cmp::max(&seat_map.max_row-row-1,col);
  for i in 1..=i_max {
      let visible = is_seat_full(&seat_map.map,row+i,col-i);
      if visible > 0 {
        visible_count += visible;
        break;
      } else if is_empty_seat(&seat_map.map,row+i,col-i) {
        break;
      }
    
  }

  return visible_count;
}

fn print_map(map: &mut HashMap<(i32,i32),u8>) {


  let max_col = *map.keys().map(|(a,b)| b ).max().unwrap();
  let max_row = *map.keys().map(|(a,b)| a ).max().unwrap();


  for row in 0..=max_row {
    for col in 0..=max_col {
      print!("{}",*map.get(&(row,col)).unwrap() as char);

    }
    println!();
  }
}

fn solve_part1(map: &mut HashMap<(i32,i32),u8>) -> i32 {
  let mut retval = 0;

  let max_col = *map.keys().map(|(a,b)| b ).max().unwrap();
  let max_row = *map.keys().map(|(a,b)| a ).max().unwrap();
  // println!("max {} {}", max_row,max_col);


  // let mut occupied = 0;
  // for row in 0..=max_row {
  //   for col in 0..=max_col {
  //     occupied += is_seat_full(map, row, col);

  //   }
  // }
  // println!("starting occupied {}", occupied);
  let mut new_map = map.clone();

  let mut changed = 0;
  for row in 0..=max_row {
    for col in 0..=max_col {
      if new_map.get(&(row,col)) != None && new_map.get(&(row,col)).unwrap() == &b'L' {
        if check_adjacent_seats(&new_map, row, col) {
          // println!("filling a seat {} {}", row,col);
          map.entry((row,col)).and_modify(|n| *n=b'#');
          changed += 1;
        }
      } else if is_seat_full(&new_map, row, col) > 0 {
        if count_adjacent_seats(&new_map, row, col) >= 4 {
          // println!("emptying a seat {} {}", row,col);
          map.entry((row,col)).and_modify(|n| *n=b'L');
          changed +=1;
        }
      }

    }
  }
  // println!("Changed {}", changed);
  let mut occupied = 0;
  for row in 0..=max_row {
    for col in 0..=max_col {
      occupied += is_seat_full(&map, row, col);

    }
  }

  
  return occupied;
}

fn solve_part2(map: &mut HashMap<(i32,i32),u8>) -> i32 {
  let mut retval = 0;

  let max_col = *map.keys().map(|(a,b)| b ).max().unwrap();
  let max_row = *map.keys().map(|(a,b)| a ).max().unwrap();
  // println!("max {} {}", max_row,max_col);

  let mut new_map = map.clone();
  let seat_map = SeatMap {map: new_map, max_col: max_col , max_row: max_row };


  let mut changed = 0;
  for row in 0..=max_row {
    for col in 0..=max_col {
      if seat_map.map.get(&(row,col)) != None && seat_map.map.get(&(row,col)).unwrap() == &b'L' {
        if count_visible_seats(&seat_map, row, col, true) == 0 {
          // println!("filling a seat {} {}", row,col);
          map.entry((row,col)).and_modify(|n| *n=b'#');
          changed += 1;
        }
      } else if is_seat_full(&seat_map.map, row, col) > 0 {
        if count_visible_seats(&seat_map, row, col, false) >= 5 {
          // println!("emptying a seat {} {}", row,col);
          map.entry((row,col)).and_modify(|n| *n=b'L');
          changed +=1;
        }
      }

    }
  }

  let mut occupied = 0;
  for row in 0..=max_row {
    for col in 0..=max_col {
      occupied += is_seat_full(&map, row, col);

    }
  }

  
  return occupied;
}


pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec11.txt").lines().map(|s| (&*s).to_string() ).collect();

  let mut map = <HashMap<(i32,i32),u8>>::new();

  let mut col = 0;
  let mut row = 0;

  for line in lines {
    let bytes = line.as_bytes();
    for &item in bytes {
      map.entry((row,col)).or_insert(item);
      col += 1;
    }
    col = 0;
    row +=1;
  }

  // solve logic
  let mut seats = 0;
  let mut last_seats = -1;

  for i in 0..100 {
    seats = solve_part1(&mut map);
    if seats == last_seats {
      println!("Day 11 Part 1 iteration {} resulting in {} occupied seats", i, seats);
      break;
    }
    last_seats = seats;
  }

// PART 2
  let lines: Vec<String> = include_str!("../inputs/dec11.txt").lines().map(|s| (&*s).to_string() ).collect();

  let mut map = <HashMap<(i32,i32),u8>>::new();

  let mut col = 0;
  let mut row = 0;

  for line in lines {
    let bytes = line.as_bytes();
    for &item in bytes {
      map.entry((row,col)).or_insert(item);
      col += 1;
    }
    col = 0;
    row +=1;
  }

  // solve logic
  // println!("found {:?}", map.get(&(93,15)));
  // map.entry((93,15)).and_modify(|n| *n=b'#');
  // println!("found {:?}", map.get(&(93,15)));

  let mut seats = 0;
  let mut last_seats = -1;

  // print_map(&mut map);
  for i in 0..100 {
    seats = solve_part2(&mut map);
    // print_map(&mut map);
    if seats == last_seats {
      println!("Day 11 Part 2 {} iteratioin resulting in {} occupied seats", i, seats);
      break;
    }
    last_seats = seats;
  }
}

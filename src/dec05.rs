use std::fs;
use std::cmp;

pub fn read_input(filename: String) -> Vec<String> {
  let mut seats = Vec::<String>::new();

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let splits = contents.lines();

  for split in splits {
    let s = String::from(split);
    seats.push(s);
  }

  seats
}

const ROW_HI: u32 = 127;
const ROW_LO: u32 = 0;
const COL_HI: u32 = 7;
const COL_LO: u32 = 0;

fn calculate_seat_id(seat: String) -> u32 {
  let instructions = seat.as_bytes();
  let mut row_hi = ROW_HI;
  let mut row_lo = ROW_LO;
  let mut col_hi = COL_HI;
  let mut col_lo = COL_LO;

  for i in instructions {
    if *i == b'F' {
      row_hi = row_hi - (row_hi-row_lo)/2;
    } else if *i == b'B' {
      let diff = row_hi - row_lo;
      row_lo = row_lo + diff/2 + diff % 2;
    } else if *i == b'L' {
      col_hi = col_hi - (col_hi-col_lo)/2;
    } else if *i == b'R' {
      let diff = col_hi - col_lo;
      col_lo = col_lo + diff/2 + diff % 2;
    }
  }
  // println!("seat {} row {} col {}", seat, row_lo, col_lo);
  return row_lo * 8 + col_lo;
}

pub fn solve(seats: &Vec<String>, which: usize, max:u32) -> u32 {

  if which >= seats.len() {
    return max;
  } else {
    let s = &seats[which];
    return solve(seats,
          which+1,
          cmp::max(max,calculate_seat_id(s.to_string()))
        );
  }
  // let s = seats[which];

  // for s in seats {
  //   println!("")
  // }
}

pub fn validate() {
  assert!(calculate_seat_id("BFFFBBFRRR".to_string())==567);
  assert!(calculate_seat_id("FFFBBBFRRR".to_string())==119);
}

#[derive(Debug)]
pub struct Thing {
  operator: Operator,
  argument: i32,
}
#[derive(Debug,Copy,Clone)]
enum Operator {
  North,
  South,
  East,
  West,
  Left,
  Right,
  Forward,
}
// ********** COMMON ***************

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec12.txt").lines().map(|s| (&*s).to_string() ).collect();
  let mut things = <Vec<Thing>>::new();

  for line in lines {
    let op : &str = &line[0..1].to_string();
    let arg = line[1..].to_string();

    let operator = match op {
      "N" => Operator::North,
      "S" => Operator::South,
      "E" => Operator::East,
      "W" => Operator::West,
      "L" => Operator::Left,
      "R" => Operator::Right,
      "F" => Operator::Forward,
      _ => panic!("Invalid char"),
    };

    things.push( Thing {argument: arg.parse::<i32>().unwrap(), operator: operator } );
  }

  // solve logic
  solve_part1(&things);
  solve_part2(&things);
}

// ********** PART 1 ***************
// ********** PART 1 ***************
// ********** PART 1 ***************
fn move_part1(direction:Operator, units: i32, pos: (i32,i32)) -> (i32,i32) {

  return match direction {
    Operator::North => {(pos.0, pos.1 + units)}
    Operator::East => {(pos.0 + units, pos.1)}
    Operator::South => {(pos.0,pos.1 - units)}
    Operator::West => {(pos.0 - units,pos.1)}
    _ => {panic!("invalid direction"); }
  };
}

fn solve_part1(things: &Vec<Thing>) {
  let mut facing = Operator::East;
  let mut position = (0,0);

  for t in things {
    match t.operator {
      Operator::Left | Operator::Right => { facing = turn(facing, t.argument, t.operator); }
      Operator::North | Operator::East | Operator::West | Operator::South => {
        position = move_part1(t.operator, t.argument, position);
      }
      Operator::Forward => {
        position = move_part1(facing, t.argument, position);
      }
    }
  }
  println!("Day 12 Part 1 manhattan is {}", (position.0 as i32).abs() + (position.1 as i32).abs());
}

fn next_direction(direction: Operator) -> Operator {
  match direction {
    Operator::North => { Operator::East }
    Operator::East => { Operator::South }
    Operator::South => { Operator::West }
    Operator::West => { Operator::North }
    _ => {panic!("invalid direction"); }
  }

}
fn turn(op: Operator, degrees: i32, direction: Operator) -> Operator {
  match direction {
    Operator::Right => {turn_right(op, degrees)}
    Operator::Left => {turn_left(op,degrees)}
    _ => { panic!("failed turn"); }
  }
}

fn turn_right(op: Operator, degrees: i32) -> Operator {
  match degrees {
    90 => {   next_direction(op) }
    180 => { turn_right(turn_right(op,90),90) }
    270 => { turn_right(turn_right(turn_right(op,90),90),90) }
    360 => { op }
    _ => {panic!("failed left turn");}
  }
}

fn turn_left(op: Operator, degrees: i32) -> Operator {
  match degrees {
    90 => {   turn_right(op, 270) }
    180 => { turn_left(turn_left(op,90),90) }
    270 => { turn_left(turn_left(turn_left(op,90),90),90) }
    360 => { op }
    _ => {panic!("failed left turn");}
  }
}

// ********** PART 2 ***************
// ********** PART 2 ***************
// ********** PART 2 ***************
fn turn_waypoint (position: (i32,i32), degrees: i32, direction: Operator) -> (i32,i32) {
  match direction {
    Operator::Left => {turn_left_waypoint(position, degrees)}
    Operator::Right => {turn_right_waypoint(position, degrees)}
    _ => { panic!("failed turn waypoint"); }
  }
}

fn turn_left_waypoint(position: (i32,i32), degrees : i32) -> (i32,i32) {
  return
    match degrees {
      90 => { turn_right_waypoint(turn_right_waypoint(turn_right_waypoint(position,90),90),90) }
      180 => { turn_left_waypoint(turn_left_waypoint(position,90),90) }
      270 => { turn_right_waypoint(position,90) }
      360 => { position }
      _ => {panic!("failed left turn");}
    };
}

fn turn_right_waypoint(position: (i32,i32), degrees : i32) -> (i32,i32) {
  return match degrees {
    90 => { (position.1, -1 * position.0) }
    180 => { turn_right_waypoint(turn_right_waypoint(position,90),90) }
    270 => { turn_right_waypoint(turn_right_waypoint(turn_right_waypoint(position,90),90),90) }
    360 => { panic!("360"); }
    _ => {panic!("failed left turn");}
  }
}

fn solve_part2(things: &Vec<Thing>) {
  let mut waypoint = (10,1);
  let mut position = (0,0);

  for t in things {
    match t.operator {
      Operator::Left | Operator::Right => { waypoint = turn_waypoint(waypoint, t.argument, t.operator); }
      Operator::North | Operator::East | Operator::South | Operator::West => {
        waypoint = move_part1(t.operator, t.argument, waypoint);
      }
      Operator::Forward => {
        position = (position.0 + waypoint.0 * t.argument, position.1 + waypoint.1 * t.argument);

      }
    }
  }
  println!("Day 12 Part 2 manhattan is {}", (position.0 as i32).abs() + (position.1 as i32).abs());

}
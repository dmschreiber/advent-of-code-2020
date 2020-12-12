

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
  let new_x;
  let new_y;

    match direction {
    Operator::North => {
      new_y = pos.1 + units;
      new_x = pos.0;
    }
    Operator::East => {
      new_x = pos.0 + units;
      new_y = pos.1;
    }
    Operator::South => {
      new_y = pos.1 - units;
      new_x = pos.0;
    }
    Operator::West => {
      new_x = pos.0- units;
      new_y = pos.1;
    }
    _ => {panic!("invalid direction"); }
  }
  (new_x,new_y)
}

fn solve_part1(things: &Vec<Thing>) {
  let mut facing = Operator::East;
  let mut position = (0,0);

  for t in things {
    match t.operator {
      Operator::Left => {
        facing = turn_left(facing,t.argument);
      }
      Operator::Right => {
        facing = turn_right(facing,t.argument);
      }
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

fn turn_right(op: Operator, degrees: i32) -> Operator {
  match degrees {
    90 => {   
      match op {
        Operator::North => { Operator::East }
        Operator::East => { Operator::South }
        Operator::South => { Operator::West }
        Operator::West => { Operator::North }
        _ => {panic!("failed turn"); }
      }
    }
    180 => { turn_right(turn_right(op,90),90) }
    270 => { turn_right(turn_right(turn_right(op,90),90),90) }
    360 => { op }
    _ => {panic!("failed left turn");}
  }
}

fn turn_left(op: Operator, degrees: i32) -> Operator {
  match degrees {
    90 => {   
      match op {
        Operator::North => { Operator::West }
        Operator::West => { Operator::South }
        Operator::South => { Operator::East }
        Operator::East => { Operator::North }
        _ => {panic!("failed turn"); }
      }
    }
    180 => { turn_left(turn_left(op,90),90) }
    270 => { turn_left(turn_left(turn_left(op,90),90),90) }
    360 => { op }
    _ => {panic!("failed left turn");}
  }
}

// ********** PART 2 ***************
// ********** PART 2 ***************
// ********** PART 2 ***************
fn turn_left_waypoint(position: (i32,i32), degrees : i32) -> (i32,i32) {
  let new_pos;

  match degrees {
    90 => { new_pos = turn_right_waypoint(turn_right_waypoint(turn_right_waypoint(position,90),90),90); }
    180 => { new_pos = turn_left_waypoint(turn_left_waypoint(position,90),90); }
    270 => { new_pos = turn_right_waypoint(position,90); }
    360 => { new_pos = position; }
    _ => {panic!("failed left turn");}
  }

  new_pos
}

fn turn_right_waypoint(position: (i32,i32), degrees : i32) -> (i32,i32) {
  let new_pos;

  match degrees {
    90 => { new_pos = (position.1, -1 * position.0); }
    180 => { new_pos = turn_right_waypoint(turn_right_waypoint(position,90),90); }
    270 => { new_pos = turn_right_waypoint(turn_right_waypoint(turn_right_waypoint(position,90),90),90); }
    360 => { panic!("360"); }
    _ => {panic!("failed left turn");}
  }

  new_pos
}

fn solve_part2(things: &Vec<Thing>) {
  let mut waypoint = (10,1);
  let mut position = (0,0);

  for t in things {
    match t.operator {
      Operator::Left => {
        waypoint = turn_left_waypoint(waypoint,t.argument);
      }
      Operator::Right => {
        waypoint = turn_right_waypoint(waypoint,t.argument);
      }
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
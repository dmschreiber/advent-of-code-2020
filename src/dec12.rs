

#[derive(Debug)]
pub struct Thing {
  operator: Operator,
  argument: i32,
}
#[derive(Debug)]
enum Operator {
  North,
  South,
  East,
  West,
  Left,
  Right,
  Forward,
}

// ********** PART 1 ***************
// ********** PART 1 ***************
// ********** PART 1 ***************

fn turn_right(op: Operator, degrees: i32) -> Operator {
  match op {
    Operator::North => {
      match degrees {
        90 => { Operator::East }
        180 => { Operator::South }
        270 => { Operator::West }
        360 => { Operator::North }
        _ => {panic!("failed left turn");}
      }
    }
    Operator::East => {
      match degrees {
        90 => { Operator::South }
        180 => { Operator::West }
        270 => { Operator::North }
        360 => { Operator::East }
        _ => {panic!("failed left turn");}
      }
    }
    Operator::South => {
      match degrees {
        90 => { Operator::West }
        180 => { Operator::North }
        270 => { Operator::East }
        360 => { Operator::East }
        _ => {panic!("failed left turn");}
      }

    }
    Operator::West => {
      match degrees {
        90 => { Operator::North }
        180 => { Operator::East }
        270 => { Operator::South }
        360 => { Operator::West }
        _ => {panic!("failed left turn");}
      }

    }
    _ => {panic!("failed"); }
  }
}

fn turn_left(op: Operator, degrees: i32) -> Operator {
  match op {
    Operator::North => {
      match degrees {
        90 => { Operator::West }
        180 => { Operator::South }
        270 => { Operator::East }
        360 => { Operator::North }
        _ => {panic!("failed left turn");}
      }
    }
    Operator::East => {
      match degrees {
        90 => { Operator::North }
        180 => { Operator::West }
        270 => { Operator::South }
        360 => { Operator::East }
        _ => {panic!("failed left turn");}
      }
    }
    Operator::South => {
      match degrees {
        90 => { Operator::East }
        180 => { Operator::North }
        270 => { Operator::West }
        360 => { Operator::East }
        _ => {panic!("failed left turn");}
      }

    }
    Operator::West => {
      match degrees {
        90 => { Operator::South }
        180 => { Operator::East }
        270 => { Operator::North }
        360 => { Operator::West }
        _ => {panic!("failed left turn");}
      }

    }
    _ => {panic!("failed"); }
  }
}
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
  let mut facing = Operator::East;
  let mut x = 0;
  let mut y = 0;

  for t in &things {
    match t.operator {
      Operator::Left => {
        facing = turn_left(facing,t.argument);
      }
      Operator::Right => {
        facing = turn_right(facing,t.argument);
      }
      Operator::North => {
        y = y + t.argument;
      }
      Operator::East => {
        x = x + t.argument;
      }
      Operator::South => {
        y = y - t.argument;
      }
      Operator::West => {
        x = x - t.argument;
      }
      Operator::Forward => {
        match facing {
          Operator::North => {
            y = y + t.argument;
          }
          Operator::East => {
            x = x + t.argument;
          }
          Operator::South => {
            y = y - t.argument;
          }
          Operator::West => {
            x = x - t.argument;
          }
          _ => { panic!("fail");}
        }
      }
    }
  }
  println!("Day 12 Part 1 manhattan is {}", x.abs() + y.abs());
  solve_part2(&things);
}

// ********** PART 2 ***************
// ********** PART 2 ***************
// ********** PART 2 ***************
fn turn_left_waypoint(units_x : i32, units_y :i32, degrees : i32) -> (i32,i32) {
  let new_x;
  let new_y;

  match degrees {
      270 => { new_x = units_y; new_y = -1 * units_x; }
      180 => { new_x = -1 * units_x; new_y = -1 * units_y; }
      90 => { new_x = -1 * units_y; new_y = units_x; }
      360 => { panic!("360");}
      _ => {panic!("failed left turn");}
  }


  (new_x,new_y)
}

fn turn_right_waypoint(units_x : i32, units_y :i32, degrees : i32) -> (i32,i32) {
  let new_x;
  let new_y;

  match degrees {
    90 => { new_x = units_y; new_y = -1 * units_x; }
    180 => { new_x = -1 * units_x; new_y = -1 * units_y; }
    270 => { new_x = -1 * units_y; new_y = units_x; }
    360 => { panic!("360"); }
    _ => {panic!("failed left turn");}
  }

  
  (new_x,new_y)
}

fn solve_part2(things: &Vec<Thing>) {

  let mut units_x = 10;
  let mut units_y = 1;

  let mut x = 0;
  let mut y = 0;

  for t in things {
    match t.operator {
      Operator::Left => {
        let a = turn_left_waypoint(units_x,units_y,t.argument);
        units_x = a.0;
        units_y = a.1;
      }
      Operator::Right => {
        let a = turn_right_waypoint(units_x,units_y,t.argument);
        units_x = a.0;
        units_y = a.1;
      }
      Operator::North => {
        units_y = units_y + t.argument;
      }
      Operator::East => {
        units_x = units_x + t.argument;
      }
      Operator::South => {
        units_y = units_y - t.argument;
      }
      Operator::West => {
        units_x = units_x - t.argument;
      }
      Operator::Forward => {
        x = x + units_x * t.argument;
        y = y + units_y * t.argument;

      }
    }
  }
  println!("Day 12 Part 2 manhattan is {}", x.abs() + y.abs());

}
fn solve_part1(nums : &Vec<i64>) {

}

fn solve_part2(nums : &Vec<i64>) {

}

pub fn solve() {
  let lines: Vec<String> = include_str!("../inputs/dec13.txt").lines().map(|s| (&*s).to_string() ).collect();
  let start = lines.get(0).unwrap().parse::<i64>().unwrap();

  let mut buses = <Vec<i64>>::new();
  for s in lines.get(1).unwrap().split(",") {
    if s != "x" {   
      buses.push(s.parse::<i64>().unwrap());
    }
  }

  println!("strt: {:?}, buses {:?}", start, buses);
  let mut found_bus = 0;

  // solve logic
  for t in start..=start+1000 {
    if found_bus > 0 {
      break;
    }
    for b in &buses {
      if t % b == 0 {
        println!("Found bus {} at timestamp {} product is {}", b, t, (t-start)*b);
        found_bus = *b;
        break;
      }
    }
  }

  // solve part 2
  let lines: Vec<String> = include_str!("../inputs/dec13.txt").lines().map(|s| (&*s).to_string() ).collect();
  let start = lines.get(0).unwrap().parse::<i64>().unwrap();

  let mut buses = <Vec<i64>>::new();
  for s in lines.get(1).unwrap().split(",") {
    if s != "x" {   
      buses.push(s.parse::<i64>().unwrap());
    } else {
      buses.push(-1);
    }
  }

  println!("strt: {:?}, buses {:?}", start, buses);
  let mut bus_offset = <Vec<(i64,i64)>>::new();
  let mut i = 0;
  for b in &buses {
    // println!("comparing {} slot ahead", i);
    if *b > 0 {
        bus_offset.push((i,*b));
        println!("offset {} bus {} ", i, b);
      } else {
        // println!("{} mod {} is  zero", t+i, b);
      }
      i += 1;
    }

    let mut t : i64 ;
  let mut p = 191000000000;
  // let mut p = 1;
  loop {
    t= 521*p-19;
    // t = 59*p-4;
    if p % 1000000000 == 0 {
      println!("trying {} at time {}", p, t);
    }
    
    if bus_offset.iter().map(|b| (t+b.0) % b.1 ).sum::<i64>() == 0 {

      let mut found = true;
      let mut i = 0;

      for b in &buses {
        // println!("comparing {} slot ahead", i);
        if *b > 0 {
          if (t + i) % b != 0 {
            // println!("{} mod {} is not zero", t+i, b);
            found = false;
            break;
          } else {
            // println!("{} mod {} is  zero", t+i, b);
          }
        }
        i += 1;
      }
      if found {
        println!("Found at time {}", t);
        break;
      }
    }
    p += 1;
  }
}
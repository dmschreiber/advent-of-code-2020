
fn mod_inv(a: i64, module: i64) -> i64 {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
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

  // println!("strt: {:?}, buses {:?}", start, buses);
  let mut found_bus = 0;

  // solve logic
  for t in start..=start+1000 {
    if found_bus > 0 {
      break;
    }
    for b in &buses {
      if t % b == 0 {
        println!("Day 13 part 1 Found bus {} at timestamp {} product is {}", b, t, (t-start)*b);
        found_bus = *b;
        break;
      }
    }
  }

  // solve part 2
  let lines: Vec<String> = include_str!("../inputs/dec13.txt").lines().map(|s| (&*s).to_string() ).collect();

  let mut buses = <Vec<i64>>::new();
  for s in lines.get(1).unwrap().split(",") {
    if s != "x" {   
      buses.push(s.parse::<i64>().unwrap());
    } else {
      buses.push(-1);
    }
  }

  let mut bus_offset = <Vec<(i64,i64)>>::new();
  let mut i = 0;
  for b in &buses {
    // println!("comparing {} slot ahead", i);
    if *b > 0 {
        bus_offset.push((i,*b));
        // println!("offset {} bus {} ", i, b);
      } else {
        // println!("{} mod {} is  zero", t+i, b);
      }
      i += 1;
    }

  let t : i64 ;

  let mut total = 0;
  for bus in &bus_offset {
    let bi = (-bus.0).rem_euclid(bus.1);
    let big_n_i = bus_offset.iter().filter(|b| b.1 != bus.1).fold(1, | acc, x| acc * x.1 );
    let xi = mod_inv(big_n_i,bus.1);
    total += bi*big_n_i*xi;
    // println!("bi {} Ni {} xi {} xi-check {} total {}", bi, Ni, xi, xi*Ni % bus.1, total);
  }

  total = total % bus_offset.iter().fold(1, | acc, x| acc * x.1 );
  t = total;

  loop {
    if bus_offset.iter().map(|b| (t+b.0) % b.1 ).sum::<i64>() == 0 {
      let  found = true;
      if found {
        println!("Day 13 part 2 Found at time {}", t);
        break;
      }
    }
  }
}
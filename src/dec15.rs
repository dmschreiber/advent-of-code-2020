use std::collections::HashMap;

#[cfg(test)]
mod tests {

  #[test]
  pub fn solve_prod() {
      let lines: Vec<String> = include_str!("../inputs/dec15.txt").lines().map(|s| (&*s).to_string() ).collect();
      assert!(276==super::solve(&lines,2020));
      assert!(31916==super::solve(&lines,30000000));
  }

  #[test]
  pub fn solve_test() {
    let lines: Vec<String> = include_str!("../inputs/dec15-test.txt").lines().map(|s| (&*s).to_string() ).collect();
    let start = std::time::Instant::now();
    assert!(1836==super::solve(&lines,2020));
    assert!(362==super::solve(&lines,30000000));
    println!("My way in {:?}", start.elapsed());

    let line = &lines[0];
    let args : Vec<u32> = line.split(",").map(|s| (&*s).parse::<u32>().unwrap() ).collect();
    // println!("INPUT LINE {}",line);
    
    let start = std::time::Instant::now();
    let mut v = super::van_eck_sequence(args).take(30000000);
    for n in v.by_ref().skip(2019).take(1) {
      println!("{}",n);
      assert!(1836==n);
    }

    for n in v.by_ref().skip(30000000-2020-1) {
      println!("{}",n);
      assert!(362==n);
    }
    println!("Rosetta Code way in {:?}", start.elapsed());
  }
}

#[allow(dead_code)]
fn van_eck_sequence(seed : Vec<u32>) -> impl std::iter::Iterator<Item = u32> {
  let mut index = 0;
  let mut last_term = 0;
  let mut last_pos = std::collections::HashMap::new();
  std::iter::from_fn(move || {
      let result = last_term;
      let mut next_term = 0;
      if index < seed.len() {
        last_pos.insert(seed[index],index);
      } else if let Some(v) = last_pos.get_mut(&last_term) {
          next_term = index - *v;
          *v = index;
      } else {
          last_pos.insert(last_term, index);
      }
      last_term = next_term as u32;
      index += 1;
      Some(result)
  })
}

pub fn solve(lines : &Vec<String>, position : usize) -> u32 {

      let line = &lines[0];
      let args : Vec<u32> = line.split(",").map(|s| (&*s).parse::<u32>().unwrap() ).collect();
      // println!("INPUT LINE {}",line);
      
      let mut history = <HashMap<u32,Vec<usize>>>::new();

      // solve
      let mut index = 0;
      let mut last_number = 0;
      let mut num;
      loop {

          if index < args.len() {
            num = args[index];
          } else if let Some(pos_array) = history.get(&last_number) {
            if pos_array.len() < 2 {
              num = 0;
            } else {
              num = (pos_array[0] - pos_array[1])as u32;
            }
          } else {
            panic!("Shouldn't get here");
          }
          
          if let Some(pos_array) = history.get_mut(&num) { // None or Some(value)
            pos_array.insert(0,index);
            if pos_array.len() > 2 {
              pos_array.pop();
            }
          } else {
            history.insert(num,vec![index]); 
          }

          index += 1;
          last_number = num;

          if index == position {
            // println!("num: {}", num);
           return num;
          }
      }
}



use std::time::{Instant};
mod dec01;
mod dec02;

fn dec01() {
    let mut my_numbers = dec01::read_input("/Users/dschreiber/Projects/advent-of-code-2020/src/dec01.txt".to_string());

    // sort
    my_numbers.sort();
    // println!("{:?}", my_numbers);

    dec01::solve(my_numbers.clone());
    dec01::solve_part2(my_numbers.clone());

}

fn dec02() {
    let my_passwords = dec02::read_input("/Users/dschreiber/Projects/advent-of-code-2020/src/dec02.txt".to_string());
    // println!("{:?}", my_passwords);
    dec02::solve_part1(&my_passwords);
    dec02::solve_part2(&my_passwords);
}

fn main() {
    let start = Instant::now();
    dec01();
    println!("Day 1 complete in {:?}", start.elapsed());
    let start = Instant::now();
    dec02();
    println!("Day 2 complete in {:?}", start.elapsed());

}

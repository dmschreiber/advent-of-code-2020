
use std::time::{Instant};
mod dec01;
mod dec02;
mod dec03;

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

    dec02::solve_part1(&my_passwords);
    dec02::solve_part2(&my_passwords);
}

fn dec03() {
    let my_tree_map = dec03::read_tree_map("/Users/dschreiber/Projects/advent-of-code-2020/src/dec03.txt".to_string());

    let start = Instant::now();
    let p = 
    dec03::solve(&my_tree_map,1,1) as u128 *
    dec03::solve(&my_tree_map,1,3)  as u128*
    dec03::solve(&my_tree_map,1,5) as u128*
    dec03::solve(&my_tree_map,1,7) as u128*
    dec03::solve(&my_tree_map,2,1) as u128;
    println!("{} is the product of all (4239209520 is wrong)", p);
    println!("Day 3 complete in {:?}", start.elapsed());

}

fn main() {
    let start = Instant::now();
    dec01();
    println!("Day 1 complete in {:?}", start.elapsed());
    let start = Instant::now();

    dec02();
    println!("Day 2 complete in {:?}", start.elapsed());

    dec03();
}

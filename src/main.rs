
#[macro_use] extern crate lazy_static;
use std::time::{Instant};
mod dec01;
mod dec02;
mod dec03;
mod dec04;

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
    dec03::solve(&my_tree_map,1,3) as u128*
    dec03::solve(&my_tree_map,1,5) as u128*
    dec03::solve(&my_tree_map,1,7) as u128*
    dec03::solve(&my_tree_map,2,1) as u128;
    println!("{} is the product of all", p);
    println!("Day 3 complete in {:?}", start.elapsed());

}

fn dec04() {

    assert!(dec04::validate_birthyear("2002"));
    assert!(!dec04::validate_birthyear("2003"));
    assert!(dec04::validate_height("60in"));
    assert!(dec04::validate_height("190cm"));
    assert!(!dec04::validate_height("190in"));
    assert!(!dec04::validate_height("190"));
    assert!(dec04::validate_haircolor("#123abc"));
    assert!(!dec04::validate_haircolor("#123abz"));
    assert!(!dec04::validate_haircolor("123abc"));
    assert!(dec04::validate_eyecolor("brn"));
    assert!(!dec04::validate_eyecolor("wat"));
    assert!(dec04::validate_pid("000000001"));
    assert!(!dec04::validate_pid("0123456789"));


    let start = Instant::now();
    let passports = dec04::read_input("/Users/dschreiber/Projects/advent-of-code-2020/src/dec04.txt".to_string());
    println!("Day 4 read file in {:?}", start.elapsed());

    let start = Instant::now();
    dec04::solve(&passports, false);
    println!("Day 4 part 1 in {:?}", start.elapsed());

    let start = Instant::now();
    dec04::solve(&passports, true);
    println!("Day 4 part 2 in {:?}", start.elapsed());

}
fn main() {
    let start = Instant::now();
    dec01();
    println!("Day 1 complete in {:?}", start.elapsed());
    let start = Instant::now();

    dec02();
    println!("Day 2 complete in {:?}", start.elapsed());

    dec03();

    let start = Instant::now();
    dec04();
    println!("Day 4 complete in {:?}", start.elapsed());
}

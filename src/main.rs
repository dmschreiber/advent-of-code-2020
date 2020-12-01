mod dec01;

fn main() {
    println!("Hello, AoC!");

    let mut my_numbers = dec01::read_input("/Users/dschreiber/Projects/advent-of-code-2020/src/dec01.txt".to_string());

    // sort
    my_numbers.sort();
    // println!("{:?}", my_numbers);

    dec01::solve(my_numbers);
}

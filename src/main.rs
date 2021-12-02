mod day1;
mod day2;

fn main() {
    let (part_1, part_2) = day1::solve();
    println!("Day 1 --- Part 1: {} Part 2: {}", part_1, part_2);

    let (part_1, part_2) = day2::solve();
    println!("Day 1 --- Part 1: {} Part 2: {}", part_1, part_2);
}

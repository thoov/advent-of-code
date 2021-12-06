mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let (part_1, part_2) = day1::solve();
    println!("Day 1 --- Part 1: {} Part 2: {}", part_1, part_2);

    let (part_1, part_2) = day2::solve();
    println!("Day 2 --- Part 1: {} Part 2: {}", part_1, part_2);

    let (part_1, part_2) = day3::solve();
    println!("Day 3 --- Part 1: {} Part 2: {}", part_1, part_2);

    let (part_1, part_2) = day4::solve();
    println!("Day 4 --- Part 1: {} Part 2: {}", part_1, part_2);

    let (part_1, part_2) = day5::solve();
    println!("Day 5 --- Part 1: {} Part 2: {}", part_1, part_2);
}

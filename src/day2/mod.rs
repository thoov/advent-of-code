use std::fs;

pub fn solve() -> (isize, isize) {
    let contents =
        fs::read_to_string("src/day2/input.txt").expect("Something went wrong reading the file");

    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> isize {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for command_str in contents.lines() {
        let command_split: Vec<&str> = command_str.split_whitespace().collect();
        let command = command_split[0];
        let amount: isize = command_split[1].parse().unwrap();

        match command {
            "forward" => horizontal_pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => println!("Unknown command"),
        }
    }

    horizontal_pos * depth
}

fn part_2(contents: &str) -> isize {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command_str in contents.lines() {
        let command_split: Vec<&str> = command_str.split_whitespace().collect();
        let command = command_split[0];
        let amount: isize = command_split[1].parse().unwrap();

        match command {
            "forward" => {
                horizontal_pos += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => println!("Unknown command"),
        }
    }

    horizontal_pos * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (1728414, 1765720035));
    }

    #[test]
    fn examples_works() {
        let test_str = String::from("forward 5
down 5
forward 8
up 3
down 8
forward 2");

        assert_eq!(part_1(&test_str), 150);
        assert_eq!(part_2(&test_str), 900);
    }
}

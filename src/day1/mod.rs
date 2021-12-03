use std::fs;

pub fn solve() -> (isize, isize) {
    let contents =
        fs::read_to_string("src/day1/input.txt").expect("Something went wrong reading the file");

    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> isize {
    let depth_readings: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();
    count_increases(&depth_readings)
}

fn part_2(contents: &str) -> isize {
    let mut depth_groupings: Vec<i32> = vec![];
    let number_of_readings = contents.lines().count();

    for (idx, depth) in contents.lines().enumerate() {
        let depth_reading: i32 = depth.parse().unwrap();

        if idx == 0 {
            depth_groupings.push(depth_reading);
        } else if idx == 1 {
            depth_groupings.push(depth_reading);
            depth_groupings[idx - 1] += depth_reading;
        } else if idx < number_of_readings - 2 {
            depth_groupings.push(depth_reading);
            depth_groupings[idx - 2] += depth_reading;
            depth_groupings[idx - 1] += depth_reading;
        } else if idx == number_of_readings - 2 {
            depth_groupings[idx - 2] += depth_reading;
            depth_groupings[idx - 1] += depth_reading;
        } else if idx == number_of_readings - 1 {
            depth_groupings[idx - 2] += depth_reading;
        }
    }

    count_increases(&depth_groupings)
}

fn count_increases(depth_readings: &[i32]) -> isize {
    let mut last_depth_reading = 0;
    let mut depth_has_increased = 0;

    for (idx, &depth) in depth_readings.iter().enumerate() {
        if last_depth_reading < depth && idx != 0 {
            depth_has_increased += 1;
        }

        last_depth_reading = depth;
    }

    depth_has_increased
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (1292, 1262));
    }

    #[test]
    fn examples_work() {
        let test_str = String::from(
            "199
200
208
210
200
207
240
269
260
263",
        );
        assert_eq!(part_1(&test_str), 7);
        assert_eq!(part_2(&test_str), 5);
    }
}

use std::fs;
use std::cmp;
use sscanf::*;

pub fn solve() -> (usize, usize) {
    let contents = fs::read_to_string("src/day5/input.txt").expect("Something went wrong reading the file");
    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> usize {
    count_of_dangerous_areas(contents, false)
}

fn part_2(contents: &str) -> usize {
    count_of_dangerous_areas(contents, true)
}

fn count_of_dangerous_areas(vent_readings: &str, allow_diagonals: bool) -> usize {
    let mut field: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for vent_lines in vent_readings.lines() {
        let parsed = scanf!(vent_lines, "{},{} -> {},{}", usize, usize, usize, usize);
        let (x1, y1, x2, y2) = parsed.unwrap();

        if x1 == x2 {
            for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                field[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                field[x][y1] += 1;
            }
        } else if allow_diagonals {
            let diff = cmp::max(x1, x2) - cmp::min(x1, x2);

            let x_rising = x1 < x2;
            let y_rising = y1 < y2;

            for offset in 0..=diff  {
                field[if x_rising { x1 + offset } else { x1 - offset }][if y_rising { y1 + offset } else { y1 - offset }] += 1;
            }
        }
    }

    let mut count = 0;

    for row in field.iter() {
        for vent in row {
            if vent >= &2 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (5294, 21698));
    }

    #[test]
    fn examples_works() {
        let test_str = String::from("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");

        assert_eq!(part_1(&test_str), 5);
        assert_eq!(part_2(&test_str), 12);
    }
}

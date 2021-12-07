use std::{fs, cmp};

pub fn solve() -> (usize, usize) {
    let contents = fs::read_to_string("src/day7/input.txt").expect("Something went wrong reading the file");
    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> usize {
    let horizontal_pos: Vec<usize> = contents.lines().next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    find_efficent_burn_strat(&horizontal_pos, false)
}

fn part_2(contents: &str) -> usize {
    let horizontal_pos: Vec<usize> = contents.lines().next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    find_efficent_burn_strat(&horizontal_pos, true)
}

fn find_efficent_burn_strat(horizontal_pos: &[usize], precalculate: bool) -> usize {
    let min = *horizontal_pos.iter().min().unwrap();
    let max = *horizontal_pos.iter().max().unwrap();
    let burn_rate_lookup = precalculate_burn_rate(max);
    
    let mut lowest_fuel_cost = 999999999999999;

    for i in min..=max {
        let cost = horizontal_pos.iter().fold(0, |sum, item| {
            let fuel_cost = cmp::max(*item, i) - cmp::min(*item, i);

            if precalculate {
               return sum + burn_rate_lookup[fuel_cost]
            }

            sum + fuel_cost
        });

        if cost < lowest_fuel_cost {
            lowest_fuel_cost = cost;
        }
    }

    lowest_fuel_cost
}

fn precalculate_burn_rate(num_of_steps: usize) -> Vec<usize> {
    let mut burn_rate: Vec<usize> = vec![];
    let mut last_calculated_value = 0;

    for i in 0..=num_of_steps {
        let next_value = i + last_calculated_value;
        burn_rate.push(next_value);
        last_calculated_value = next_value;
    }

    burn_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (337833, 96678050));
    }

    #[test]
    fn examples_works() {
        let test_str = String::from("16,1,2,0,4,2,7,1,2,14");

        assert_eq!(part_1(&test_str), 37);
        assert_eq!(part_2(&test_str), 168);
    }
}


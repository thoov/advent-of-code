use std::{fs, vec};

pub fn solve() -> (usize, usize) {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Something went wrong reading the file");
    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> usize {
    let age_of_fish: Vec<usize> = contents.lines().next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    count_lanternfish(&age_of_fish, 80)
}

fn part_2(contents: &str) -> usize {
    let age_of_fish: Vec<usize> = contents.lines().next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    count_lanternfish(&age_of_fish, 256)
}

fn count_lanternfish(age_of_fish: &[usize], num_days: usize) -> usize {
    let mut count_at_stage: Vec<usize> = vec![
        0,
        age_of_fish.iter().filter(|&f| f == &1).count(),
        age_of_fish.iter().filter(|&f| f == &2).count(),
        age_of_fish.iter().filter(|&f| f == &3).count(),
        age_of_fish.iter().filter(|&f| f == &4).count(),
        age_of_fish.iter().filter(|&f| f == &5).count(),
        0,
        0,
        0
    ];

    for _day in 0..num_days {
        let stage_0 = count_at_stage[0];
        let stage_1 = count_at_stage[1];
        let stage_2 = count_at_stage[2];
        let stage_3 = count_at_stage[3];
        let stage_4 = count_at_stage[4];
        let stage_5 = count_at_stage[5];
        let stage_6 = count_at_stage[6];
        let stage_7 = count_at_stage[7];
        let stage_8 = count_at_stage[8];

        count_at_stage[6] = stage_0 + stage_7;
        count_at_stage[7] = stage_8;
        count_at_stage[8] = stage_0;
        count_at_stage[5] = stage_6;
        count_at_stage[4] = stage_5;
        count_at_stage[3] = stage_4;
        count_at_stage[2] = stage_3;
        count_at_stage[1] = stage_2;
        count_at_stage[0] = stage_1;
    }

    count_at_stage.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (374994, 1686252324092));
    }

    #[test]
    fn examples_works() {
        let test_str = String::from("3,4,3,1,2");

        assert_eq!(part_1(&test_str), 5934);
        assert_eq!(part_2(&test_str), 26984457539);
    }
}


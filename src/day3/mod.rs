use std::fs;

pub fn solve() -> (isize, isize) {
    let contents =
        fs::read_to_string("src/day3/input.txt").expect("Something went wrong reading the file");

    (part_1(&contents), part_2(&contents))
}

fn part_1(contents: &str) -> isize {
    let binary_numbers: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let pos_count = calculate_most_popular_bit_type(&binary_numbers);

    let mut gamma_rate_vec: Vec<char> = vec![];
    let mut epsilon_rate_vec: Vec<char> = vec![];

    for value in pos_count {
        if value * 2 > binary_numbers.len() {
            gamma_rate_vec.push('1');
            epsilon_rate_vec.push('0');
        } else {
            gamma_rate_vec.push('0');
            epsilon_rate_vec.push('1');
        }
    }

    let gamma_rate = isize::from_str_radix(&String::from_iter(gamma_rate_vec), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&String::from_iter(epsilon_rate_vec), 2).unwrap();

    gamma_rate * epsilon_rate
}

#[derive(PartialEq)]
enum BinaryType {
    MostPopular,
    LeastPopular,
}

fn part_2(contents: &str) -> isize {
    let binary_numbers: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();

    let oxygen_generator_rating = recursively_find(binary_numbers.clone(), 0, BinaryType::MostPopular);
    let co2_scrubber_rating = recursively_find(binary_numbers, 0, BinaryType::LeastPopular);

    oxygen_generator_rating * co2_scrubber_rating
}

fn recursively_find(binary_numbers: Vec<Vec<char>>, idx: usize, binary_type: BinaryType) -> isize {
    let pos_count = calculate_most_popular_bit_type(&binary_numbers);
    let len = binary_numbers.len();

    let filtered_numbers: Vec<Vec<char>> = binary_numbers.into_iter().filter(|number| {
        let most_pop_char = if pos_count[idx] * 2 >= len { '1' } else { '0' };
        let least_pop_char = if pos_count[idx] * 2 >= len { '0' } else { '1' };
        let filter_char = number.get(idx).unwrap();

        if binary_type == BinaryType::MostPopular {
            return most_pop_char == *filter_char;
        }
        
        least_pop_char == *filter_char
    }).collect();

    if filtered_numbers.len() == 1 {
        return isize::from_str_radix(&String::from_iter(&filtered_numbers[0]), 2).unwrap();
    }

    recursively_find(filtered_numbers, idx + 1, binary_type)
}


fn calculate_most_popular_bit_type(binary_numbers: &[Vec<char>]) -> Vec<usize> {
    let mut pos_count: Vec<usize> = vec![];

    for (binary_idx, binary) in binary_numbers.iter().enumerate() {
        for (idx, char) in binary.iter().enumerate() {
            if binary_idx == 0 {
                pos_count.push(0);
            }


            if char == &'1' { 
                pos_count[idx] += 1 
            }
        }
    }

    pos_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(solve(), (3885894, 4375225));
    }

    #[test]
    fn examples_works() {
        let test_str = String::from("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010");
        assert_eq!(part_1(&test_str), 198);
        assert_eq!(part_2(&test_str), 230);
    }
}

use utils::read_usize_list;
use std::collections::HashMap;

fn count_digits(mut num: usize) -> usize {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    return count;
}

fn reverse_number(mut num: usize) -> usize {
    let mut res = 0;
    while num > 0 {
        res = res * 10 + num % 10;
        num /= 10;
    }
    return res;
}

fn split_in_half(num: usize) -> (usize, usize) {
    let digit_count = count_digits(num);
    let (mut num1, mut num2) = (num, 1);  // Leading 1 to prevent from losing trailing zeros

    for _ in 0..digit_count / 2 {
        num2 = num2 * 10 + num1 % 10;
        num1 /= 10;
    }

    return (num1, reverse_number(num2) / 10);  // Division to remove trailing 1
}

fn evolve_stone(stone: usize, blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    fn get_result(stone: usize, blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if let Some(&res) = cache.get(&(stone, blinks)) {
            return res;
        }

        let res = evolve_stone(stone, blinks, cache);
        cache.insert((stone, blinks), res);
        return res;
    }

    if blinks == 0 {
        return 1;
    }

    if stone == 0 {
        return get_result(1, blinks - 1, cache);
    }

    if count_digits(stone) % 2 == 0 {
        let (stone1, stone2) = split_in_half(stone);
        return get_result(stone1, blinks - 1, cache) + get_result(stone2, blinks - 1, cache);
    }

    return get_result(stone * 2024, blinks - 1, cache);
}

fn main() {
    let stones = read_usize_list();
    let mut cache = HashMap::new();

    let result1: usize = stones.iter()
        .map(|&stone| evolve_stone(stone, 25, &mut cache))
        .sum();
    let result2: usize = stones.iter()
        .map(|&stone| evolve_stone(stone, 75, &mut cache))
        .sum();

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

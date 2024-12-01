use std::io::{self, BufRead};
use std::collections::HashMap;

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let (mut list1, mut list2) = (vec![], vec![]);

    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|s| s.unwrap());
    for line in lines {
        let nums: Vec<i32> = line.trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        list1.push(nums[0]);
        list2.push(nums[1]);
    }


    return (list1, list2);
}

fn count(vec: Vec<i32>) -> HashMap<i32, i32> {
    let mut res = HashMap::new();

    for elem in vec {
        *res.entry(elem).or_insert(0) += 1;
    }

    return res;
}

fn similarity_score(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let list2_count = count(list2);

    let score = list1.into_iter()
        .map(|num| num * list2_count.get(&num).unwrap_or(&0))
        .sum();

    return score;
}

fn main() {
    let (list1, list2) = parse_input();
    let score = similarity_score(list1, list2);

    println!("{}", score);
}

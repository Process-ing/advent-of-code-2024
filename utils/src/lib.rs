use std::io::{self, BufRead};

pub fn parse_two_lists() -> (Vec<i32>, Vec<i32>) {
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

pub fn parse_int_lists() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let lists: Vec<Vec<i32>> = lines.map(|line| {
        line.split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect()
    }).collect();

    return lists;
}

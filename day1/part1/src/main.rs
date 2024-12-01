use std::io::{self, BufRead};

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

fn solve(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();

    let distance = list1.into_iter().zip(list2.into_iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum();

    return distance;
}

fn main() {
    let (list1, list2) = parse_input();
    let result = solve(list1, list2);

    println!("{}", result);
}

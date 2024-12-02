use std::collections::HashMap;
use utils::parse_two_lists;

fn distance(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();

    let distance = list1.into_iter().zip(list2.into_iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum();

    return distance;
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
    let (list1, list2) = parse_two_lists();
    let dist = distance(list1.clone(), list2.clone());
    let score = similarity_score(list1, list2);

    println!("Distance: {dist}");
    println!("Similarity score: {score}");
}

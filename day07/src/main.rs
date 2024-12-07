use std::io::{self, BufRead};

#[derive(Debug)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
}

fn parse_equation(string: String) -> Equation {
    let mut string_parts = string.trim().split(':');

    let result = string_parts.next().unwrap().parse().unwrap();
    let operands = string_parts.next().unwrap()
        .trim()
        .split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();

    return Equation { result, operands };
}

fn read_equations() -> Vec<Equation> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let equations = lines.map(parse_equation).collect();

    return equations;
}

fn reverse_num(mut num: usize) -> usize {
    let mut res = 1;  // Initial one to not lose leading zeros

    while num > 0 {
        res = res * 10 + num % 10;
        num /= 10;
    }

    return res;
}

fn concat_nums(mut a: usize, b: usize) -> usize {
    let mut rev_b = reverse_num(b);

    while rev_b > 1 {
        a = a * 10 + rev_b % 10;
        rev_b /= 10;
    }

    return a;
}

fn is_possible(equation: &Equation, use_concat: bool) -> bool {
    fn is_possible_aux(operands: &[usize], acc: usize, result: usize, use_concat: bool) -> bool {
        if acc > result {  // Bounding condition (every operation increases the number)
            return false;
        }

        if operands.is_empty() {
            return acc == result;
        }

        return is_possible_aux(&operands[1..], acc + operands[0], result, use_concat)
            || is_possible_aux(&operands[1..], acc * operands[0], result, use_concat)
            || (use_concat && is_possible_aux(&operands[1..], concat_nums(acc, operands[0]), result, true));
    }

    let Equation { result, operands } = equation;

    return is_possible_aux(&operands[1..], operands[0], *result, use_concat);
}

fn main() {
    let equations = read_equations();

    let result1: usize = equations.iter()
        .filter(|eq| is_possible(eq, false))
        .map(|eq| eq.result)
        .sum();
    let result2: usize = equations.iter()
        .filter(|eq| is_possible(eq, true))
        .map(|eq| eq.result)
        .sum();

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

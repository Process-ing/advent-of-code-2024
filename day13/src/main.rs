use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn apply_correction(&self) -> Self {
        let Machine {
            button_a,
            button_b,
            prize: (px, py),
        } = *self;

        return Self {
            button_a,
            button_b,
            prize: (px + 10000000000000, py + 10000000000000),
        };
    }
}

fn read_machines() -> Vec<Machine> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines()
        .map(|l| l.unwrap())
        .peekable();

    let mut res = vec![];
    while lines.peek().is_some() {
        let button_a_line = lines.next().unwrap();
        let button_a_parts: Vec<&str> = button_a_line[10..].split_whitespace().collect();
        let button_a = (button_a_parts[0][1..button_a_parts[0].len() - 1].parse().unwrap(), button_a_parts[1][1..].parse().unwrap());

        let button_b_line = lines.next().unwrap();
        let button_b_parts: Vec<&str> = button_b_line[10..].split_whitespace().collect();
        let button_b = (button_b_parts[0][1..button_b_parts[0].len() - 1].parse().unwrap(), button_b_parts[1][1..].parse().unwrap());

        let prize_line = lines.next().unwrap();
        let prize_parts: Vec<&str> = prize_line[7..].split_whitespace().collect();
        let prize = (prize_parts[0][2..prize_parts[0].len() - 1].parse().unwrap(), prize_parts[1][2..].parse().unwrap());

        lines.next();  // Ignore new line

        res.push(Machine { button_a, button_b, prize });
    }

    return res;
}

fn get_min_tokens(machine: Machine) -> usize {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (px, py) = machine.prize;

    // Linear algebra ftw

    let det = (ax * by - bx * ay) as f64;
    let det1 = (px * by - bx * py) as f64;
    let det2 = (ax * py - px * ay) as f64;
    let (a_presses, b_presses) = ((det1 / det).round() as i64, (det2 / det).round() as i64);
    
    return if verify_solution(machine, a_presses, b_presses) { (3 * a_presses + b_presses) as usize } else { 0 };
}

fn verify_solution(machine: Machine, a_presses: i64, b_presses: i64) -> bool {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;

    return (ax * a_presses + bx * b_presses, ay * a_presses + by * b_presses) == machine.prize;
}

fn main() {
    let machines = read_machines();

    let result1: usize = machines.iter().cloned()
        .map(get_min_tokens)
        .sum();
    let result2: usize = machines.iter()
        .map(Machine::apply_correction)
        .map(get_min_tokens)
        .sum();

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

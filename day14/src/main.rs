use std::fs::create_dir_all;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

use image::{ImageBuffer, Rgb};

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn step(&mut self, width: i32, height: i32) {
        self.x = (self.x + self.vx).rem_euclid(width);
        self.y = (self.y + self.vy).rem_euclid(height);
    }

    fn step_after(&mut self, width: i32, height: i32, secs: i32) {
        self.x = (self.x + self.vx * secs).rem_euclid(width);
        self.y = (self.y + self.vy * secs).rem_euclid(height);
    }
}

#[derive(Debug)]
struct ParseRobotError;

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(' ').ok_or(ParseRobotError)?;

        let (x, y) = pos
            .strip_prefix("p=")
            .and_then(|s| s.split_once(','))
            .ok_or(ParseRobotError)?;

        let (vx, vy) = vel
            .strip_prefix("v=")
            .and_then(|s| s.split_once(','))
            .ok_or(ParseRobotError)?;

        Ok(Self {
            x: x.parse().map_err(|_| ParseRobotError)?,
            y: y.parse().map_err(|_| ParseRobotError)?,
            vx: vx.parse().map_err(|_| ParseRobotError)?,
            vy: vy.parse().map_err(|_| ParseRobotError)?,
        })
    }
}

fn read_robots() -> Result<Vec<Robot>, ParseRobotError> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.map_err(|_| ParseRobotError))
        .map(|l| l.and_then(|l| l.parse()))
        .collect()
}

fn get_safety_factor(robots: &[Robot], secs: i32, width: i32, height: i32) -> i32 {
    let x_middle = width / 2;
    let y_middle = height / 2;

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        let mut robot = *robot;
        robot.step_after(width, height, secs);

        if robot.y < y_middle {
            if robot.x > x_middle {
                q1 += 1;
            } else if robot.x < x_middle {
                q2 += 1;
            }
        } else if robot.y > y_middle {
            if robot.x < x_middle {
                q3 += 1;
            } else if robot.x > x_middle {
                q4 += 1;
            }
        }
    }

    q1 * q2 * q3 * q4
}

fn print_robots(robots: &[Robot], width: i32, height: i32) {
    let mut image = ImageBuffer::new(width as u32, height as u32);

    for robot in robots {
        image[(robot.x as u32, robot.y as u32)] = Rgb([255 as u8, 255 as u8, 255 as u8]);
    }

    create_dir_all("./img").expect("Failed to open directory");
    image.save(format!("./img/tree.png")).expect("Failed to save image");
}

fn move_robots(robots: &mut [Robot], width: i32, height: i32) {
    robots
        .into_iter()
        .for_each(|robot| robot.step(width, height));
}

fn has_pattern(robots: &[Robot], width: i32, height: i32) -> bool {
    let mut map = vec![vec![false; width as usize]; height as usize];
    const LINE_TOLERANCE: i32 = 10;

    for robot in robots {
        map[robot.y as usize][robot.x as usize] = true;
    }

    // A priori, no other robot placement will produce a long horizontal line
    for y in 0..height {
        let mut count = 0;
        for x in 0..width {
            if map[y as usize][x as usize] {
                count += 1;
                if count == LINE_TOLERANCE {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }

    return false;
}

fn find_tree(robots: &mut [Robot], width: i32, height: i32) -> i32 {
    let mut secs = 0;
    while !has_pattern(robots, width, height) {
        move_robots(robots, width, height);
        secs += 1;
    }

    return secs;
}

fn main() {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    let mut robots = read_robots().expect("Failed to read input");

    let result1 = get_safety_factor(&robots, 100, WIDTH, HEIGHT);
    let result2 = find_tree(&mut robots, WIDTH, HEIGHT);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");

    print_robots(&robots, WIDTH, HEIGHT);
    println!("Tree image stored in ./img/tree.png");
}

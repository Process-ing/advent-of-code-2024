use std::io::Read;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct ParseMoveError;

impl Move {
    fn from_byte(byte: u8) -> Result<Move, ParseMoveError> {
        match byte {
            b'^' => Ok(Move::Up),
            b'v' => Ok(Move::Down),
            b'<' => Ok(Move::Left),
            b'>' => Ok(Move::Right),
            _   => Err(ParseMoveError),
        }
    }

    fn to_dir(&self) -> (i32, i32) {
        match *self {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ParseInputError;

fn read_map_and_moves() -> Result<(Vec<Vec<u8>>, Vec<Move>), ParseInputError> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).map_err(|_| ParseInputError)?;

    let (map_str, moves_str) = input
        .split_once("\n\n")
        .ok_or(ParseInputError)?;

    let map = map_str.lines()
        .map(|l| l.as_bytes().into_iter().cloned().collect())
        .collect();


    let moves = moves_str.lines()
        .flat_map(|line| {
            line.as_bytes().into_iter().cloned()
                .map(Move::from_byte)
                .map(|mv| mv.map_err(|_| ParseInputError))
        })
        .collect::<Result<_, _>>()?;

    Ok((map, moves))
}

// fn print_map(map: &[Vec<u8>]) {
//     for line in map {
//         for &cell in line {
//             print!("{}", cell as char);
//         }
//         println!();
//     }
// }

#[derive(Debug)]
struct RobotNotFoundError;

fn get_robot_pos(map: &[Vec<u8>]) -> Result<(i32, i32), RobotNotFoundError> {
    map.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, &c)| ((x, y), c)))
        .find(|(_, c)| c == &b'@')
        .map(|((x, y), _)| (x as i32, y as i32))
        .ok_or(RobotNotFoundError)
}

fn map_get(map: &[Vec<u8>], pos: (i32, i32)) -> Option<&u8> {
    let (x, y) = pos;

    map.get(y as usize).and_then(|l| l.get(x as usize))
}

fn map_get_mut(map: &mut [Vec<u8>], pos: (i32, i32)) -> Option<&mut u8> {
    let (x, y) = pos;

    map.get_mut(y as usize).and_then(|l| l.get_mut(x as usize))
}

fn execute_move(mut map: Vec<Vec<u8>>, robot_pos: (i32, i32), mv: Move) -> (Vec<Vec<u8>>, (i32, i32)) {
    let (dx, dy) = mv.to_dir();
    let (mut robot_x, mut robot_y) = robot_pos;
    let (mut next_x, mut next_y) = (robot_x + dx, robot_y + dy);

    while let Some(cell) = map_get(&map, (next_x, next_y)) {
        if cell == &b'#' {
            return (map, robot_pos);
        }

        if cell == &b'.' {
            *map_get_mut(&mut map, (next_x, next_y)).unwrap() = b'O';
            *map_get_mut(&mut map, (robot_x, robot_y)).unwrap() = b'.';

            robot_x += dx;
            robot_y += dy;
            *map_get_mut(&mut map, (robot_x, robot_y)).unwrap() = b'@';

            break;
        }

        next_x += dx;
        next_y += dy;
    }

    return (map, (robot_x, robot_y));
}

fn execute_moves(map: Vec<Vec<u8>>, moves: &[Move]) -> Result<Vec<Vec<u8>>, RobotNotFoundError> {
    let robot_pos = get_robot_pos(&map)?;
    
    let (new_map, _) = moves.into_iter().cloned()
        .fold((map, robot_pos), |(map, robot_pos), mv| execute_move(map, robot_pos, mv));

    Ok(new_map)
}

fn get_gps_sum(map: &[Vec<u8>]) -> usize {
    map.into_iter().enumerate()
        .flat_map(|(y, l)| l.into_iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter(|(_, &c)| c == b'O' || c == b'[')
        .map(|((x, y), _)| y * 100 + x)
        .sum()
}

#[derive(Debug)]
struct UnrecognizedCellError;

fn to_wide_cell(c: u8) -> Result<[u8; 2], UnrecognizedCellError> {
    match c {
        b'#' | b'.' => Ok([c, c]),
        b'O' => Ok([b'[', b']']),
        b'@' => Ok([b'@', b'.']),
        _ => Err(UnrecognizedCellError),
    }
}

fn to_wide_map(map: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, UnrecognizedCellError> {
    map.into_iter()
        .map(|l| {
            l.into_iter()
                .map(|c| to_wide_cell(c))
                .collect::<Result<Vec<_>, _>>()
                .map(|v| v.into_iter().flat_map(|arr| arr.into_iter()).collect())
        })
        .collect()
}

fn move_obstacle_wide(map: &mut Vec<Vec<u8>>, pos: (i32, i32), mv: Move) -> bool {
    let (x, y) = pos;

    match mv {
        Move::Up | Move::Down => {
            let dy = mv.to_dir().1;
            if let (Some(&cell1), Some(&cell2)) = (map_get(map, (x, y + dy)), map_get(map, (x + 1, y + dy))) {
                if cell1 == b'#' || cell2 == b'#' {
                    return false;
                } else if cell1 == b'[' && !move_obstacle_wide(map, (x, y + dy), mv) {
                    return false;
                } else if cell1 == b']' && !move_obstacle_wide(map, (x - 1, y + dy), mv) {
                    return false;
                } else if cell2 == b'[' && !move_obstacle_wide(map, (x + 1, y + dy), mv) {
                    return false;
                } else {
                    *map_get_mut(map, (x, y + dy)).unwrap() = b'[';
                    *map_get_mut(map, (x + 1, y + dy)).unwrap() = b']';
                    *map_get_mut(map, (x, y)).unwrap() = b'.';
                    *map_get_mut(map, (x + 1, y)).unwrap() = b'.';

                    return true;
                }
            } else {
                return false;
            }
        },
        Move::Left => {
            if let Some(&cell) = map_get(map, (x - 1, y)) {
                if cell == b'#' {
                    return false;
                }
                if cell == b']' && !move_obstacle_wide(map, (x - 2, y), mv) {
                    return false;
                }
                
                *map_get_mut(map, (x - 1, y)).unwrap() = b'[';
                *map_get_mut(map, (x, y)).unwrap() = b']';
                *map_get_mut(map, (x + 1, y)).unwrap() = b'.';

                return true;
            } else {
                return false;
            }
        },
        Move::Right => {
            if let Some(&cell) = map_get(map, (x + 2, y)) {
                if cell == b'#' {
                    return false;
                }
                if cell == b'[' && !move_obstacle_wide(map, (x + 2, y), mv) {
                    return false;
                }

                *map_get_mut(map, (x + 2, y)).unwrap() = b']';
                *map_get_mut(map, (x + 1, y)).unwrap() = b'[';
                *map_get_mut(map, (x, y)).unwrap() = b'.';

                return true;
            } else {
                return false;
            }
        },
    }
}

fn execute_move_wide(mut map: Vec<Vec<u8>>, robot_pos: (i32, i32), mv: Move) -> (Vec<Vec<u8>>, (i32, i32)) {
    let (dx, dy) = mv.to_dir();
    let (mut robot_x, mut robot_y) = robot_pos;

    if let Some(&cell) = map_get(&map, (robot_x + dx, robot_y + dy)) {
        if cell == b'[' {
            let mut new_map = map.clone();
            if move_obstacle_wide(&mut new_map, (robot_x + dx, robot_y + dy), mv) {
                *map_get_mut(&mut new_map, (robot_x, robot_y)).unwrap() = b'.';
                
                robot_x += dx;
                robot_y += dy;
                *map_get_mut(&mut new_map, (robot_x, robot_y)).unwrap() = b'@';
                map = new_map;
            }
        } else if cell == b']' {
            let mut new_map = map.clone();
            if move_obstacle_wide(&mut new_map, (robot_x + dx - 1, robot_y + dy), mv) {
                *map_get_mut(&mut new_map, (robot_x, robot_y)).unwrap() = b'.';
                
                robot_x += dx;
                robot_y += dy;
                *map_get_mut(&mut new_map, (robot_x, robot_y)).unwrap() = b'@';
                map = new_map;
            }
        } else if cell == b'.' {
            *map_get_mut(&mut map, (robot_x, robot_y)).unwrap() = b'.';
                
            robot_x += dx;
            robot_y += dy;
            *map_get_mut(&mut map, (robot_x, robot_y)).unwrap() = b'@';
        }
    }

    return (map, (robot_x, robot_y));
}

fn execute_moves_wide(map: Vec<Vec<u8>>, moves: &[Move]) -> Result<Vec<Vec<u8>>, RobotNotFoundError> {
    let robot_pos = get_robot_pos(&map)?;
    
    let (new_map, _) = moves.into_iter().cloned()
        .fold((map, robot_pos), |(map, robot_pos), mv| execute_move_wide(map, robot_pos, mv));

    Ok(new_map)
}

fn main() {
    let (map, moves) = read_map_and_moves().expect("Failed to parse input");

    let new_map = execute_moves(map.clone(), &moves).expect("Robot not found");
    let result1 = get_gps_sum(&new_map);

    let wide_map = to_wide_map(map).expect("Unknown cell detected");
    let new_wide_map = execute_moves_wide(wide_map, &moves).expect("Robot not found");
    let result2 = get_gps_sum(&new_wide_map);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

use utils::read_byte_matrix;

const EMPTY: u8 = 0;
const VISITED_UP: u8 = 1 << 0;
const VISITED_RIGHT: u8 = 1 << 1;
const VISITED_DOWN: u8 = 1 << 2;
const VISITED_LEFT: u8 = 1 << 3;
const OBSTACLE: u8 = 1 << 4;
const START: u8 = 1 << 5;

fn to_area_byte(cell: u8) -> u8 {
    match cell {
        b'.' => EMPTY,
        b'#' => OBSTACLE,
        b'^' => START,
        _ => panic!("Invalid cell")
    }
}

fn convert_area(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    return matrix.iter()
        .map(|line| line.iter()
            .map(|&b| to_area_byte(b))
            .collect())
        .collect()
}

fn get_start(area: &[Vec<u8>]) -> (i32, i32) {
    return area.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, v)| (x, y, v)))
        .find(|(_, _, &v)| v == START)
        .map(|(x, y, _)| (x as i32, y as i32))
        .unwrap()
}

fn in_bounds(area: &[Vec<u8>], x: i32, y: i32) -> bool {
    let (width, height) = (area[0].len() as i32, area.len() as i32);

    return 0 <= x && x < width && 0 <= y && y < height;
}

fn area_move(area: &[Vec<u8>], x: i32, y: i32, dx: i32, dy: i32) -> (i32, i32, i32, i32, bool) {
    if area[(y + dy) as usize][(x + dx) as usize] == OBSTACLE {
        (x, y, -dy, dx, true)
    } else {
        (x + dx, y + dy, dx, dy, false)
    }
}

fn direction_to_bit(dx: i32, dy: i32) -> u8 {
    match (dx, dy) {
        (0, -1) => VISITED_UP,
        (1, 0) => VISITED_RIGHT,
        (0, 1) => VISITED_DOWN,
        (-1, 0) => VISITED_LEFT,
        _ => panic!("Invalid direction"),
    }
}

fn calculate_path(area: &mut [Vec<u8>]) -> i32 {
    let mut count = 1;
    let (mut x, mut y) = get_start(area);
    area[y as usize][x as usize] = direction_to_bit(0, -1);

    let (mut dx, mut dy) = (0, -1);  // Up
    while in_bounds(area, x + dx, y + dy) {
        (x, y, dx, dy, _) = area_move(area, x, y, dx, dy);

        if area[y as usize][x as usize] == EMPTY {
            count += 1;
            area[y as usize][x as usize] = direction_to_bit(dx, dy);
        } else {
            if area[y as usize][x as usize] & direction_to_bit(dx, dy) != 0 {
                return -1;
            }
            area[y as usize][x as usize] |= direction_to_bit(dx, dy);
        }
    }

    return count;
}

fn is_visited(cell: u8) -> bool {
    cell & 0b1111 != 0
}

fn adjacent_to_visited(area: &[Vec<u8>], x: i32, y: i32) -> bool {
    [(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)].into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(x, y)| in_bounds(area, x, y))
        .any(|(x, y)| is_visited(area[y as usize][x as usize]))
} 

fn count_cycles(area: &[Vec<u8>], travelled_area: &[Vec<u8>]) -> i32 {
    let mut count = 0;

    for y in 0..area.len() as i32 {
        for x in 0..area[y as usize].len() as i32 {
            if area[y as usize][x as usize] != EMPTY || !adjacent_to_visited(travelled_area, x, y) {
                continue;
            }

            let mut temp_area: Vec<Vec<u8>> = area.iter().map(|x| x.clone()).collect();
            temp_area[y as usize][x as usize] = OBSTACLE;
            count += (calculate_path(&mut temp_area) == -1) as i32; 
        }
    }

    return count;
}

fn main() {
    let area = convert_area(&read_byte_matrix());
    let mut travelled_area =  area.clone();

    let result1 = calculate_path(&mut travelled_area);
    let result2 = count_cycles(&area, &travelled_area);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

use utils::read_byte_matrix;

fn get_start(area: &[Vec<u8>]) -> (i32, i32) {
    return area.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, v)| (x, y, v)))
        .find(|(_, _, &v)| v == b'^')
        .map(|(x, y, _)| (x as i32, y as i32))
        .unwrap()
}

fn in_bounds(area: &[Vec<u8>], x: i32, y: i32) -> bool {
    let (width, height) = (area[0].len() as i32, area.len() as i32);

    return 0 <= x && x < width && 0 <= y && y < height;
}

fn area_move(area: &[Vec<u8>], x: i32, y: i32, dx: i32, dy: i32) -> (i32, i32, i32, i32, bool) {
    if area[(y + dy) as usize][(x + dx) as usize] == b'#' {
        (x, y, -dy, dx, true)
    } else {
        (x + dx, y + dy, dx, dy, false)
    }
}

fn direction_to_bit(dx: i32, dy: i32) -> u8 {
    match (dx, dy) {
        (0, -1) => 0b0001,
        (1, 0) => 0b0010,
        (0, 1) => 0b0100,
        (-1, 0) => 0b1000,
        _ => 0 
    }
}

fn calculate_path(area: &mut [Vec<u8>]) -> i32 {
    let mut count = 1;
    let (mut x, mut y) = get_start(area);
    area[y as usize][x as usize] = b'X';

    let (mut dx, mut dy) = (0, -1);  // Up
    while in_bounds(area, x + dx, y + dy) {
        (x, y, dx, dy, _) = area_move(area, x, y, dx, dy);

        if area[y as usize][x as usize] == b'.' {
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

fn count_cycles(area: &mut [Vec<u8>]) -> i32 {
    let mut count = 0;

    for y in 0..area.len() as i32 {
        for x in 0..area[y as usize].len() as i32 {
            if area[y as usize][x as usize] != b'.' {
                continue;
            }

            area[y as usize][x as usize] = b'#';
            let mut temp_area: Vec<Vec<u8>> = area.iter().map(|x| x.clone()).collect();
            count += (calculate_path(&mut temp_area) == -1) as i32; 
            area[y as usize][x as usize] = b'.';
        }
    }

    return count;
}

fn main() {
    let mut matrix = read_byte_matrix();

    let result1 = calculate_path(&mut matrix.clone());
    let result2 = count_cycles(&mut matrix);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

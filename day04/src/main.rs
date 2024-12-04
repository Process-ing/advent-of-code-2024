use utils::read_byte_matrix;

fn count_xmas_at(mat: &[Vec<u8>], x: i32, y: i32) -> usize {
    fn check_suffix(mat: &[Vec<u8>], x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let (width, height) = (mat[0].len() as i32, mat.len() as i32);
        
        if x + 3 * dx < 0 || x + 3 * dx >= width {
            return false;
        }
        if y + 3 * dy < 0 || y + 3 * dy >= height {
            return false;
        }

        let (mut i, mut j) = (x + dx, y + dy);
        for &c in "MAS".as_bytes() {  // The first character has already been checked
            if mat[j as usize][i as usize] != c {
                return false;
            }

            i += dx;
            j += dy;
        }

        return true;
    }

    const DIRECTIONS: [(i32, i32); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

    if mat[y as usize][x as usize] != b'X' {
        return 0;
    }

    let count = DIRECTIONS.into_iter()
        .map(|(dx, dy)| check_suffix(mat, x, y, dx, dy) as usize)
        .sum();

    return count;
}

fn count_xmas(mat: &[Vec<u8>]) -> usize {
    let (width, height) = (mat[0].len() as i32, mat.len() as i32);

    return (0..width)
        .flat_map(move |x| (0..height).map(move |y| (x, y)))
        .map(|(x, y)| count_xmas_at(mat, x, y))
        .sum();
}

fn count_x_mas_at(mat: &[Vec<u8>], x: i32, y: i32) -> usize {
    fn check_corners(mat: &[Vec<u8>], x: i32, y: i32, dx1: i32, dy1: i32) -> bool {
        let rotations = [(dx1, dy1), (-dy1, dx1), (-dx1, -dy1), (dy1, -dx1)];
        
        for ((dx2, dy2), &c) in rotations.into_iter().zip("SMMS".as_bytes()) {
            if mat[(y + dy2) as usize][(x + dx2) as usize] != c {
                return false;
            }
        }

        return true;
    }
    
    const CORNERS: [(i32, i32); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
        
    if mat[y as usize][x as usize] != b'A' {
        return 0;
    }

    let count = CORNERS.into_iter()
        .map(|(dx1, dy1)| check_corners(mat, x, y, dx1, dy1) as usize)
        .sum();

    return count;
}

fn count_x_mas(mat: &[Vec<u8>]) -> usize {
    let (width, height) = (mat[0].len() as i32, mat.len() as i32);

    return (1..width - 1)
        .flat_map(move |x| (1..height - 1).map(move |y| (x, y)))
        .map(|(x, y)| count_x_mas_at(mat, x, y))
        .sum();
}

fn main() {
    let matrix = read_byte_matrix();

    let xmas_count = count_xmas(&matrix);
    let x_mas_count = count_x_mas(&matrix);

    println!("Part 1 result: {xmas_count}");
    println!("Part 2 result: {x_mas_count}");
}

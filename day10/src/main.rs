use utils::read_byte_matrix;

fn in_bounds(map: &[Vec<u8>], (x, y): (i32, i32)) -> bool {
    let (width, height) = (map[0].len() as i32, map.len() as i32);

    return 0 <= x && x < width && 0 <= y && y < height;
}

fn get_adjs<'a>(map: &'a [Vec<u8>], (x, y): (i32, i32)) -> impl Iterator<Item = (i32, i32)> + 'a {
    return [(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
        .filter(|&pos| in_bounds(map, pos));
}

fn trailhead_score_dfs(map: &[Vec<u8>], (x, y): (i32, i32), visited: &mut [Vec<bool>]) -> usize {
    visited[y as usize][x as usize] = true;
    let curr_level = map[y as usize][x as usize];
    if curr_level == b'9' {
        return 1;
    }

    let adjs: Vec<(i32, i32)> = get_adjs(map, (x, y))
    .   filter(|&(x, y)| map[y as usize][x as usize] == curr_level + 1 && !visited[y as usize][x as usize])
        .collect();
    let count = adjs.into_iter()
        .map(|pos| trailhead_score_dfs(map, pos, visited))
        .sum();

    return count;   
}

fn new_visited(map: &[Vec<u8>]) -> Vec<Vec<bool>> {
    return map.iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();
}

fn get_trailhead_scores(map: &[Vec<u8>]) -> usize {
    return map.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, &c)| ((x as i32, y as i32), c)))
        .filter(|(_, c)| *c == b'0')
        .map(|(pos, _)| trailhead_score_dfs(map, pos, &mut new_visited(map)))
        .sum();
}

fn trailhead_rating_dfs(map: &[Vec<u8>], (x, y): (i32, i32)) -> usize {
    let curr_level = map[y as usize][x as usize];
    if curr_level == b'9' {
        return 1;
    }

    let adjs: Vec<(i32, i32)> = get_adjs(map, (x, y))
    .   filter(|&(x, y)| map[y as usize][x as usize] == curr_level + 1)
        .collect();
    let count = adjs.into_iter()
        .map(|pos| trailhead_rating_dfs(map, pos))
        .sum();

    return count;   
}

fn get_trailhead_ratings(map: &[Vec<u8>]) -> usize {
    return map.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, &c)| ((x as i32, y as i32), c)))
        .filter(|(_, c)| *c == b'0')
        .map(|(pos, _)| trailhead_rating_dfs(map, pos))
        .sum();
}

fn main() {
    let map = read_byte_matrix();

    let result1 = get_trailhead_scores(&map);
    let result2 = get_trailhead_ratings(&map);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

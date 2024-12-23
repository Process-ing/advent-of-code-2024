use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{io, usize};
use std::io::Read;

#[derive(Debug)]
struct ParseMatrixError;

fn read_byte_matrix() -> Result<Vec<Vec<u8>>, ParseMatrixError> {
    let mut stdin = io::stdin();
    let mut text = String::new();
    stdin.read_to_string(&mut text).map_err(|_| ParseMatrixError)?;

    let matrix = text.lines()
        .map(|line| line.as_bytes().into_iter().cloned().collect())
        .collect();

    Ok(matrix)
}

fn get_start(map: &[Vec<u8>]) -> Option<(i32, i32)> {
    map.into_iter().enumerate()
        .flat_map(|(y, l)| l.into_iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .find(|(_, c)| **c == b'S')
        .map(|((x, y), _)| (x as i32, y as i32))
}

fn get_end(map: &[Vec<u8>]) -> Option<(i32, i32)> {
    map.into_iter().enumerate()
        .flat_map(|(y, l)| l.into_iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .find(|(_, c)| **c == b'E')
        .map(|((x, y), _)| (x as i32, y as i32))
}

fn new_scores(map: &[Vec<u8>]) -> Vec<Vec<[usize; 4]>> {
    map.iter().map(
        |l| l.iter().map(|_| [usize::MAX; 4]).collect()
    ).collect()
}

fn new_preds(map: &[Vec<u8>]) -> Vec<Vec<[u8; 4]>> {
    map.iter().map(
        |line| line.iter().map(|_| [0; 4]).collect()
    ).collect()
}

#[derive(Debug)]
struct  StartNotFoundError;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    pos: (i32, i32),
    dir: u8,
    prev_dir: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn map_get<T>(map: &[Vec<T>], pos: (i32, i32)) -> Option<&T> {
    let (x, y) = pos;

    map.get(y as usize).and_then(|line| line.get(x as usize))
}

fn map_get_mut<T>(map: &mut [Vec<T>], pos: (i32, i32)) -> Option<&mut T> {
    let (x, y) = pos;

    map.get_mut(y as usize).and_then(|line| line.get_mut(x as usize))
}

fn map2_get<T>(map: &[Vec<[T; 4]>], pos: (i32, i32), dir: u8) -> Option<&T> {
    let (x, y) = pos;

    map.get(y as usize)
        .and_then(|line| line.get(x as usize))
        .map(|cell| &cell[dir as usize])
}

fn map2_get_mut<T>(map: &mut [Vec<[T; 4]>], pos: (i32, i32), dir: u8) -> Option<&mut T> {
    let (x, y) = pos;

    map.get_mut(y as usize)
        .and_then(|line| line.get_mut(x as usize))
        .map(|cell| &mut cell[dir as usize])
}

fn advance(pos: (i32, i32), dir: u8) -> (i32, i32) {
    let (x, y) = pos;

    match dir {
        0 => (x + 1, y),
        1 => (x, y - 1),
        2 => (x - 1, y),
        3 => (x, y + 1),
        _ => panic!("Invalid direction")
    }
}

fn rotate_left(dir: u8) -> u8 {
    (dir + 1) % 4
}

fn rotate_right(dir: u8) -> u8 {
    (dir - 1) % 4
}

fn invert(dir: u8) -> u8 {
    (dir + 2) % 4
}

fn get_lowest_score(map: &[Vec<u8>], end: (i32, i32)) -> Result<(usize, Vec<Vec<[u8; 4]>>), StartNotFoundError> {
    let mut scores = new_scores(map);
    let mut preds = new_preds(map);

    let start = get_start(map).ok_or(StartNotFoundError)?;
    let mut heap = BinaryHeap::from([State {
        cost: 0,
        pos: start,
        dir: 0,
        prev_dir: 0,
    }]);

    let mut end_score = usize::MAX;
    while let Some(state) = heap.pop() {
        let State { cost, pos, dir, prev_dir } = state;

        if cost > end_score {
            break;
        }

        if map2_get(&scores, pos, dir).is_some_and(|score| *score < cost) {
            continue;
        }

        *map2_get_mut(&mut scores, pos, dir).unwrap() = cost;
        *map2_get_mut(&mut preds, pos, dir).unwrap() |= 1 << prev_dir;

        if pos == end {
            end_score = end_score.min(cost);
        }

        if map_get(map, pos).is_some_and(|c| *c != b'#') {
            heap.push(State { cost: cost + 1, pos: advance(pos, dir), dir, prev_dir: dir});
        }
        heap.push(State { cost: cost + 1000, pos, dir: rotate_left(dir), prev_dir: dir});
        heap.push(State { cost: cost + 1000, pos, dir: rotate_right(dir), prev_dir: dir});
    }

    *map2_get_mut(&mut preds, start, 0).unwrap() ^= 1 << 0;  // Unset initial direction
    Ok((end_score, preds))
} 

fn new_paths(preds: &[Vec<[u8; 4]>]) -> Vec<Vec<u8>> {
    preds.iter()
        .map(|l| l.iter().map(|_| 0).collect())
        .collect()
}

fn reconstruct_paths(preds: &[Vec<[u8; 4]>], end: (i32, i32)) -> Vec<Vec<u8>> {
    let mut paths = new_paths(preds);

    for dir in 0..4 {
        path_dfs(preds, end, dir, &mut paths);
    }

    return paths;
}

fn path_dfs(preds: &[Vec<[u8; 4]>], root: (i32, i32), dir: u8, paths: &mut [Vec<u8>]) {
    let cell = map_get_mut(paths, root).unwrap();
    if *cell & (1 << dir) != 0 {
        return;
    }

    *cell |= 1 << dir;
    let pred = map2_get(preds, root, dir).unwrap();
    for prev_dir in 0..4 {
        if pred & (1 << prev_dir) != 0 {
            if dir == prev_dir {
                path_dfs(preds, advance(root, invert(dir)), dir, paths);
            } else {
                path_dfs(preds, root, prev_dir, paths);
            }
        }
    }
}

fn main() {
    let map = read_byte_matrix().expect("Failed to parse byte matrix");

    let end = get_end(&map).expect("Failed to obtain end");
    let (result1, preds) = get_lowest_score(&map, end).expect("Failed to obtain lowest score");
    let paths = reconstruct_paths(&preds, end);

    let result2 = paths.into_iter().flat_map(|l| l.into_iter()).filter(|&c| c != 0).count();

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

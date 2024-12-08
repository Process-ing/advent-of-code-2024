use utils::read_byte_matrix;
use std::collections::HashMap;

fn get_frequency_map(map: &[Vec<u8>]) -> HashMap<u8, Vec<(i32, i32)>> {
    let mut res = HashMap::new();

    for (y, l) in map.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            if c != b'.' {
                res.entry(c)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }

    return res;
}

fn map_get<T>(map: &[Vec<T>], x: i32, y: i32) -> Option<T> where T: Copy {
    if x < 0 || y < 0 {
        None
    } else {
        map.get(y as usize).and_then(|l| l.get(x as usize)).map(|c| *c)
    }
}

fn count_antinodes(map: &[Vec<u8>], frequency_map: &HashMap<u8, Vec<(i32, i32)>>) -> usize {
    let mut count = 0;
    let mut is_antinode: Vec<Vec<bool>> = map.iter().map(|l| l.iter().map(|_| false).collect()).collect();

    for (_, points) in frequency_map.iter() {
        let pairs = points.iter().enumerate()
            .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| (p1, p2)));

        for (&(x1, y1), &(x2, y2)) in pairs {
            let (dx, dy) = (x2 - x1, y2 - y1);
            
            for (x, y) in [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)] {
                if map_get(&is_antinode, x, y).is_some_and(|antinode| !antinode) {
                    count += 1;
                    is_antinode[y as usize][x as usize] = true;
                }
            }
        }
    }

    return count;
}

fn get_line_antinodes<'a>(map: &'a [Vec<u8>], (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> impl Iterator<Item = (i32, i32)> + 'a {
    let (dx, dy) = (x2 - x1, y2 - y1);

    let it1 = (0..)
        .map(move |i| (x1 - i * dx, y1 - i * dy))
        .take_while(|&(x, y)| map_get(map, x, y).is_some());

    let it2 = (0..)
        .map(move |i| (x2 + i * dx, y2 + i * dy))
        .take_while(|&(x, y)| map_get(map, x, y).is_some());

    return it1.chain(it2);
}

fn count_antinodes2(map: &[Vec<u8>], frequency_map: &HashMap<u8, Vec<(i32, i32)>>) -> usize {
    let mut count = 0;
    let mut is_antinode: Vec<Vec<bool>> = map.iter().map(|l| l.iter().map(|_| false).collect()).collect();

    for (_, points) in frequency_map.iter() {
        let pairs = points.iter().enumerate()
            .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| (p1, p2)));

        for (&(x1, y1), &(x2, y2)) in pairs {
            for (x, y) in get_line_antinodes(map, (x1, y1), (x2, y2)) {
                if !is_antinode[y as usize][x as usize] {
                    count += 1;
                    is_antinode[y as usize][x as usize] = true;
                }
            }
        }
    }

    return count;
}

fn main() {
    let map = read_byte_matrix();
    let frequency_map = get_frequency_map(&map); 

    let result1 = count_antinodes(&map, &frequency_map);
    let result2 = count_antinodes2(&map, &frequency_map);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

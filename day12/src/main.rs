use utils::read_byte_matrix;

fn map_get(map: &[Vec<u8>], pos: (i32, i32)) -> Option<&u8> {
    let (x, y) = pos;

    return map.get(y as usize)
        .and_then(|l| l.get(x as usize));
}

fn get_adjs<'a>(map: &'a[Vec<u8>], pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + 'a {
    let (x, y) = pos;
    let cell = map_get(map, pos).unwrap();  

    return [(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
        .filter(move |&(x, y)| map_get(map, (x, y)).is_some_and(|other| other == cell));
}

fn new_visited(map: &[Vec<u8>]) -> Vec<Vec<bool>> {
    return map.iter().map(|l| l.iter().map(|_| false).collect()).collect();
}

fn is_visited(visited: &[Vec<bool>], pos: (i32, i32)) -> bool {
    let (x, y) = pos;

    return visited.get(y as usize)
        .and_then(|l| l.get(x as usize))
        .cloned() 
        .unwrap_or(false);
}

fn set_visited(visited: &mut [Vec<bool>], pos: (i32, i32)) {
    let (x, y) = pos;

    visited[y as usize][x as usize] = true;
}

fn region_dfs1(map: &[Vec<u8>], root: (i32, i32), visited: &mut [Vec<bool>]) -> (usize, usize) {
    let adjs: Vec<(i32, i32)> = get_adjs(map, root).collect();

    set_visited(visited, root);  

    let mut perimeter = 4 - adjs.len();
    let mut area = 1;

    for adj in adjs {
        if !is_visited(visited, adj) {
            let (adj_perimeter, adj_area) = region_dfs1(map, adj, visited);
            perimeter += adj_perimeter;
            area += adj_area;
        }
    }

    return (perimeter, area);
}

fn get_price1(map: &[Vec<u8>]) -> usize {
    let mut visited = new_visited(map);

    let range  = map.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, _)| (x as i32, y as i32)));

    let mut price = 0;
    for pos in range {
        if !is_visited(&visited, pos) {
            let (perimeter, area) = region_dfs1(map, pos, &mut visited);
            price += perimeter * area;
        }
    }

    return price;
}

fn count_corners(map: &[Vec<u8>], pos: (i32, i32)) -> usize {
    let (x, y) = pos;
    let cell = map_get(map, pos);

    return [(1, 1), (-1, 1), (-1, -1), (1, -1)].into_iter()
        .map(|(dx, dy)| (map_get(map, (x + dx, y + dy)), map_get(map, (x, y + dy)), map_get(map, (x + dx, y))))
        .filter(|&(diag, side1, side2)| (diag != cell && side1 == cell && side2 == cell) || (side1 != cell && side2 != cell))
        .count(); 
}

fn region_dfs2(map: &[Vec<u8>], root: (i32, i32), visited: &mut [Vec<bool>]) -> (usize, usize) {
    let adjs: Vec<(i32, i32)> = get_adjs(map, root).collect();

    set_visited(visited, root);  

    let mut sides = count_corners(map, root);
    let mut area = 1;

    for adj in adjs {
        if !is_visited(visited, adj) {
            let (adj_sides, adj_area) = region_dfs2(map, adj, visited);
            sides += adj_sides;
            area += adj_area;
        }
    }

    return (sides, area);
}

fn get_price2(map: &[Vec<u8>]) -> usize {
    let mut visited = new_visited(map);

    let range  = map.iter().enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, _)| (x as i32, y as i32)));

    let mut price = 0;
    for pos in range {
        if !is_visited(&visited, pos) {
            let (sides, area) = region_dfs2(map, pos, &mut visited);
            price += sides * area;
        }
    }

    return price;
}

fn main() {
    let map = read_byte_matrix();

    let result1 = get_price1(&map);
    let result2 = get_price2(&map);

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

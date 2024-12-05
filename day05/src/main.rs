use std::io::{self, BufRead};
use std::collections::HashMap;

struct Vertex {
    indegree: i32,
    adj: Vec<i32>,
}

impl Vertex {
    fn new() -> Self {
        Self {
            indegree: 0,
            adj: vec![],
        }        
    }
}

fn read_order() -> Vec<(i32, i32)> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let order = lines.take_while(|line| !line.is_empty())
        .map(|line|
            line.split('|')
                .map(|word| word.parse().unwrap())
                .collect::<Vec<i32>>()
        )
        .map(|edge| (edge[0], edge[1]))
        .collect();

    return order;
}

fn read_updates() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let updates = lines
        .map(|line|
            line.split(',')
                .map(|word| word.parse().unwrap())
                .collect::<Vec<i32>>()
        )
        .collect();

    return updates;
}

fn to_graph(order: &[(i32, i32)], update: &[i32]) -> HashMap<i32, Vertex> {
    let mut graph = HashMap::new();

    for &page in update {
        graph.insert(page, Vertex::new());
    }

    for &(before, after) in order {
        if graph.contains_key(&before) && graph.contains_key(&after) {
            graph.entry(before).and_modify(|v| v.adj.push(after));
            graph.entry(after).and_modify(|v| v.indegree += 1);
        }
    }

    return graph;
}

fn is_ordered(update: &[i32], order: &[(i32, i32)]) -> bool {
    let mut graph = to_graph(order, update);

    for page in update {
        let vertex = graph.get(page).unwrap();
        if vertex.indegree > 0 {
            return false;
        }

        let adj = vertex.adj.clone();
        for adj_page in adj {
            let neighbor = graph.get_mut(&adj_page).unwrap();
            neighbor.indegree -= 1;
        }
    }

    return true;
}

fn topsort(update: &[i32], order: &[(i32, i32)]) -> Vec<i32> {
    let mut graph = to_graph(order, update);
    let mut no_inc_vertices: Vec<i32> = graph.iter()
        .filter(|&(_, v)| v.indegree == 0)
        .map(|(&page, _)| page)
        .collect();

    let mut new_update = vec![];
    while let Some(page) = no_inc_vertices.pop() {
        new_update.push(page);

        let vertex = graph.get(&page).unwrap();
        let adj = vertex.adj.clone();

        for adj_page in adj {
            let neighbor = graph.get_mut(&adj_page).unwrap();
            neighbor.indegree -= 1;
            if neighbor.indegree == 0 {
                no_inc_vertices.push(adj_page);
            }
        }
    }

    return new_update;
}

fn middle_element(slice: &[i32]) -> i32 {
    slice[slice.len() / 2]
}

fn main() {
    let order = read_order();
    let updates = read_updates();

    let result1: i32 = updates.iter()
        .filter(|update| is_ordered(update, &order))
        .map(|update| middle_element(&update))
        .sum();
    let result2: i32 = updates.into_iter()
        .filter(|update| !is_ordered(update, &order))
        .map(|update| middle_element(&topsort(&update, &order)))
        .sum();


    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}

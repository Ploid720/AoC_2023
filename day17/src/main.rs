use std::{fs, time::Instant, collections::{BinaryHeap, HashMap}};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Node {
    x: usize,
    y: usize,
    dir: Direction,
    straight_count: usize
}

#[derive(Debug, Copy, Clone)]
struct SignedNode {
    x: isize,
    y: isize,
    dir: Direction,
    straight_count: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    node: Node,
    cost: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part_1(input: &String) -> Option<usize> {
    let straight_line_limit = 3;

    let sl_max_val = straight_line_limit - 1;

    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10) as usize)
            .collect())
        .collect();

    let w = mx.get(0)
        .map(|v| v.len())
        .unwrap_or(0);
    let h = mx.len();

    let start = (0, 0);
    let end = (w - 1, h - 1);

    let start_x = start.0;
    let start_y = start.1;
    let end_x = end.0;
    let end_y = end.1;

    let adj_map: HashMap<Node, Vec<(Node, usize)>> =(0..w)
        .flat_map(|x| (0..h).map(move |y| (x, y)))
        .flat_map(|(x, y)| vec!(
            Direction::Up, 
            Direction::Down, 
            Direction::Left, 
            Direction::Right)
            .into_iter()
            .map(move |dir| (x, y, dir)))
        .flat_map(|(x, y, dir)| (0..straight_line_limit)
            .map(move |slc| (x, y, dir, slc)))
        .map(|(x, y, dir, slc)| {
            let xi = x as isize;
            let yi = y as isize;

            let nbs = match dir {
                Direction::Up if slc < sl_max_val => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,    straight_count: slc + 1},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),
                Direction::Up => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),
                Direction::Down if slc < sl_max_val => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down,  straight_count: slc + 1},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),
                Direction::Down => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),
                Direction::Left if slc < sl_max_val => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left, straight_count: slc + 1},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                ),
                Direction::Left => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                ),
                Direction::Right if slc < sl_max_val => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: slc + 1},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                ),
                Direction::Right => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                )
            }
                .into_iter()
                .filter_map(|node| {
                    if (node.x < 0) || (node.y < 0) {
                        return None;
                    }
                    let xu = node.x as usize;
                    let yu = node.y as usize;
                    if (xu >= w) || (yu >= h) {
                        return None;
                    }
                    return Some((Node{x: xu as usize, y: yu as usize, dir: node.dir, straight_count: node.straight_count}, mx[yu][xu]));
                })
                .collect();

            return (Node{x, y, dir, straight_count: slc}, nbs);
        })
        .collect();

    let mut heap = BinaryHeap::new();
    let mut dists: HashMap<Node, usize> = HashMap::new();

    for dir in vec!(
        Direction::Up, 
        Direction::Down, 
        Direction::Left, 
        Direction::Right) {
        let node = Node{x: start_x, y: start_y, dir, straight_count: 0};
        heap.push(State{node, cost: 0});
        dists.insert(node, 0);
    }

    let empty_nb_vec = vec!();
    while let Some(state) = heap.pop() {
        let node = state.node;
        let cost = state.cost;

        if dists.get(&node)
            .map(|v| cost > *v)
            .unwrap_or(false) {
            continue;
        }

        for neighbor in adj_map.get(&node).unwrap_or_else(|| &empty_nb_vec) {
            let next_node: Node = neighbor.0;
            let next_cost: usize = cost + neighbor.1;

            if dists.get(&next_node)
                .map(|v| next_cost < *v)
                .unwrap_or(true) {

                heap.push(State{node: next_node, cost: next_cost});
                dists.insert(next_node, next_cost);
            }
        }
    }

    return (0..straight_line_limit)
        .flat_map(|stl| vec!(
            Direction::Up, 
            Direction::Down, 
            Direction::Left, 
            Direction::Right)
            .into_iter()
            .map(move |dir| (dir, stl)))
        .filter_map(|(dir, stl)| dists.get(&Node{x: end_x, y: end_y, dir, straight_count: stl}))
        .map(|res| *res)
        .min();
}

fn solve_part_2(input: &String) -> Option<usize> {
    let straight_line_lower_limit = 4;
    let straight_line_upper_limit = 10;

    let sl_min_val = straight_line_lower_limit - 1;
    let sl_max_val = straight_line_upper_limit - 1;

    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(10) as usize)
            .collect())
        .collect();

    let w = mx.get(0)
        .map(|v| v.len())
        .unwrap_or(0);
    let h = mx.len();

    let start = (0, 0);
    let end = (w - 1, h - 1);

    let start_x = start.0;
    let start_y = start.1;
    let end_x = end.0;
    let end_y = end.1;

    let adj_map: HashMap<Node, Vec<(Node, usize)>> =(0..w)
        .flat_map(|x| (0..h).map(move |y| (x, y)))
        .flat_map(|(x, y)| vec!(
            Direction::Up, 
            Direction::Down, 
            Direction::Left, 
            Direction::Right)
            .into_iter()
            .map(move |dir| (x, y, dir)))
        .flat_map(|(x, y, dir)| (0..straight_line_upper_limit)
            .map(move |slc| (x, y, dir, slc)))
        .map(|(x, y, dir, slc)| {
            let xi = x as isize;
            let yi = y as isize;

            let nbs = match dir {
                Direction::Up if slc < sl_min_val => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,    straight_count: slc + 1},
                ),
                Direction::Up if slc < sl_max_val => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,    straight_count: slc + 1},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),
                Direction::Up => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),

                Direction::Down if slc < sl_min_val => vec!(
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down,  straight_count: slc + 1},
                ),
                Direction::Down if slc < sl_max_val => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down,  straight_count: slc + 1},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),
                Direction::Down => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left,  straight_count: 0},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: 0}
                ),

                Direction::Left if slc < sl_min_val => vec!(
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left, straight_count: slc + 1},
                ),
                Direction::Left if slc < sl_max_val => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi - 1, y: yi    , dir: Direction::Left, straight_count: slc + 1},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                ),
                Direction::Left => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                ),

                Direction::Right if slc < sl_min_val => vec!(
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: slc + 1},
                ),
                Direction::Right if slc < sl_max_val => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi + 1, y: yi    , dir: Direction::Right, straight_count: slc + 1},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                ),
                Direction::Right => vec!(
                    SignedNode{x: xi    , y: yi - 1, dir: Direction::Up,   straight_count: 0},
                    SignedNode{x: xi    , y: yi + 1, dir: Direction::Down, straight_count: 0}
                )
            }
                .into_iter()
                .filter_map(|node| {
                    if (node.x < 0) || (node.y < 0) {
                        return None;
                    }
                    let xu = node.x as usize;
                    let yu = node.y as usize;
                    if (xu >= w) || (yu >= h) {
                        return None;
                    }
                    return Some((Node{x: xu as usize, y: yu as usize, dir: node.dir, straight_count: node.straight_count}, mx[yu][xu]));
                })
                .collect();

            return (Node{x, y, dir, straight_count: slc}, nbs);
        })
        .collect();

    let mut heap = BinaryHeap::new();
    let mut dists: HashMap<Node, usize> = HashMap::new();

    for dir in vec!(
        Direction::Up, 
        Direction::Down, 
        Direction::Left, 
        Direction::Right) {
        let node = Node{x: start_x, y: start_y, dir, straight_count: 0};
        heap.push(State{node, cost: 0});
        dists.insert(node, 0);
    }

    let empty_nb_vec = vec!();
    while let Some(state) = heap.pop() {
        let node = state.node;
        let cost = state.cost;

        if dists.get(&node)
            .map(|v| cost > *v)
            .unwrap_or(false) {
            continue;
        }

        for neighbor in adj_map.get(&node).unwrap_or_else(|| &empty_nb_vec) {
            let next_node: Node = neighbor.0;
            let next_cost: usize = cost + neighbor.1;

            if dists.get(&next_node)
                .map(|v| next_cost < *v)
                .unwrap_or(true) {

                heap.push(State{node: next_node, cost: next_cost});
                dists.insert(next_node, next_cost);
            }
        }
    }

    return (sl_min_val..straight_line_upper_limit)
        .flat_map(|stl| vec!(
            Direction::Up, 
            Direction::Down, 
            Direction::Left, 
            Direction::Right)
            .into_iter()
            .map(move |dir| (dir, stl)))
        .filter_map(|(dir, stl)| dists.get(&Node{x: end_x, y: end_y, dir, straight_count: stl}))
        .map(|res| *res)
        .min();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content);
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content);
    let t2 = inst2.elapsed();

    match part_1_res {
        Some(res) => println!("Part 1 result: {}, solved in {} ms", res, t1.as_secs_f64() * 1000.0),
        None => println!("No result found for part 1, solved in {} ms", t1.as_secs_f64() * 1000.0)
    }
    match part_2_res {
        Some(res) => println!("Part 2 result: {}, solved in {} ms", res, t2.as_secs_f64() * 1000.0),
        None => println!("No result found for part 2, solved in {} ms", t1.as_secs_f64() * 1000.0)
    }
}
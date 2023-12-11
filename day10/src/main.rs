use std::{fs, time::Instant, collections::{HashSet, VecDeque}};

#[derive(Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthToEastBend,
    NorthToWestBend,
    SouthToEastBend,
    SouthToWestBend,
    Ground,
    Start
}

fn solve_part_1(input: &String) -> i32 {
    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line
            .chars()
            .map(|x| match x {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NorthToEastBend,
                'J' => Tile::NorthToWestBend,
                '7' => Tile::SouthToWestBend,
                'F' => Tile::SouthToEastBend,
                'S' => Tile::Start,
                _ => Tile::Ground,
            })
            .collect())
        .collect();

    let start_point = mx.iter()
        .enumerate()
            .find_map(|(y, row)| row
                .iter()
                .enumerate()
                .find_map(|(x, v)| match v {
                    Tile::Start => Some((x, y)),
                    _ => None
                }))
            .expect("Input should contain starting point but doesn't");
    
    fn move_from(x: isize, y: isize, tile: &Tile) -> Vec<(isize, isize)> {
        return match tile {
            Tile::Vertical =>        [(x, y + 1), (x, y - 1)].iter().copied().collect(),
            Tile::Horizontal =>      [(x + 1, y), (x - 1, y)].iter().copied().collect(),
            Tile::NorthToEastBend => [(x, y - 1), (x + 1, y)].iter().copied().collect(),
            Tile::NorthToWestBend => [(x, y - 1), (x - 1, y)].iter().copied().collect(),
            Tile::SouthToWestBend => [(x, y + 1), (x - 1, y)].iter().copied().collect(),
            Tile::SouthToEastBend => [(x, y + 1), (x + 1, y)].iter().copied().collect(),
            Tile::Start => [].iter().copied().collect(),
            Tile::Ground => [].iter().copied().collect(),
        };
    }

    let mut queue: VecDeque<_> = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|v: &(isize, isize)| {
            let x = start_point.0 as isize + v.0;
            let y = start_point.1 as isize + v.1;
            if x < 0 || y < 0 || y as usize >= mx.len() || x as usize >= mx[y as usize].len() {
                return None;
            }

            let tile = &mx[y as usize][x as usize];
            if move_from(x, y, tile)
                .iter()
                .any(|(nx, ny)| *nx as usize == start_point.0 && *ny as usize == start_point.1) {
                return Some((x, y, tile, 1));
            }
            else {
                return None;
            }
        })
        .collect();

    let mut visited: HashSet<(isize, isize)> = queue.iter()
        .map(|(x, y, _tile, _dist)| (*x, *y))
        .chain([(start_point.0 as isize, start_point.1 as isize)].into_iter())
        .collect();

    let mut max_dist = 1;
    while !queue.is_empty()
    {
        let curr = queue.pop_front().unwrap();
        let curr_dist = curr.3;
        if curr_dist > max_dist {
            max_dist = curr_dist;
        }
        for (x, y) in move_from(curr.0, curr.1, curr.2) {
            let pos = (x, y);
            if visited.contains(&pos) {
                continue;
            }
            let tile = &mx[y as usize][x as usize];
            visited.insert(pos);
            queue.push_back((x, y, tile, curr_dist + 1));
        }
    }

    return max_dist;
}

fn solve_part_2(input: &String) -> usize {
    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line
            .chars()
            .map(|x| match x {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NorthToEastBend,
                'J' => Tile::NorthToWestBend,
                '7' => Tile::SouthToWestBend,
                'F' => Tile::SouthToEastBend,
                'S' => Tile::Start,
                _ => Tile::Ground,
            })
            .collect())
        .collect();

    let start_point = mx.iter()
        .enumerate()
            .find_map(|(y, row)| row
                .iter()
                .enumerate()
                .find_map(|(x, v)| match v {
                    Tile::Start => Some((x, y)),
                    _ => None
                }))
            .expect("Input should contain starting point but doesn't");
    
    fn move_from(x: isize, y: isize, tile: &Tile) -> Vec<(isize, isize)> {
        return match tile {
            Tile::Vertical =>        [(x, y + 1), (x, y - 1)].iter().copied().collect(),
            Tile::Horizontal =>      [(x + 1, y), (x - 1, y)].iter().copied().collect(),
            Tile::NorthToEastBend => [(x, y - 1), (x + 1, y)].iter().copied().collect(),
            Tile::NorthToWestBend => [(x, y - 1), (x - 1, y)].iter().copied().collect(),
            Tile::SouthToWestBend => [(x, y + 1), (x - 1, y)].iter().copied().collect(),
            Tile::SouthToEastBend => [(x, y + 1), (x + 1, y)].iter().copied().collect(),
            Tile::Start => [].iter().copied().collect(),
            Tile::Ground => [].iter().copied().collect(),
        };
    }

    let mut stack: Vec<_> = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|v: &(isize, isize)| {
            let x = start_point.0 as isize + v.0;
            let y = start_point.1 as isize + v.1;
            if x < 0 || y < 0 || y as usize >= mx.len() || x as usize >= mx[y as usize].len() {
                return None;
            }

            let tile = &mx[y as usize][x as usize];
            if move_from(x, y, tile)
                .iter()
                .any(|(nx, ny)| *nx as usize == start_point.0 && *ny as usize == start_point.1) {
                return Some((x, y, tile));
            }
            else {
                return None;
            }
        })
        .collect();

    let mut visited: HashSet<(isize, isize)> = HashSet::from([(start_point.0 as isize, start_point.1 as isize)]);

    let mut loop_path: Vec<(isize, isize, isize, isize)> = vec!();
    let mut winding = 0;

    while !stack.is_empty()
    {
        let curr = stack.pop().unwrap();
        visited.insert((curr.0, curr.1));

        for (x, y) in move_from(curr.0, curr.1, curr.2) {
            let pos = (x, y);
            if visited.contains(&pos) {
                continue;
            }

            let tile = &mx[y as usize][x as usize];

            match (curr.2, x - curr.0) {
                (Tile::NorthToEastBend, dx) if dx == 0 => winding += 1,
                (Tile::NorthToEastBend, _) => winding -= 1,
                (Tile::NorthToWestBend, dx) if dx < 0 => winding += 1,
                (Tile::NorthToWestBend, _) => winding -= 1,
                (Tile::SouthToEastBend, dx) if dx > 0 => winding += 1,
                (Tile::SouthToEastBend, _) => winding -= 1,
                (Tile::SouthToWestBend, dx) if dx == 0 => winding += 1,
                (Tile::SouthToWestBend, _) => winding -= 1,
                _ => {}
            }
            visited.insert(pos);
            stack.push((x, y, tile));
            loop_path.push((curr.0, curr.1, x, y));
        }
    }

    if loop_path.is_empty() {
        return 0;
    }
    let clockwise = winding > 0;

    let start = *loop_path.first().unwrap();
    let end = *loop_path.last().unwrap();
    loop_path.push((end.2, end.3, start_point.0 as isize, start_point.1 as isize));
    loop_path.push((start_point.0 as isize, start_point.1 as isize, start.0, start.1));

    let mut fill_stack: Vec<(isize, isize)> = vec!();
    let mut inside_count = 0;

    for (lcx, lcy, lnx, lny) in &loop_path {
        let ldx = lnx - lcx;
        let ldy = lny - lcy;
        let lctile = &mx[*lcy as usize][*lcx as usize];

        let offs = match (ldx, ldy, clockwise) {
            (1, 0, true) => match lctile {
                Tile::NorthToEastBend => vec!((-1, 0), (0, 1)),
                _ => vec!((0, 1))
            },
            (1, 0, false) => match lctile {
                Tile::SouthToEastBend => vec!((-1, 0), (0, -1)),
                _ => vec!((0, -1))
            },
            (0, 1, true) => match lctile {
                Tile::SouthToEastBend => vec!((-1, 0), (0, -1)),
                _ => vec!((-1, 0))
            },
            (0, 1, false) => match lctile {
                Tile::SouthToWestBend => vec!((1, 0), (0, -1)),
                _ => vec!((1, 0))
            },
            (-1, 0, true) => match lctile {
                Tile::SouthToWestBend => vec!((1, 0), (0, -1)),
                _ => vec!((0, -1))
            },
            (-1, 0, false) => match lctile {
                Tile::NorthToWestBend => vec!((1, 0), (0, 1)),
                _ => vec!((0, 1))
            },
            (0, -1, true) => match lctile {
                Tile::NorthToWestBend => vec!((1, 0), (0, 1)),
                _ => vec!((1, 0))
            },
            (0, -1, false) => match lctile {
                Tile::NorthToEastBend => vec!((-1, 0), (0, 1)),
                _ => vec!((-1, 0))
            },
            (a, b, c) => panic!("Unreachable state: ({}, {}), clockwise={}", a, b, c)
        };

        for (offx, offy) in offs {
            let sx = lcx + offx;
            let sy: isize = lcy + offy;
            if visited.contains(&(sx, sy)) {
                continue;
            }

            fill_stack.push((sx, sy));
            visited.insert((sx, sy));

            while !fill_stack.is_empty()
            {
                let curr = fill_stack.pop().unwrap();
                inside_count += 1;

                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x = curr.0 + dx;
                    let y = curr.1 + dy;
                    let pos = (x, y);
                    if visited.contains(&pos) {
                        continue;
                    }
                    visited.insert(pos);
                    fill_stack.push(pos);
                }
            }
        }
    }

    return inside_count;
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

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
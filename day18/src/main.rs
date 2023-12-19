use std::{fs, time::Instant, collections::{HashSet, HashMap}};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn solve_part_1(input: &String) -> usize {
    let start_pos = (0, 0);

    let path_points_vec: Vec<((i32, i32), Direction)> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            let dir = match it.next().unwrap()  {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                d => panic!("Invalid direction: {}", d)
            };
            let dist = it.next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            return (dir, dist);
        })
        .scan(start_pos.clone(), |pos, (dir, dist)| {
            let curr_pos = pos.clone();
            match dir {
                Direction::Up => pos.1 -= dist,
                Direction::Down => pos.1 += dist,
                Direction::Left => pos.0 -= dist,
                Direction::Right => pos.0 += dist,
            }
            return Some((curr_pos, dir, dist));
        })
        .flat_map(|(pos, dir, dist)| match dir {
            Direction::Up => (0..=dist).map(|y_off| ((pos.0, pos.1 - y_off), dir.clone())).collect::<Vec<_>>(),
            Direction::Down => (0..=dist).map(|y_off| ((pos.0, pos.1 + y_off), dir.clone())).collect(),
            Direction::Left => (0..=dist).map(|x_off| ((pos.0 - x_off, pos.1), dir.clone())).collect(),
            Direction::Right => (0..=dist).map(|x_off|((pos.0 + x_off, pos.1), dir.clone())).collect(),
        })
        .collect();

    let mut path_points: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();
    for ((x, y), dir) in path_points_vec {
        match path_points.get_mut(&(x, y)) {
            Some(dirs) => {
                dirs.push(dir);
            }
            None => {
                path_points.insert((x, y), vec!(dir));
            }
        }
    }

    let (min_x, min_y, max_x, max_y) = path_points
        .keys()
        .fold((start_pos.0, start_pos.1, start_pos.0, start_pos.1), 
            |(min_x, min_y, max_x, max_y), (x, y)| 
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y)));

    let empty_dir_vec = vec!();

    let points: HashSet<_> = (min_y..=max_y)
        .flat_map(|y| {
            let ppcap = &path_points;
            let edvcap = &empty_dir_vec;
            (min_x..=max_x)
                .scan(false, move |inside, x| {
                    let mut on_border = false;
                    for dir in ppcap.get(&(x, y)).unwrap_or(&edvcap) {
                        match dir {
                            Direction::Up => {
                                on_border = true;
                                *inside = true;
                            },
                            Direction::Down => {
                                on_border = true;
                                *inside = false;
                            },
                            _ => on_border = true
                        }
                    }
                    return Some((*inside || on_border, x, y));
                })
                .filter_map(|(inside, x, y)| if inside {Some((x, y))} else {None})
        })
        .collect();

    return points.len();
}

fn solve_part_2(input: &String) -> usize {
    let start_pos = (0, 0);

    let path_points: Vec<((i64, i64), Direction, i64)> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            it.next();
            it.next();
            let str = it.next().unwrap();

            let dist = i64::from_str_radix(&str[2..str.len()-2], 16)
                .unwrap();
            let dir = match str.chars().nth(str.len()-2)  {
                Some('0') => Direction::Right,
                Some('1') => Direction::Down,
                Some('2') => Direction::Left,
                Some('3') => Direction::Up,
                Some(d) => panic!("Invalid direction: {}", d),
                None => panic!("No direction provided")
            };
            return (dir, dist);
        })
        .scan(start_pos.clone(), |pos, (dir, dist)| {
            let curr_pos = pos.clone();
            match dir {
                Direction::Up => pos.1 -= dist,
                Direction::Down => pos.1 += dist,
                Direction::Left => pos.0 -= dist,
                Direction::Right => pos.0 += dist,
            }
            return Some((curr_pos, dir, dist));
        })
        .collect();

    let (_min_x, min_y, _max_x, max_y) = path_points
        .iter()
        .fold((start_pos.0 as i64, start_pos.1 as i64, start_pos.0 as i64, start_pos.1 as i64), 
            |(min_x, min_y, max_x, max_y), ((x, y), _, _)| 
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y)));

    return (min_y..=max_y)
        .map(|y| {
            let mut lines: Vec<_> = path_points
                .iter()
                .filter(|((_, ly), dir, dist)| match dir {
                    Direction::Up => (y <= *ly) && (y >= (ly - dist)),
                    Direction::Down => (y >= *ly) && (y <= (ly + dist)),
                    _ => y == *ly
                })
                .collect();
            lines.sort_by(|((x1, _), dir1, dist1), ((x2, _), dir2, dist2)| { 
                let lx1 = if *dir1 == Direction::Left {x1 - dist1} else {*x1};
                let lx2 = if *dir2 == Direction::Left {x2 - dist2} else {*x2};
                lx1.cmp(&lx2).then_with(|| match (dir1, dir2) {
                    (Direction::Up, Direction::Left)
                    | (Direction::Up, Direction::Right)
                    | (Direction::Down, Direction::Left)
                    | (Direction::Down, Direction::Right) => std::cmp::Ordering::Less,
                    (Direction::Left, Direction::Up)
                    | (Direction::Left, Direction::Down)
                    | (Direction::Right, Direction::Up)
                    | (Direction::Right, Direction::Down) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal
                })
            });

            let mut count = 0;
            let mut last_up_x = None;
            let mut last_down_x = None;
            for ((x, _), dir, _) in &lines {
                match (dir, last_up_x, last_down_x) {
                    (Direction::Up, None, None) => last_up_x = Some(x),
                    (Direction::Up, Some(lux), Some(ldx)) => {
                        count += ldx - lux + 1;
                        last_up_x = Some(x);
                        last_down_x = None;
                    },
                    (Direction::Down, Some(_), _) => {
                        last_down_x = Some(x);
                    }
                    (Direction::Left | Direction::Right, _, Some(_)) => last_down_x = None,
                    _ => {}
                }
            }

            match (last_up_x, last_down_x) {
                (Some(lux), Some(ldx)) => count += ldx - lux + 1,
                _ => {}
            }

            return count as usize;
        })
        .sum();
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
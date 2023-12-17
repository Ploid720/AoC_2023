use std::{fs, time::Instant, collections::HashSet};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Beam {
    x: usize,
    y: usize,
    dir: Direction
}

#[derive(Debug, Clone)]
enum Command {
    HeadUp,
    HeadDown,
    HeadLeft,
    HeadRight,
    SplitUpDown,
    SplitLeftRight,
    Exit
}

#[derive(Debug)]
struct Visited {
    going_up: bool,
    going_down: bool,
    going_left: bool,
    going_right: bool,
}

fn solve_part_1(input: &String) -> usize {
    let w = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.len())
        .min()
        .unwrap_or(0);
    let h = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let left_mx: Vec<Vec<_>> = input.lines()
        .map(|line| line
            .chars()
            .enumerate()
            .fold(vec!((Command::Exit, 0)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadDown, i)),
                    '\\' => acc.push((Command::HeadUp, i)),
                    '|' => acc.push((Command::SplitUpDown, i)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            }))
        .collect();

    let right_mx: Vec<Vec<_>> = input.lines()
        .map(|line| line
            .chars()
            .rev()
            .enumerate()
            .fold(vec!((Command::Exit, w - 1)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadUp, (w as isize - i as isize - 1).max(0) as usize)),
                    '\\' => acc.push((Command::HeadDown, (w as isize - i as isize - 1).max(0) as usize)),
                    '|' => acc.push((Command::SplitUpDown, (w as isize - i as isize - 1).max(0) as usize)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            })
            .iter()
            .map(|x| x.clone())
            .rev()
            .skip(1)
            .chain(std::iter::once((Command::Exit, w - 1)))
            .collect())
        .collect();

    let up_mx: Vec<Vec<_>> = transpose_lines(input)
        .map(|it| it
            .enumerate()
            .fold(vec!((Command::Exit, 0)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadRight, i)),
                    '\\' => acc.push((Command::HeadLeft, i)),
                    '-' => acc.push((Command::SplitLeftRight, i)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            }))
        .collect();

    let down_mx: Vec<Vec<_>> = transpose_lines(input)
        .map(|it| it
            .rev()
            .enumerate()
            .fold(vec!((Command::Exit, h - 1)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadLeft, (h as isize - i as isize - 1).max(0) as usize)),
                    '\\' => acc.push((Command::HeadRight, (h as isize - i as isize - 1).max(0) as usize)),
                    '-' => acc.push((Command::SplitLeftRight, (h as isize - i as isize - 1).max(0) as usize)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            })
            .iter()
            .map(|x| x.clone())
            .rev()
            .skip(1)
            .chain(std::iter::once((Command::Exit, h - 1)))
            .collect())
        .collect();

    let mut beam_stack = match input.chars().next().unwrap_or('.') {
        '/' => vec!(Beam{x: 0, y: 0, dir: Direction::Up}),
        '\\' => vec!(Beam{x: 0, y: 0, dir: Direction::Down}),
        '|' => vec!(
            Beam{x: 0, y: 0, dir: Direction::Up}, 
            Beam{x: 0, y: 0, dir: Direction::Down}),
        _ => vec!(Beam{x: 0, y: 0, dir: Direction::Right})
    };

    let mut beam_paths = vec!();

    let mut visited: Vec<Vec<_>> = (0..h)
        .map(|_y| (0..w)
            .map(|_x| Visited{
                going_up: false, 
                going_down: false, 
                going_left: false,
                going_right: false
            })
            .collect())
        .collect();

    while !beam_stack.is_empty() {
        let mut beam = beam_stack.pop().unwrap();
        let mut beam_path = vec!();

        loop {
            beam_path.push((beam.x, beam.y));

            let (i, j) = match beam.dir {
                Direction::Up => {
                    (beam.x, beam.y)
                },
                Direction::Down => {

                    (beam.x, beam.y)
                },
                Direction::Left => {

                    (beam.y, beam.x)
                },
                Direction::Right => {

                    (beam.y, beam.x)
                }
            };

            let (cmd, v) = match beam.dir {
                Direction::Up => &up_mx[i][j],
                Direction::Down => &down_mx[i][j],
                Direction::Left => &left_mx[i][j],
                Direction::Right => &right_mx[i][j]
            };

            match beam.dir {
                Direction::Up => beam.y = *v,
                Direction::Down => beam.y = *v,
                Direction::Left => beam.x = *v,
                Direction::Right => beam.x = *v
            }

            let vis = &mut visited[beam.y][beam.x];
            let vis_var = match beam.dir {
                Direction::Up =>  &mut vis.going_up,
                Direction::Down => &mut vis.going_down,
                Direction::Left => &mut vis.going_left,
                Direction::Right => &mut vis.going_right
            };

            if *vis_var && 
                ((mx[beam.y][beam.x] == '-') || (mx[beam.y][beam.x] == '|')) {
                break;
            }
            else {
                *vis_var = true;
            }

            match cmd {
                Command::HeadUp => beam.dir = Direction::Up,
                Command::HeadDown => beam.dir = Direction::Down,
                Command::HeadLeft => beam.dir = Direction::Left,
                Command::HeadRight => beam.dir = Direction::Right,
                Command::SplitUpDown => {
                    beam.dir = Direction::Up;
                    beam_stack.push(Beam{x: beam.x, y: beam.y, dir: Direction::Down});
                }
                Command::SplitLeftRight => {
                    beam.dir = Direction::Left;
                    beam_stack.push(Beam{x: beam.x, y: beam.y, dir: Direction::Right});
                }
                Command::Exit => {
                    assert!(match beam.dir {
                        Direction::Up => beam.y == 0,
                        Direction::Down => beam.y == h - 1,
                        Direction::Left => beam.x == 0,
                        Direction::Right => beam.x == w - 1
                    });
                    break;
                }
            }
        }

        beam_path.push((beam.x, beam.y));
        beam_paths.push(beam_path);
    }

    return beam_paths
        .iter()
        .map(|v| v.iter().zip(v.iter().skip(1)))
        .flat_map(|it| it
            .flat_map(|(a, b)| if a.0 != b.0 {
                    (a.0.min(b.0)..=a.0.max(b.0)).map(|x| (x, a.1)).collect::<Vec<_>>().into_iter()
                } 
                else {
                    (a.1.min(b.1)..=a.1.max(b.1)).map(|y| (a.0, y)).collect::<Vec<_>>().into_iter()
                }))
        .collect::<HashSet<(usize, usize)>>()
        .len();
}

fn solve_part_2(input: &String) -> usize {
    let w = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.len())
        .min()
        .unwrap_or(0);
    let h = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let left_mx: Vec<Vec<_>> = input.lines()
        .map(|line| line
            .chars()
            .enumerate()
            .fold(vec!((Command::Exit, 0)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadDown, i)),
                    '\\' => acc.push((Command::HeadUp, i)),
                    '|' => acc.push((Command::SplitUpDown, i)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            }))
        .collect();

    let right_mx: Vec<Vec<_>> = input.lines()
        .map(|line| line
            .chars()
            .rev()
            .enumerate()
            .fold(vec!((Command::Exit, w - 1)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadUp, (w as isize - i as isize - 1).max(0) as usize)),
                    '\\' => acc.push((Command::HeadDown, (w as isize - i as isize - 1).max(0) as usize)),
                    '|' => acc.push((Command::SplitUpDown, (w as isize - i as isize - 1).max(0) as usize)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            })
            .iter()
            .map(|x| x.clone())
            .rev()
            .skip(1)
            .chain(std::iter::once((Command::Exit, w - 1)))
            .collect())
        .collect();

    let up_mx: Vec<Vec<_>> = transpose_lines(input)
        .map(|it| it
            .enumerate()
            .fold(vec!((Command::Exit, 0)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadRight, i)),
                    '\\' => acc.push((Command::HeadLeft, i)),
                    '-' => acc.push((Command::SplitLeftRight, i)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            }))
        .collect();

    let down_mx: Vec<Vec<_>> = transpose_lines(input)
        .map(|it| it
            .rev()
            .enumerate()
            .fold(vec!((Command::Exit, h - 1)), |mut acc, (i, c)| {
                match c {
                    '/' => acc.push((Command::HeadLeft, (h as isize - i as isize - 1).max(0) as usize)),
                    '\\' => acc.push((Command::HeadRight, (h as isize - i as isize - 1).max(0) as usize)),
                    '-' => acc.push((Command::SplitLeftRight, (h as isize - i as isize - 1).max(0) as usize)),
                    _ => acc.push(acc.last().unwrap().clone())
                }
                return acc;
            })
            .iter()
            .map(|x| x.clone())
            .rev()
            .skip(1)
            .chain(std::iter::once((Command::Exit, h - 1)))
            .collect())
        .collect();

    let mut res = 0;

    for (dir, v) in vec!(Direction::Up, Direction::Down, Direction::Left, Direction::Right)
        .iter()
        .flat_map(|d| match d {
            Direction::Up => 0..w,
            Direction::Down => 0..w,
            Direction::Left => 0..h,
            Direction::Right => 0..h
        }.map(move |v| (d, v))) {

        let (start_x, start_y) = match dir {
            Direction::Up => (v, h - 1),
            Direction::Down => (v, 0),
            Direction::Left => (w - 1, v),
            Direction::Right => (0, v)
        };

        let mut beam_stack = match mx[start_y][start_x] {
            '/' => vec!(Beam{x: start_x, y: start_y, dir: match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up
            }}),
            '\\' => vec!(Beam{x: start_x, y: start_y, dir: match dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down
            }}),
            '|' => match dir {
                Direction::Left | Direction::Right => vec!(
                    Beam{x: start_x, y: start_y, dir: Direction::Up}, 
                    Beam{x: start_x, y: start_y, dir: Direction::Down}),
                _ => vec!(Beam{x: start_x, y: start_y, dir: dir.clone()})
            }
            '-' => match dir {
                Direction::Up | Direction::Down => vec!(
                    Beam{x: start_x, y: start_y, dir: Direction::Left}, 
                    Beam{x: start_x, y: start_y, dir: Direction::Right}),
                _ => vec!(Beam{x: start_x, y: start_y, dir: dir.clone()})
            }
            _ => vec!(Beam{x: start_x, y: start_y, dir: dir.clone()})
        };

        let mut beam_paths = vec!();

        let mut visited: Vec<Vec<_>> = (0..h)
            .map(|_y| (0..w)
                .map(|_x| Visited{
                    going_up: false, 
                    going_down: false, 
                    going_left: false,
                    going_right: false
                })
                .collect())
            .collect();

        while !beam_stack.is_empty() {
            let mut beam = beam_stack.pop().unwrap();
            let mut beam_path = vec!();

            loop {
                beam_path.push((beam.x, beam.y));

                let (i, j) = match beam.dir {
                    Direction::Up => {
                        (beam.x, beam.y)
                    },
                    Direction::Down => {

                        (beam.x, beam.y)
                    },
                    Direction::Left => {

                        (beam.y, beam.x)
                    },
                    Direction::Right => {

                        (beam.y, beam.x)
                    }
                };

                let (cmd, v) = match beam.dir {
                    Direction::Up => &up_mx[i][j],
                    Direction::Down => &down_mx[i][j],
                    Direction::Left => &left_mx[i][j],
                    Direction::Right => &right_mx[i][j]
                };

                match beam.dir {
                    Direction::Up => beam.y = *v,
                    Direction::Down => beam.y = *v,
                    Direction::Left => beam.x = *v,
                    Direction::Right => beam.x = *v
                }

                let vis = &mut visited[beam.y][beam.x];
                let vis_var = match beam.dir {
                    Direction::Up =>  &mut vis.going_up,
                    Direction::Down => &mut vis.going_down,
                    Direction::Left => &mut vis.going_left,
                    Direction::Right => &mut vis.going_right
                };

                if *vis_var && 
                    ((mx[beam.y][beam.x] == '-') || (mx[beam.y][beam.x] == '|')) {
                    break;
                }
                else {
                    *vis_var = true;
                }

                match cmd {
                    Command::HeadUp => beam.dir = Direction::Up,
                    Command::HeadDown => beam.dir = Direction::Down,
                    Command::HeadLeft => beam.dir = Direction::Left,
                    Command::HeadRight => beam.dir = Direction::Right,
                    Command::SplitUpDown => {
                        beam.dir = Direction::Up;
                        beam_stack.push(Beam{x: beam.x, y: beam.y, dir: Direction::Down});
                    }
                    Command::SplitLeftRight => {
                        beam.dir = Direction::Left;
                        beam_stack.push(Beam{x: beam.x, y: beam.y, dir: Direction::Right});
                    }
                    Command::Exit => {
                        assert!(match beam.dir {
                            Direction::Up => beam.y == 0,
                            Direction::Down => beam.y == h - 1,
                            Direction::Left => beam.x == 0,
                            Direction::Right => beam.x == w - 1
                        });
                        break;
                    }
                }
            }

            beam_path.push((beam.x, beam.y));
            beam_paths.push(beam_path);
        }

        let curr_res = beam_paths
            .iter()
            .map(|v| v.iter().zip(v.iter().skip(1)))
            .flat_map(|it| it
                .flat_map(|(a, b)| if a.0 != b.0 {
                        (a.0.min(b.0)..=a.0.max(b.0)).map(|x| (x, a.1)).collect::<Vec<_>>().into_iter()
                    } 
                    else {
                        (a.1.min(b.1)..=a.1.max(b.1)).map(|y| (a.0, y)).collect::<Vec<_>>().into_iter()
                    }))
            .collect::<HashSet<(usize, usize)>>()
            .len();
        if curr_res > res {
            res = curr_res;
        }
    }

    return res;
}

fn transpose_lines(str: &str) 
    -> impl Iterator<Item = impl DoubleEndedIterator<Item = char> + '_>
{
    return (0 ..)
        .scan((), move |&mut (), row_idx| {
            let mut col_iter = str.lines();
            let first_col = col_iter.next()?;
            let first = first_col
                .chars()
                .nth(row_idx)?;
            return Some(Iterator::chain(
                std::iter::once(first),
                col_iter
                    .filter_map(move |column| {
                        column
                            .chars()
                            .nth(row_idx)
                    })
            ));
        })
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
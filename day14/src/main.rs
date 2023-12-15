use std::{fs, time::Instant, collections::{HashSet, HashMap}};

enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

fn solve_part_1(input: &String) -> usize {
    return transpose_lines(input)
        .map(|it| it
            .enumerate()
            .fold((vec!(), 0), |(mut acc, col_len), (i, c)| match c {
                'O' => {
                    let bucket = acc.pop().unwrap_or((0, 0));
                    acc.push((bucket.0, bucket.1 + 1));
                    return (acc, col_len + 1);
                },
                '#' => {
                    acc.push((i + 1, 0));
                    return (acc, col_len + 1);
                }
                _ => (acc, col_len + 1)
            }))
        .map(|(buckets, col_len)| buckets
            .iter()
            .map(|(start, count)| (count * (2 * (col_len - start) + 1 - count)) / 2)
            .sum::<usize>())
        .sum();
}

fn solve_part_2(input: &String) -> usize {
    let target_cycle_count = 1000000000;

    let w = input
        .lines()
        .map(|line| line.trim().len())
        .min()
        .unwrap_or(0);
    let h = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    let west_mx: Vec<Vec<_>> = input.lines()
        .map(|line| line
            .chars()
            .enumerate()
            .fold(vec!(0), |mut acc, (i, c)| match c {
                '#' => {
                    acc.push(i + 1);
                    return acc;
                }
                _ => {
                    let last = acc.last().unwrap_or(&0);
                    acc.push(*last);
                    return acc;
                }
            }))
        .collect();

    let east_mx: Vec<Vec<_>> = input.lines()
        .map(|line| line
            .chars()
            .rev()
            .enumerate()
            .fold(vec!(w - 1), |mut acc, (i, c)| match c {
                '#' => {
                    acc.push((w as isize - i as isize - 2) as usize);
                    return acc;
                }
                _ => {
                    let last = acc.last().unwrap();
                    acc.push(*last);
                    return acc;
                }
            })
            .iter()
            .map(|x| *x)
            .rev()
            .collect())
        .collect();

    let north_mx: Vec<Vec<_>> = transpose_lines(input)
        .map(|it| it
            .enumerate()
            .fold(vec!(0), |mut acc, (i, c)| match c {
                '#' => {
                    acc.push(i + 1);
                    return acc;
                }
                _ => {
                    let last = acc.last().unwrap_or(&0);
                    acc.push(*last);
                    return acc;
                }
            }))
        .collect();

    let south_mx: Vec<Vec<_>> = transpose_lines(input)
        .map(|it| it
            .rev()
            .enumerate()
            .fold(vec!(h - 1), |mut acc, (i, c)| match c {
                '#' => {
                    acc.push((h as isize - i as isize - 2) as usize);
                    return acc;
                }
                _ => {
                    let last = acc.last().unwrap();
                    acc.push(*last);
                    return acc;
                }
            })
            .iter()
            .map(|x| *x)
            .rev()
            .collect())
        .collect();

    let base_rocks: Vec<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                'O' => Some((x, y)),
                _ => None
            }))
        .collect();

    fn move_rocks(rocks: &mut HashSet<(usize, usize)>, mx: &Vec<Vec<usize>>, dir: Direction) {
        let x_axis = match dir {
            Direction::NORTH => false,
            Direction::WEST => true,
            Direction::SOUTH => false,
            Direction::EAST => true
        };
        let res = rocks.iter()
            .fold(HashMap::new(), |mut acc, rock| {
                let v = if x_axis {mx[rock.1][rock.0]} else {mx[rock.0][rock.1]};
                let bucket = if x_axis {(v, rock.1)} else {(rock.0, v)};
                let count = acc.get(&bucket).unwrap_or(&0);
                acc.insert(bucket, count + 1);
                return acc;
            });

        rocks.clear();
        res.iter()
            .for_each(|((x, y), v)| (0..*v)
                .for_each(|i| {rocks.insert(match dir {
                    Direction::NORTH => (*x, *y + i),
                    Direction::WEST => (*x + i, *y),
                    Direction::SOUTH => (*x, *y - i),
                    Direction::EAST => (*x - i, *y)
                });}));
    }

    let mut cycle = 0;
    let mut slow_rocks: HashSet<(usize, usize)> = base_rocks.iter().map(|x| *x).collect();
    let mut fast_rocks: HashSet<(usize, usize)> = base_rocks.iter().map(|x| *x).collect();

    loop {
        move_rocks(&mut slow_rocks, &north_mx, Direction::NORTH);
        move_rocks(&mut slow_rocks, &west_mx, Direction::WEST);
        move_rocks(&mut slow_rocks, &south_mx, Direction::SOUTH);
        move_rocks(&mut slow_rocks, &east_mx, Direction::EAST);

        for _ in 0..2 {
            move_rocks(&mut fast_rocks, &north_mx, Direction::NORTH);
            move_rocks(&mut fast_rocks, &west_mx, Direction::WEST);
            move_rocks(&mut fast_rocks, &south_mx, Direction::SOUTH);
            move_rocks(&mut fast_rocks, &east_mx, Direction::EAST);
        }

        cycle += 1;

        if slow_rocks == fast_rocks {
            break;
        }
    }

    for _ in 0..(target_cycle_count % cycle) {
        move_rocks(&mut fast_rocks, &north_mx, Direction::NORTH);
        move_rocks(&mut fast_rocks, &west_mx, Direction::WEST);
        move_rocks(&mut fast_rocks, &south_mx, Direction::SOUTH);
        move_rocks(&mut fast_rocks, &east_mx, Direction::EAST);
    }

    return fast_rocks.iter()
        .map(|v| h - v.1)
        .sum();
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
use std::{fs, time::Instant, collections::HashSet};

fn solve(input: &String, expand_factor: usize) -> usize {
    let mx: Vec<Vec<_>> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line
            .chars()
            .collect())
        .collect();

    let rows_to_expand: HashSet<usize> = mx
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|c| *c == '.') {
                return Some(i);
            }
            else {
                return None;
            }
        })
        .collect();
    let cols_to_expand: HashSet<usize> = (0..
        (mx.iter().map(|row| row.len()).min().unwrap_or(0)))
            .filter(|i| mx.iter().all(|row| row[*i] == '.'))
            .collect();

    let galaxies: Vec<_> = mx.iter()
        .enumerate()
        .map(|(y, row)| row
            .iter()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                _ => None
            }))
        .flatten()
        .collect();

    return galaxies.iter()
        .enumerate()
        .flat_map(|(i1, g1)| {
            galaxies[i1..]
                .iter()
                .map(|g2| {
                    let x_min = g1.0.min(g2.0);
                    let x_max = g1.0.max(g2.0);
                    let y_min = g1.1.min(g2.1);
                    let y_max = g1.1.max(g2.1);
                    return (x_max - x_min)
                        + (expand_factor * cols_to_expand
                            .iter()
                            .filter(|x| **x > x_min && **x < x_max)
                            .count())
                        + (y_max - y_min)
                        + (expand_factor * rows_to_expand
                            .iter()
                            .filter(|y| **y > y_min && **y < y_max)
                            .count());
                })
        })
        .sum();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve(&content, 1);
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve(&content, 999999);
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
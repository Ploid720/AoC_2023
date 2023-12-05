use std::{fs, cmp, time::Instant};

fn solve_part_1(input: &String) -> i64 {
    let mut lines = input.lines();

    let seeds: Vec<_> = lines.next()
        .unwrap_or("")
        .split(" ")
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect();

    let mut a = seeds.clone();
    let mut b = seeds.clone();

    let res = lines
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold((&mut a, &mut b), |(original, parsed), line| {
            let vals: Vec<_> = line.split(" ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>())
                .collect();

            if vals.iter().all(|x| x.is_ok()) {
                let dest_start = vals[0].as_ref().unwrap();
                let src_start = vals[1].as_ref().unwrap();
                let len = vals[2].as_ref().unwrap();
                let src_end = src_start + len;

                for e in original.iter().enumerate() {
                    if e.1 >= src_start && e.1 < &src_end {
                        parsed[e.0] = e.1 - src_start + dest_start;
                    }
                }

                return (original, parsed);
            }

            for e in parsed.iter().enumerate() {
                original[e.0] = *e.1;
            }
            return (original, parsed);
        })
        .1
        .iter()
        .min()
        .unwrap_or(&0);

    return *res;
}

fn solve_part_2(input: &String) -> i64 {
    let mut lines = input.lines();

    let seeds_raw: Vec<_> = lines.next()
        .unwrap_or("")
        .split(" ")
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect();

    let mut seeds: Vec<(i64, i64)> = vec!();
    for i in (0..seeds_raw.len()).step_by(2) {
        seeds.push((seeds_raw[i], seeds_raw[i] + seeds_raw[i + 1]));
    }

    let res = lines
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .chain(["done"].into_iter())
        .fold((&mut seeds, &mut vec!(), &mut vec!()), |(original, parsed, checked_ranges), line| {
            let vals: Vec<_> = line.split(" ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>())
                .collect();

            if vals.iter().all(|x| x.is_ok()) {
                let dest_start = *vals[0].as_ref().unwrap();
                let src_start = *vals[1].as_ref().unwrap();
                let len = vals[2].as_ref().unwrap();
                let src_end = src_start + len;

                for e in original.iter() {
                    let start = cmp::max(e.0, src_start);
                    let end = cmp::min(e.1, src_end);

                    if start <= end {
                        let off = src_start - dest_start;
                        parsed.push((start - off, end - off));
                    }
                }

                checked_ranges.push((src_start, src_end));
                return (original, parsed, checked_ranges);
            }

            for e in original.iter() {
                let mut start = e.0;
                let max_end_incl = e.1;
                let mut curr_end = max_end_incl;
                loop {
                    for checked_range in checked_ranges.iter() {
                        if checked_range.0 <= start {
                            start = cmp::max(start, checked_range.1);
                        }
                        else {
                            curr_end = cmp::min(curr_end, checked_range.0);
                        }
                    }
                    if start >= max_end_incl {
                        break;
                    }
                    if start >= curr_end {
                        break;
                    }
                    parsed.push((start, curr_end));
                    start = curr_end;
                    curr_end = max_end_incl;
                }
            }
            original.clear();
            for e in parsed.iter() {
                original.push(*e);
            }
            parsed.clear();
            checked_ranges.clear();

            return (original, parsed, checked_ranges);
        })
        .0
        .iter()
        .map(|x| x.0)
        .min()
        .unwrap_or(0);

    return res;
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
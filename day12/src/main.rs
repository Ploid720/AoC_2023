use std::{fs, time::Instant, collections::HashMap};

fn solve_part_1(input: &String) -> usize {
    return input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut spl = line.split_ascii_whitespace();
            let runs_str = spl.next().unwrap_or("");
            let counts_str = spl.next().unwrap_or("");

            let c_vec: Vec<_> = runs_str
                .chars()
                .collect();

            let runs = runs_str
                .chars()
                .enumerate()
                .fold((vec!(), false), |(mut acc, in_run), (i, e)| {
                    if e == '.' {
                        return (acc, false);
                    }
                    else if in_run {
                        let run: (usize, usize) = acc.pop().unwrap();
                        acc.push((run.0, run.1 + 1));
                        return (acc, true);
                    }
                    else {
                        acc.push((i, 1));
                        return (acc, true);
                    }
                })
                .0;

            let counts: Vec<_> = counts_str
                .split(",")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            return (c_vec, runs, counts);
        })
        
        .map(|(row, runs, counts)| {
            let mut res = 0;

            let mut stack: Vec<(_, _)> = vec!((runs, counts));

            while !stack.is_empty() {
                let (mut runs_left_mut, mut counts_left_mut) = stack.pop().unwrap();
                if runs_left_mut.is_empty() {
                    if counts_left_mut.is_empty() {
                        res += 1;
                    }
                    continue;
                }

                let run = runs_left_mut.pop().unwrap();
                let run_start = run.0;
                let run_len = run.1;
                let count = counts_left_mut.pop().unwrap_or(0);

                if count == 0 {
                    if !(run_start..(run_start + run_len)).all(|i| row[i] == '?') {
                        continue;
                    }
                    stack.push((runs_left_mut, counts_left_mut));
                    continue;
                }
                if run_len < count {
                    if (run_start..(run_start + run_len)).all(|i| row[i] == '?') {
                        counts_left_mut.push(count);
                        stack.push((runs_left_mut, counts_left_mut));
                        continue;
                    }
                    continue;
                }
                let margin = run_len - count;
                let valid_margins: Vec<_> = (0..=margin)
                    .filter(|run_offset| {
                        let mut valid = ((count + run_offset)..run_len)
                            .all(|i| row[run_start + i] == '?');
                        if *run_offset > 0 {
                            valid &= row[run_start + run_offset - 1] == '?';
                        }
                        return valid;
                    })
                    .collect();
                
                for margin in valid_margins {
                    let mut new_runs_left_mut = runs_left_mut.clone();
                    let new_counts_left_mut = counts_left_mut.clone();

                    if margin > 1 {
                        new_runs_left_mut.push((run_start, margin - 1));
                    }

                    stack.push((new_runs_left_mut, new_counts_left_mut));
                }

                if (run_start..(run_start + run_len)).all(|i| row[i] == '?') {
                    let new_runs_left_mut = runs_left_mut.clone();
                    let mut new_counts_left_mut = counts_left_mut.clone();
                    new_counts_left_mut.push(count);
                    stack.push((new_runs_left_mut, new_counts_left_mut));
                }
            }

            return res;
        })
        .sum();
}

fn solve_part_2(input: &String) -> usize {
    type Rows = Vec<(usize, usize)>;
    type Counts = Vec<usize>;
    type SolutionCache = HashMap<(Rows, Counts), usize>;

    return input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut spl = line.split_ascii_whitespace();
            let mut runs_str = spl.next().unwrap_or("").to_string();
            let mut counts_str = spl.next().unwrap_or("").to_string();

            let repeat_factor = 5;

            runs_str = std::iter::repeat(runs_str)
                .take(repeat_factor)
                .reduce(|acc, e| format!("{}?{}", acc, &e))
                .unwrap_or("".to_string());

            counts_str = std::iter::repeat(counts_str)
                .take(repeat_factor)
                .reduce(|acc, e| format!("{},{}", acc, &e))
                .unwrap_or("".to_string());

                let c_vec: Vec<_> = runs_str
                .chars()
                .collect();

            let runs = runs_str
                .chars()
                .enumerate()
                .fold((vec!(), false), |(mut acc, in_run), (i, e)| {
                    if e == '.' {
                        return (acc, false);
                    }
                    else if in_run {
                        let run: (usize, usize) = acc.pop().unwrap();
                        acc.push((run.0, run.1 + 1));
                        return (acc, true);
                    }
                    else {
                        acc.push((i, 1));
                        return (acc, true);
                    }
                })
                .0;

            let counts: Vec<_> = counts_str
                .split(",")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            return (c_vec, runs, counts);
        })   
        .map(|(row, runs, counts)| {
            fn solve(row: &Vec<char>, mut runs_left_mut: Rows, mut counts_left_mut: Counts, cache: &mut SolutionCache) -> usize
            {
                if runs_left_mut.is_empty() {
                    if counts_left_mut.is_empty() {
                        return 1;
                    }
                    return 0;
                }

                let run = runs_left_mut.pop().unwrap();
                let run_start = run.0;
                let run_len = run.1;
                let count = counts_left_mut.pop().unwrap_or(0);

                if count == 0 {
                    if !(run_start..(run_start + run_len)).all(|i| row[i] == '?') {
                        return 0;
                    }

                    let runs_counts = (runs_left_mut, counts_left_mut);
                    let cached = cache.get(&runs_counts);
                    if cached.is_some() {
                        return *cached.unwrap();
                    }
                    let rec_res = solve(row, runs_counts.0.clone(), runs_counts.1.clone(), cache);
                    cache.insert(runs_counts, rec_res);
                    return rec_res;
                }
                if run_len < count {
                    if (run_start..(run_start + run_len)).all(|i| row[i] == '?') {
                        counts_left_mut.push(count);

                        let runs_counts = (runs_left_mut, counts_left_mut);
                        let cached = cache.get(&runs_counts);
                        if cached.is_some() {
                            return *cached.unwrap();
                        }
                        let rec_res = solve(row, runs_counts.0.clone(), runs_counts.1.clone(), cache);
                        cache.insert(runs_counts, rec_res);
                        return rec_res;
                    }
                    return 0;
                }

                let margin = run_len - count;
                let valid_margins: Vec<_> = (0..=margin)
                    .filter(|run_offset| {
                        let mut valid = ((count + run_offset)..run_len)
                            .all(|i| row[run_start + i] == '?');
                        if *run_offset > 0 {
                            valid &= row[run_start + run_offset - 1] == '?';
                        }
                        return valid;
                    })
                    .collect();

                let mut res = 0;

                for margin in valid_margins {
                    let mut new_runs_left_mut = runs_left_mut.clone();
                    let new_counts_left_mut = counts_left_mut.clone();

                    if margin > 1 {
                        new_runs_left_mut.push((run_start, margin - 1));
                    }

                    let runs_counts = (new_runs_left_mut, new_counts_left_mut);
                    let cached = cache.get(&runs_counts);
                    if cached.is_some() {
                        res += *cached.unwrap();
                    }
                    else {
                        let rec_res = solve(row, runs_counts.0.clone(), runs_counts.1.clone(), cache);
                        cache.insert(runs_counts, rec_res);
                        res += rec_res;
                    }
                }

                if (run_start..(run_start + run_len)).all(|i| row[i] == '?') {
                    let new_runs_left_mut = runs_left_mut.clone();
                    let mut new_counts_left_mut = counts_left_mut.clone();
                    new_counts_left_mut.push(count);

                    let runs_counts = (new_runs_left_mut, new_counts_left_mut);
                    let cached = cache.get(&runs_counts);
                    if cached.is_some() {
                        res += *cached.unwrap();
                    }
                    else {
                        let rec_res = solve(row, runs_counts.0.clone(), runs_counts.1.clone(), cache);
                        cache.insert(runs_counts, rec_res);
                        res += rec_res;
                    }
                }

                return res;
            }

            return solve(&row, runs, counts, &mut HashMap::new());
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
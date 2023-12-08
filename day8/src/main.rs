use std::{fs, time::Instant, collections::HashMap};
use regex::Regex;

fn solve_part_1(input: &String) -> i32 {
    let path_reg = Regex::new(
        r"^(?P<in>\w+)\s*=\s*\(\s*(?P<outL>\w+)\s*,\s*(?P<outR>\w+)\s*\)[^\S\n\r]*$"
    ).unwrap();

    let mut lines_iter = input.lines();

    let order = lines_iter.next().unwrap_or("").trim();
    let mut order_gen = order.chars().cycle();

    let paths: HashMap<_, _> = lines_iter
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| path_reg.captures(line))
        .map(|m| (m["in"].to_string(), (m["outL"].to_string(), m["outR"].to_string())))
        .collect();

    let mut curr_node = &"AAA".to_string();
    let mut path_length = 0;
    while curr_node != "ZZZ" {
        let dir_opt = order_gen.next();
        let curr_paths_opt = paths.get(curr_node);
        if dir_opt.is_none() || curr_paths_opt.is_none() {
            return -1;
        }
        let dir = dir_opt.unwrap();
        let curr_paths = curr_paths_opt.unwrap();
        curr_node = match dir {
            'L' => &curr_paths.0,
            'R' => &curr_paths.1,
            _ => panic!("unknown direction")
        };
        path_length += 1;
    }

    return path_length;
}

fn solve_part_2(input: &String) -> i64 {
    let path_reg = Regex::new(
        r"^(?P<in>\w+)\s*=\s*\(\s*(?P<outL>\w+)\s*,\s*(?P<outR>\w+)\s*\)[^\S\n\r]*$"
    ).unwrap();

    fn lcm(a: i64, b: i64) -> i64 {
        return a * b / gcd(a, b);
    }
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b > 0 {
            let m = a % b;
            a = b;
            b = m;
        }
        return a;
    }

    let mut lines_iter = input.lines();

    let order = lines_iter.next().unwrap_or("").trim();

    let paths: HashMap<_, _> = lines_iter
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| path_reg.captures(line))
        .map(|m| (m["in"].to_string(), (m["outL"].to_string(), m["outR"].to_string())))
        .collect();

    let start_nodes: Vec<&String> = paths
        .keys()
        .filter(|path| path.ends_with('A'))
        .collect();
    
    return start_nodes.iter()
        .map(|start_node| {
            let mut order_gen = order.chars().cycle();
            let mut curr_node = *start_node;
            let mut path_length: i64 = 0;
            loop {
                let dir_opt = order_gen.next();
                let curr_paths_opt = paths.get(curr_node);
                if dir_opt.is_none() || curr_paths_opt.is_none() {
                    panic!("Invalid direction or no path found");
                }
                let dir = dir_opt.unwrap();
                let curr_paths = curr_paths_opt.unwrap();
                curr_node = match dir {
                    'L' => &curr_paths.0,
                    'R' => &curr_paths.1,
                    _ => panic!("unknown direction")
                };
                path_length += 1;

                if curr_node.ends_with('Z') {
                    return path_length; //This assumes path cycles have no phase offset
                }
            }
        })
        .reduce(|acc, e| lcm(acc, e))
        .unwrap_or(-1);
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
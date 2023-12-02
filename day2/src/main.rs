use std::fs;
use std::cmp;
use regex::Regex;
use std::collections::HashMap;

fn solve_part_1(input: &String) -> i32
{
    let game_reg = Regex::new(r"^Game\s*(?P<game_id>\d+):\s*(?P<rounds>.*)[^\S\n\r]*$").unwrap();
    let cube_reg = Regex::new(r"^(?P<cube_count>\d+)\s+(?P<cube_color>\w+)[^\S\n\r]*$").unwrap();

    let max_cube_counts: HashMap<&str, i32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ].iter().cloned().collect();

    return input.lines()
        .filter_map(|line| match line.trim() {
            "" => None,
            a => Some(a)
        })
        .filter_map(|line| {
            let game_match = game_reg.captures(line).unwrap();
            let game_id: &i32 = &game_match["game_id"].parse::<i32>().unwrap();
            for round in (&game_match["rounds"]).split(";") {
                for cube in round.trim().split(",") {
                    let cube_match = cube_reg.captures(cube.trim()).unwrap();
                    let cube_count = &cube_match["cube_count"].parse::<i32>().unwrap();
                    let cube_color = &cube_match["cube_color"];

                    let max_cube_count = max_cube_counts.get(cube_color).unwrap_or(&0);
                    if max_cube_count < cube_count {
                        return None;
                    }
                }
            }

            return Some(*game_id);
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
}

fn solve_part_2(input: &String) -> i32
{
    let game_reg = Regex::new(r"^Game\s*(?P<game_id>\d+):\s*(?P<rounds>.*)[^\S\n\r]*$").unwrap();
    let cube_reg = Regex::new(r"^(?P<cube_count>\d+)\s+(?P<cube_color>\w+)[^\S\n\r]*$").unwrap();

    return input.lines()
        .filter_map(|line| match line.trim() {
            "" => None,
            a => Some(a)
        })
        .map(|line| {
            let mut min_color_counts = HashMap::new();

            let game_match = game_reg.captures(line).unwrap();
            for round in (&game_match["rounds"]).split(";") {
                for cube in round.trim().split(",") {
                    let cube_match = cube_reg.captures(cube.trim()).unwrap();
                    let cube_count = &cube_match["cube_count"].parse::<i32>().unwrap();
                    let cube_color = &cube_match["cube_color"];

                    min_color_counts.insert(
                        cube_color.to_string(), 
                        *cmp::max(min_color_counts.get(cube_color).unwrap_or(cube_count), cube_count));
                }
            }

            let mut ret = 1;
            for min_count in min_color_counts {
                ret *= min_count.1;
            }
            return ret;
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let part_1_res = solve_part_1(&content);
    let part_2_res = solve_part_2(&content);

    println!("Part 1 result: {}", part_1_res);
    println!("Part 2 result: {}", part_2_res);
}
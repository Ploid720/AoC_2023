use std::{fs, collections::{HashSet, HashMap}};
use regex::Regex;

fn solve_part_1(input: &String) -> i32 {
    let card_reg = Regex::new(
        r"^Card\s*(?P<card_id>\d+):\s*(?P<winning>[^|]*)\s*\|\s*(?P<mine>.*)[^\S\n\r]*$"
    ).unwrap();
    
    return input.lines()
        .filter_map(|line| match line.trim() {
            "" => None,
            a => Some(a)
        })
        .map(|line| {
            let card_match = card_reg.captures(line).unwrap();
            // let card_id: &i32 = &card_match["card_id"].parse::<i32>().unwrap();
            let winning_nums: HashSet<i32> = (&card_match["winning"])
                .split(" ")
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            let win_count = (&card_match["mine"])
                .split(" ")
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .filter(|n| winning_nums.contains(n))
                .count();

            if win_count > 0 {
                return 1 << (win_count - 1);
            }
            return 0;
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
}
fn solve_part_2(input: &String) -> i32 {
    let card_reg = Regex::new(
        r"^Card\s*(?P<card_id>\d+):\s*(?P<winning>[^|]*)\s*\|\s*(?P<mine>.*)[^\S\n\r]*$"
    ).unwrap();
    
    let cards: HashMap<i32, i32> = input.lines()
        .filter_map(|line| match line.trim() {
            "" => None,
            a => Some(a)
        })
        .map(|line| {
            let card_match = card_reg.captures(line).unwrap();
            let card_id: &i32 = &card_match["card_id"].parse::<i32>().unwrap();
            let winning_nums: HashSet<i32> = (&card_match["winning"])
                .split(" ")
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            let win_count = (&card_match["mine"])
                .split(" ")
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .filter(|n| winning_nums.contains(n))
                .count();

            return (*card_id, win_count as i32);
        })
        .collect();

    let mut ret = 0;
    let mut stack: Vec<i32> = cards.iter()
        .map(|(id, _count)| *id)
        .collect();

    while !stack.is_empty()
    {
        let card_id = stack.pop().unwrap();
        let win_count = cards.get(&card_id).unwrap();
        for i in (card_id + 1)..=(card_id + win_count) {
            stack.push(i);
        }
        ret += 1;
    }

    return ret;
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
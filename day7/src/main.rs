use std::{fs, time::Instant, collections::HashMap, cmp::Ordering};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn solve_part_1(input: &String) -> i32 {
    let card_value_map: HashMap<char, isize> = "AKQJT98765432".chars()
        .enumerate()
        .map(|(i, c)| (c, -(i as isize)))
        .collect();

    fn determine_hand_type(hand: &str) -> HandType {
        let card_freq = hand
            .chars()
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                return map;
            });
        let card_freq_freq = card_freq
            .values()
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                return map;
            });

        if *card_freq_freq.get(&5).unwrap_or(&0) == 1 {
            return HandType::FiveOfAKind;
        }
        else if *card_freq_freq.get(&4).unwrap_or(&0) == 1 {
            return HandType::FourOfAKind;
        }
        else if *card_freq_freq.get(&3).unwrap_or(&0) == 1 {
            if *card_freq_freq.get(&2).unwrap_or(&0) == 1 {
                return HandType::FullHouse;
            }
            else {
                return HandType::ThreeOfAKind;
            }
        }
        let pair_count = *card_freq_freq.get(&2).unwrap_or(&0);
        if pair_count == 2 {
            return HandType::TwoPair;
        }
        else if pair_count == 1 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }

    let mut hands: Vec<_> = input.lines()
        .map(|line| line.split_ascii_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>())
        .map(|x| (x[0], x[1].parse::<i32>().unwrap_or(0)))
        .map(|(hand, bid)| (hand, determine_hand_type(hand), bid))
        .collect();

    hands.sort_unstable_by(|
        (hand_1, hand_type_1, _bid_1), 
        (hand_2, hand_type_2, _bid_2)| {
        let type_cmp = hand_type_2.cmp(hand_type_1);
        if type_cmp.is_ne() {
            return type_cmp;
        }

        for (c1, c2) in hand_1.chars().zip(hand_2.chars()) {
            let v1 = card_value_map.get(&c1).unwrap_or(&0);
            let v2 = card_value_map.get(&c2).unwrap_or(&0);
            if v1 != v2 {
                return v1.cmp(v2);
            }
        }
        return Ordering::Equal;
    });

    return hands.iter()
        .enumerate()
        .map(|(i, (_hand, _hand_type, bid))| bid * (i + 1) as i32)
        .sum();
}

fn solve_part_2(input: &String) -> i32 {
    let card_value_map: HashMap<char, isize> = "AKQT98765432J".chars()
        .enumerate()
        .map(|(i, c)| (c, -(i as isize)))
        .collect();

    fn determine_hand_type(hand: &str) -> HandType {
        let j_count = hand
            .chars()
            .filter(|c| *c == 'J')
            .count();
        let card_freq = hand
            .chars()
            .filter(|c| *c != 'J')
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                return map;
            });
        let card_freq_freq = card_freq
            .values()
            .fold(HashMap::new(), |mut map, val|{
                map.entry(val)
                    .and_modify(|frq|*frq+=1)
                    .or_insert(1);
                return map;
            });

        if *card_freq_freq.get(&5).unwrap_or(&0) == 1 {
            return HandType::FiveOfAKind;
        }
        else if *card_freq_freq.get(&4).unwrap_or(&0) == 1 {
            if j_count > 0 {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind;
        }
        else if *card_freq_freq.get(&3).unwrap_or(&0) == 1 {
            if j_count > 1 {
                return HandType::FiveOfAKind;
            }
            else if j_count > 0 {
                return HandType::FourOfAKind;
            }
            if *card_freq_freq.get(&2).unwrap_or(&0) == 1 {
                return HandType::FullHouse;
            }
            else {
                return HandType::ThreeOfAKind;
            }
        }
        let pair_count = *card_freq_freq.get(&2).unwrap_or(&0);
        if pair_count == 2 {
            return match j_count {
                n if n >= 3 => HandType::FiveOfAKind,
                2 => HandType::FourOfAKind,
                1 => HandType::FullHouse,
                _ => HandType::TwoPair
            }
        }
        else if pair_count == 1 {
            return match j_count {
                n if n >= 3 => HandType::FiveOfAKind,
                2 => HandType::FourOfAKind,
                1 => HandType::ThreeOfAKind,
                _ => HandType::OnePair
            }
        }

        return match j_count {
            n if n >= 4 => HandType::FiveOfAKind,
            3 => HandType::FourOfAKind,
            2 => HandType::ThreeOfAKind,
            1 => HandType::OnePair,
            _ => HandType::HighCard
        }
    }

    let mut hands: Vec<_> = input.lines()
        .map(|line| line.split_ascii_whitespace()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>())
        .map(|x| (x[0], x[1].parse::<i32>().unwrap_or(0)))
        .map(|(hand, bid)| (hand, determine_hand_type(hand), bid))
        .collect();

    hands.sort_unstable_by(|
        (hand_1, hand_type_1, _bid_1), 
        (hand_2, hand_type_2, _bid_2)| {
        let type_cmp = hand_type_2.cmp(hand_type_1);
        if type_cmp.is_ne() {
            return type_cmp;
        }

        for (c1, c2) in hand_1.chars().zip(hand_2.chars()) {
            let v1 = card_value_map.get(&c1).unwrap_or(&0);
            let v2 = card_value_map.get(&c2).unwrap_or(&0);
            if v1 != v2 {
                return v1.cmp(v2);
            }
        }
        return Ordering::Equal;
    });

    return hands.iter()
        .enumerate()
        .map(|(i, (_hand, _hand_type, bid))| bid * (i + 1) as i32)
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
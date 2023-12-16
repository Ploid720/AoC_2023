use std::{fs, time::Instant};

#[derive(Debug)]
enum Operation {
    Remove,
    Set
}

fn solve_part_1(input: &String) -> usize {
    return input
        .split(",")
        .map(|v| v
            .as_bytes()
            .iter()
            .filter_map(|c| match c {
                b'\n' => None,
                b'\r' => None,
                _ => Some(c)
            })
            .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256))
        .sum();
}

fn solve_part_2(input: &String) -> usize {
    return input
        .split(",")
        .map(|v| v
            .as_bytes()
            .iter()
            .filter_map(|c| match c {
                b'\n' => None,
                b'\r' => None,
                _ => Some(c)
            })
            .fold((String::new(), None, None, 0), |(mut label, op, _arg, hash), c| {
                return match c {
                    b'-' => (label, Some(Operation::Remove), None, hash),
                    b'=' => (label, Some(Operation::Set), None, hash),
                    _ => match op {
                        Some(Operation::Remove) => panic!("Operation does not accept arguments"),
                        Some(Operation::Set) => (label, op, Some(c - b'0'), hash),
                        None => {
                            label.push(*c as char);
                            (label, op, None, ((hash + *c as usize) * 17) % 256)
                        }
                    }
                }
            }))
        .map(|(label, op, arg, hash)| (label, op.unwrap(), arg.unwrap_or(0) as usize, hash as usize))
        .fold(vec!(vec!(); 256), |mut acc, (label, op, arg, hash)| {
            // acc
            //     .iter()
            //     .enumerate()
            //     .filter(|v| !v.1.is_empty())
            //     .for_each(|(i, v)| println!("\tBox {i}: {v:?}"));
            // println!("Op: {:?}, Label: {}", op, label);
            
            let lens_box: &mut Vec<(String, usize)> = &mut acc[hash];
            let pos = lens_box.iter().position(|(lens, _)| **lens == label);
            match op {
                Operation::Remove => {
                    if pos.is_some() {
                        lens_box.remove(pos.unwrap());
                    }
                    return acc;
                }
                Operation::Set => {
                    if pos.is_some() {
                        let ind = pos.unwrap();
                        let val = std::mem::take(&mut lens_box[ind]);
                        lens_box[ind] = (val.0, arg);
                    }
                    else {
                        lens_box.push((label, arg));
                    }
                    return acc;
                }
            }
        })
        .iter()
        .enumerate()
        .flat_map(|(box_ind, v)| v
            .iter()
            .enumerate()
            .map(move |(slot_ind, (_, focus))| (box_ind + 1) * (slot_ind + 1) * focus))
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
use std::{fs, time::Instant, collections::HashMap};

#[derive(Debug)]
enum Operation {
    Greater,
    Less
}

#[derive(Debug)]
struct Rule<'a> {
    argument: char,
    operation: Operation,
    value: i32,
    destination: &'a str
}

#[derive(Debug)]
struct PartRange<'a> {
    workflow: &'a str,
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32)   
}

fn solve_part_1(input: &String) -> i32 {
    let input_processed = input
        .replace("\r", "\n")
        .replace("\n\n", "\n");
    let mut data = input_processed
        .split("\n\n");

    let workflows: HashMap<_, _> = data.next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let name_end = line.find("{").unwrap();
            let name = &line[0..name_end];
            let mut rules_raw: Vec<_> = line[(name_end + 1)..(line.len() - 1)]
                .split(",")
                .collect();
            let default_dest = rules_raw.pop().unwrap();

            let rules: Vec<_> = rules_raw.iter()
                .map(|rule| {
                    let arg = rule.chars()
                        .nth(0)
                        .unwrap();
                    let op = match rule.chars()
                        .nth(1)
                        .unwrap() {
                        
                        '>' => Operation::Greater,
                        '<' => Operation::Less,
                        op => panic!("Invalid operation: {op}")
                    };
                    let value_end = rule.find(":").unwrap();
                    let value = rule[2..value_end].parse::<i32>().unwrap();
                    let destination = &rule[(value_end + 1)..];
                    return Rule{argument: arg, operation: op, value, destination};
                })
                .collect();

            return (name, (rules, default_dest));
        })
        .collect();

    let parts: Vec<HashMap<_, _>> = data.next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| &line[1..(line.len() - 1)])
        .map(|part| part.split(",")
            .map(|value| (value.chars().nth(0).unwrap(), value[2..].parse::<i32>().unwrap()))
            .collect())
        .collect();

    let mut res = 0;

    for part in parts {
        let mut curr_workflow_name = "in";
        while curr_workflow_name != "R" {
            let (rules, def) =  &workflows[&curr_workflow_name];
            let rule_match = rules.iter()
                .find(|rule| match rule.operation {
                    Operation::Greater => part.get(&rule.argument).unwrap() > &rule.value,
                    Operation::Less => part.get(&rule.argument).unwrap() < &rule.value
                });

            match rule_match {
                Some(rule) => curr_workflow_name = rule.destination,
                None => curr_workflow_name = def
            }

            if curr_workflow_name == "A" {
                res += part
                    .values()
                    .sum::<i32>();
                break;
            }
        }
    }

    return res;
}

fn solve_part_2(input: &String) -> usize {
    let input_processed = input
        .replace("\r", "\n")
        .replace("\n\n", "\n");
    let mut data = input_processed
        .split("\n\n");

    let workflows: HashMap<_, _> = data.next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let name_end = line.find("{").unwrap();
            let name = &line[0..name_end];
            let mut rules_raw: Vec<_> = line[(name_end + 1)..(line.len() - 1)]
                .split(",")
                .collect();
            let default_dest = rules_raw.pop().unwrap();

            let rules: Vec<_> = rules_raw.iter()
                .map(|rule| {
                    let arg = rule.chars()
                        .nth(0)
                        .unwrap();
                    let op = match rule.chars()
                        .nth(1)
                        .unwrap() {
                        
                        '>' => Operation::Greater,
                        '<' => Operation::Less,
                        op => panic!("Invalid operation: {op}")
                    };
                    let value_end = rule.find(":").unwrap();
                    let value = rule[2..value_end].parse::<i32>().unwrap();
                    let destination = &rule[(value_end + 1)..];
                    return Rule{argument: arg, operation: op, value, destination};
                })
                .collect();

            return (name, (rules, default_dest));
        })
        .collect();

    let mut res: usize = 0;

    let mut stack = vec!(PartRange{
        workflow: "in",
        x: (1, 4000), 
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000)});

    while let Some(range) = stack.pop() {
        if range.workflow == "R" {
            continue;
        }
        if range.workflow == "A" {
            res += (range.x.1 - range.x.0 + 1) as usize
                * (range.m.1 - range.m.0 + 1) as usize
                * (range.a.1 - range.a.0 + 1) as usize
                * (range.s.1 - range.s.0 + 1) as usize;
            continue;
        }

        let mut curr_min_x = range.x.0;
        let mut curr_max_x = range.x.1;
        let mut curr_min_m = range.m.0;
        let mut curr_max_m = range.m.1;
        let mut curr_min_a = range.a.0;
        let mut curr_max_a = range.a.1;
        let mut curr_min_s = range.s.0;
        let mut curr_max_s = range.s.1;

        let (rules, def) =  &workflows[&range.workflow];
        let mut apply_default = true;
        for rule in rules {
            match (rule.argument, &rule.operation) {
                ('x', Operation::Greater) => if curr_max_x > rule.value {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: ((rule.value + 1).max(curr_min_x), curr_max_x),
                        m: (curr_min_m, curr_max_m),
                        a: (curr_min_a, curr_max_a),
                        s: (curr_min_s, curr_max_s)});
                    curr_max_x = rule.value;
                    if curr_max_x < curr_min_x {
                        apply_default = false;
                        break;
                    }
                },
                ('x', Operation::Less) if curr_min_x < rule.value => {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, (rule.value - 1).min(curr_max_x)),
                        m: (curr_min_m, curr_max_m),
                        a: (curr_min_a, curr_max_a),
                        s: (curr_min_s, curr_max_s)});
                    curr_min_x = rule.value;
                    if curr_max_x < curr_min_x {
                        apply_default = false;
                        break;
                    }
                },
                ('m', Operation::Greater) => if curr_max_m > rule.value {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, curr_max_x),
                        m: ((rule.value + 1).max(curr_min_m), curr_max_m),
                        a: (curr_min_a, curr_max_a),
                        s: (curr_min_s, curr_max_s)});
                    curr_max_m = rule.value;
                    if curr_max_m < curr_min_m {
                        apply_default = false;
                        break;
                    }
                },
                ('m', Operation::Less) if curr_min_m < rule.value => {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, curr_max_x),
                        m: (curr_min_m, (rule.value - 1).min(curr_max_m)),
                        a: (curr_min_a, curr_max_a),
                        s: (curr_min_s, curr_max_s)});
                    curr_min_m = rule.value;
                    if curr_max_m < curr_min_m {
                        apply_default = false;
                        break;
                    }
                },
                ('a', Operation::Greater) => if curr_max_a > rule.value {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, curr_max_x),
                        m: (curr_min_m, curr_max_m),
                        a: ((rule.value + 1).max(curr_min_a), curr_max_a),
                        s: (curr_min_s, curr_max_s)});
                    curr_max_a = rule.value;    
                    if curr_max_a < curr_min_a {
                        apply_default = false;
                        break;
                    }
                },
                ('a', Operation::Less) if curr_min_a < rule.value => {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, curr_max_x),
                        m: (curr_min_m, curr_max_m),
                        a: (curr_min_a, (rule.value - 1).min(curr_max_a)),
                        s: (curr_min_s, curr_max_s)});
                    curr_min_a = rule.value;
                    if curr_max_a < curr_min_a {
                        apply_default = false;
                        break;
                    }
                },
                ('s', Operation::Greater) => if curr_max_s > rule.value {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, curr_max_x),
                        m: (curr_min_m, curr_max_m),
                        a: (curr_min_a, curr_max_a),
                        s: ((rule.value + 1).max(curr_min_s), curr_max_s)});
                    curr_max_s = rule.value;
                    if curr_max_s < curr_min_s {
                        apply_default = false;
                        break;
                    }
                },
                ('s', Operation::Less) if curr_min_s < rule.value => {
                    stack.push(PartRange{
                        workflow: rule.destination,
                        x: (curr_min_x, curr_max_x),
                        m: (curr_min_m, curr_max_m),
                        a: (curr_min_a, curr_max_a),
                        s: (curr_min_s, (rule.value - 1).min(curr_max_s))});
                    curr_min_s = rule.value;
                    if curr_max_s < curr_min_s {
                        apply_default = false;
                        break;
                    }
                },
                _ => {}
            }
        }

        if apply_default
            && (curr_max_x > curr_min_x)
            && (curr_max_m > curr_min_m)
            && (curr_max_a > curr_min_a)
            && (curr_max_s > curr_min_s) {

            stack.push(PartRange{
                workflow: def,
                x: (curr_min_x, curr_max_x),
                m: (curr_min_m, curr_max_m),
                a: (curr_min_a, curr_max_a),
                s: (curr_min_s, curr_max_s),
            });
        }
    }

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
use std::{fs, time::Instant, collections::{HashSet, HashMap, VecDeque}};

#[derive(Debug)]
enum Module {
    FlipFlop,
    Conjunction,
    Broadcaster
}

fn solve_part_1(input: &String) -> i64 {
    let repeat_count = 1000;

    let modules: HashMap<_, _> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line_split = line.split("->");
            let mut name = line_split
                .next()
                .unwrap()
                .trim();

            let module_type;
            match name.chars().next() {
                Some('%') => {
                    module_type = Module::FlipFlop;
                    name = &name[1..];
                },
                Some('&') => {
                    module_type = Module::Conjunction;
                    name = &name[1..];
                },
                _ => module_type = Module::Broadcaster
            }

            let outputs: Vec<_> = line_split
                .next()
                .unwrap()
                .split(",")
                .map(|dest| dest.trim())
                .collect();

            return (name, (module_type, outputs));
        })
        .collect();

    let mut flipflop_states: HashMap<&str, bool> = HashMap::new();
    let mut con_memory: HashMap<(&str, &str), bool> = HashMap::new();
    let mut con_inputs: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut queue: VecDeque<(&str, &str, bool)> = VecDeque::new();

    modules
        .iter()
        .for_each(|(name, (_, outputs))| outputs
            .iter()
            .for_each(|dest_name| match modules.get(dest_name) {
                Some((Module::Conjunction, _)) => {
                    con_inputs
                        .entry(dest_name)
                        .or_insert_with(|| HashSet::new())
                        .insert(name);
                },
                _ => {}
            }));
    let empty_inputs = HashSet::new();

    let mut low_signal_count = 0;
    let mut high_signal_count = 0;

    for _ in 0..repeat_count {
        queue.push_back(("broadcaster", "button", false));

        while let Some((signal_target, signal_source, signal_high)) = queue.pop_front() {
            if signal_high {
                high_signal_count += 1;
            }
            else {
                low_signal_count += 1;
            }

            match modules.get(signal_target) {
                Some((Module::Broadcaster, outputs)) => outputs
                    .iter()
                    .for_each(|dest| queue.push_back((dest, signal_target, signal_high))),
                Some((Module::FlipFlop, outputs)) => if !signal_high {
                    let state = !flipflop_states.get(signal_target).unwrap_or(&false);
                    flipflop_states.insert(signal_target, state);
                    outputs
                        .iter()
                        .for_each(|dest| queue.push_back((dest, signal_target, state)));
                },
                Some((Module::Conjunction, outputs)) => {
                    con_memory.insert((signal_target, signal_source), signal_high);

                    let all_inputs_high = con_inputs
                        .get(signal_target)
                        .unwrap_or_else(|| &empty_inputs)
                        .iter()
                        .all(|input| *con_memory
                            .get(&(signal_target, input))
                            .unwrap_or(&false));

                    let signal = !all_inputs_high;

                    outputs
                        .iter()
                        .for_each(|dest| queue.push_back((dest, signal_target, signal)));
                },
                None => {}
            }
        }
    }

    return low_signal_count * high_signal_count;
}

fn solve_part_2(input: &String) -> usize {
    fn lcm(a: usize, b: usize) -> usize {
        return a * b / gcd(a, b);
    }
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b > 0 {
            let m = a % b;
            a = b;
            b = m;
        }
        return a;
    }

    let modules: HashMap<_, _> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line_split = line.split("->");
            let mut name = line_split
                .next()
                .unwrap()
                .trim();

            let module_type;
            match name.chars().next() {
                Some('%') => {
                    module_type = Module::FlipFlop;
                    name = &name[1..];
                },
                Some('&') => {
                    module_type = Module::Conjunction;
                    name = &name[1..];
                },
                _ => module_type = Module::Broadcaster
            }

            let outputs: Vec<_> = line_split
                .next()
                .unwrap()
                .split(",")
                .map(|dest| dest.trim())
                .collect();

            return (name, (module_type, outputs));
        })
        .collect();

    let mut flipflop_states: HashMap<&str, bool> = HashMap::new();
    let mut con_memory: HashMap<(&str, &str), bool> = HashMap::new();
    let mut inputs: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut queue: VecDeque<(&str, &str, bool)> = VecDeque::new();

    modules
        .iter()
        .for_each(|(name, (_, outputs))| outputs
            .iter()
            .for_each(|dest_name| {
                inputs
                    .entry(dest_name)
                    .or_insert_with(|| HashSet::new())
                    .insert(name);
            }));
    let empty_inputs = HashSet::new();

    let source_modules: HashSet<&str> = inputs
        .get("rx")
        .unwrap_or(&empty_inputs)
        .iter()
        .flat_map(|input| inputs
            .get(input)
            .unwrap_or(&empty_inputs)
            .iter())
        .map(|x| *x)
        .collect();

    let mut button_press_count = 0;
    let mut source_low_receive_bpc = HashMap::new();

    'L0: loop {
        button_press_count += 1;
        queue.push_back(("broadcaster", "button", false));

        while let Some((signal_target, signal_source, signal_high)) = queue.pop_front() {
            if !signal_high && source_modules.contains(signal_target) {
                source_low_receive_bpc.insert(signal_target, button_press_count);
                if source_low_receive_bpc.len() >= source_modules.len() {
                    break 'L0;
                }
            }

            match modules.get(signal_target) {
                Some((Module::Broadcaster, outputs)) => outputs
                    .iter()
                    .for_each(|dest| queue.push_back((dest, signal_target, signal_high))),
                Some((Module::FlipFlop, outputs)) => if !signal_high {
                    let state = !flipflop_states.get(signal_target).unwrap_or(&false);
                    flipflop_states.insert(signal_target, state);
                    outputs
                        .iter()
                        .for_each(|dest| queue.push_back((dest, signal_target, state)));
                },
                Some((Module::Conjunction, outputs)) => {
                    con_memory.insert((signal_target, signal_source), signal_high);

                    let all_inputs_high = inputs
                        .get(signal_target)
                        .unwrap_or_else(|| &empty_inputs)
                        .iter()
                        .all(|input| *con_memory
                            .get(&(signal_target, input))
                            .unwrap_or(&false));

                    let signal = !all_inputs_high;

                    outputs
                        .iter()
                        .for_each(|dest| queue.push_back((dest, signal_target, signal)));
                },
                None => {}
            }
        }
    }

    return source_low_receive_bpc
        .values()
        .map(|v| *v)
        .reduce(|acc, e| lcm(acc, e))
        .unwrap_or(0);
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
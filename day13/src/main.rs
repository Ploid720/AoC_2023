use std::{fs, time::Instant};

fn solve_part_1(input: &String) -> usize {
    let mut row_sum = 0;
    let mut col_sum = 0;
    for pattern in input.replace("\r", "").split("\n\n") {
        let rows: Vec<_> = pattern.lines().collect();
        let cols: Vec<String> = transpose_lines(pattern)
            .map(|it| it.collect())
            .collect();

        row_sum += (1..(rows.len()))
            .filter(|i| (0..((rows.len() - i).min(*i)))
                .all(|j| rows[i + j] == rows[i - j - 1]))
            .sum::<usize>();

        col_sum += (1..(cols.len()))
            .filter(|i| (0..((cols.len() - i).min(*i)))
                .all(|j| cols[i + j] == cols[i - j - 1]))
            .sum::<usize>();
    }

    return col_sum + 100 * row_sum;
}

fn solve_part_2(input: &String) -> usize {
    let mut row_sum = 0;
    let mut col_sum = 0;
    for pattern in input.replace("\r", "").split("\n\n") {
        let rows: Vec<_> = pattern.lines().collect();
        let cols: Vec<String> = transpose_lines(pattern)
            .map(|it| it.collect())
            .collect();

        row_sum += (1..(rows.len()))
            .filter(|i| (0..((rows.len() - i).min(*i)))
                .map(|j| rows[i + j]
                    .chars()
                    .zip(rows[i - j - 1].chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count())
                .sum::<usize>() == 1)
            .sum::<usize>();

        col_sum += (1..(cols.len()))
            .filter(|i| (0..((cols.len() - i).min(*i)))
                .map(|j| cols[i + j]
                    .chars()
                    .zip(cols[i - j - 1].chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count())
                .sum::<usize>() == 1)
            .sum::<usize>();
    }

    return col_sum + 100 * row_sum;
}

fn transpose_lines(str: &str) 
    -> impl Iterator<Item = impl Iterator<Item = char> + '_>
{
    return (0 ..)
        .scan((), move |&mut (), row_idx| {
            let mut col_iter = str.lines();
            let first_col = col_iter.next()?;
            let first = first_col
                .chars()
                .nth(row_idx)?;
            return Some(Iterator::chain(
                std::iter::once(first),
                col_iter
                    .filter_map(move |column| {
                        column
                            .chars()
                            .nth(row_idx)
                    })
            ));
        })
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
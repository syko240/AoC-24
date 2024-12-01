use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let (left, right) = parse(input);

    let mut right_count = HashMap::new();
    for &val in &right {
        *right_count.entry(val).or_insert(0) += 1;
    }

    let sim_score: i32 = left.iter()
        .map(|&left_val| left_val * right_count.get(&left_val).unwrap_or(&0))
        .sum();

    sim_score.to_string()
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if let Some((left_val, right_val)) = line.split_once(char::is_whitespace) {
            left.push(left_val.trim().parse::<i32>().unwrap());
            right.push(right_val.trim().parse::<i32>().unwrap());
        }
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("\
        3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, "31".to_string());
    }
}
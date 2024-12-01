fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let tot_dist: i32 = left.iter()
        .zip(right.iter())
        .map(|(a,b)| (a-b).abs())
        .sum();

    tot_dist.to_string()
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
        assert_eq!(result, "11".to_string());
    }
}
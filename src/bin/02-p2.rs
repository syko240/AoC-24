fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let levels = parse(input);

    let output: usize = levels
        .iter()
        .filter(|l| rm_l(l))
        .count();

    output.to_string()
}

fn rm_l(l: &[i32]) -> bool {
    if cond(l) {
        return true;
    }
    for i in 0..l.len() {
        let mut new_l = l.to_vec();
        new_l.remove(i);
        if cond(&new_l) {
            return true;
        }
    }
    false
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();

    for line in input.lines() {
        let l: Vec<i32> = line
            .split_whitespace()
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        levels.push(l)
    }
    levels
}

fn cond(l: &[i32]) -> bool {
    l
        .windows(2)
        .all(|v| v[1] > v[0] && (v[1] - v[0]).abs() <= 3)
    || l
        .windows(2)
        .all(|v| v[1] < v[0] && (v[1] - v[0]).abs() <= 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("\
        7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, "4".to_string());
    }
}
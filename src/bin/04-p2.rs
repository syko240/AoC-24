fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let map = parse(input);
    let w = "MAS";

    let mut count = 0;
    for x in 1..map.len() -1 {
        for y in 1..map[0].len() - 1 {
            count += lookup(&map, w, x, y);
        }
    }
    count.to_string()
}

fn lookup(map: &Vec<Vec<char>>, w: &str, x: usize, y: usize) -> usize {
    let w_rev: Vec<char> = w.chars().rev().collect();

    let left_right = vec![ map[x-1][y-1], map[x][y], map[x+1][y+1] ];
    let right_left = vec![ map[x-1][y+1], map[x][y], map[x+1][y-1] ];

    if (left_right == w.chars().collect::<Vec<_>>() || left_right == w_rev) && (right_left == w.chars().collect::<Vec<_>>() || right_left == w_rev) {
        1
    } else {
        0
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("\
        .M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........");
        assert_eq!(result, "9".to_string());
    }
}
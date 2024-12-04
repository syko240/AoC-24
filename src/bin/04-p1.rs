fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let map = parse(input);
    let w = "XMAS";
    let dirs = [(1, 1), (-1, -1), (1, 0), (0, 1), (-1, 0), (0, -1), (-1, 1), (1, -1)];

    let mut count = 0;
    for x in 0..map.len() as isize {
        for y in 0..map[0].len() as isize {
            for &(dx, dy) in &dirs {
                count += lookup(&map, w, x, y, dx, dy);
            }
        }
    }
    count.to_string()
}

fn lookup(map: &Vec<Vec<char>>, w: &str, mut x: isize, mut y: isize, dx: isize, dy: isize) -> usize {
    let chars: Vec<char> = w.chars().collect();

    for idx in 0..chars.len() as isize {
        if x < 0 || x >= map.len() as isize || y < 0 || y >= map[0].len() as isize {
            return 0;
        }
        if map[x as usize][y as usize] != chars[idx as usize] {
            return 0;
        }
        x += dx;
        y += dy;
    }
    1
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
        MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(result, "18".to_string());
    }
}
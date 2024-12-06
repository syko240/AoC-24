use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    println!("{}", output);
}

fn aoc_solve(input: &str) -> String {
    let grid = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut pos = (0, 0);
    let mut dir = (0, -1);
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_i = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                '^' => { pos = (x, y); dir = dirs[0]; dir_i = 0; }
                '>' => { pos = (x, y); dir = dirs[1]; dir_i = 1; }
                'v' => { pos = (x, y); dir = dirs[2]; dir_i = 2; }
                '<' => { pos = (x, y); dir = dirs[3]; dir_i = 3; }
                _ => {}
            }
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(pos);

    loop {
        let (dx, dy) = dir;
        let pos_x = pos.0 as isize + dx;
        let pos_y = pos.1 as isize + dy;

        if pos_x < 0 || pos_y < 0 || pos_y as usize >= grid.len() || pos_x as usize >= grid[0].len() {
            break;
        }

        if grid[pos_y as usize][pos_x as usize] == '#' {
            dir_i = (dir_i + 1) % 4;
            dir = dirs[dir_i];
            continue;
        }
        pos = (pos_x as usize, pos_y as usize);
        visited.insert(pos);
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        let input = "\
    ....#.....\n\
.........#\n\
..........\n\
..#.......\n\
.......#..\n\
..........\n\
.#..^.....\n\
........#.\n\
#.........\n\
......#...";
        let result = aoc_solve(input);
        assert_eq!(result, "41".to_string());
    }
}

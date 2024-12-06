use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    println!("{}", output);
}

fn aoc_solve(input: &str) -> String {
    let grid = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut start_pos = (0, 0);
    let mut dir = (0, -1);
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_i = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                '^' => { start_pos = (x, y); dir = dirs[0]; dir_i = 0; }
                '>' => { start_pos = (x, y); dir = dirs[1]; dir_i = 1; }
                'v' => { start_pos = (x, y); dir = dirs[2]; dir_i = 2; }
                '<' => { start_pos = (x, y); dir = dirs[3]; dir_i = 3; }
                _ => {}
            }
        }
    }
    
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if (x, y) == start_pos || grid[y][x] == '#' {
                continue;
            }

            let mut new_grid = grid.clone();
            new_grid[y][x] = '#';

            if check_it(&new_grid, start_pos, dir, &dirs, dir_i) {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn check_it(grid: &Vec<Vec<char>>, mut pos: (usize, usize), mut dir: (isize, isize), dirs: &[(isize, isize)], mut dir_i: usize) -> bool {
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

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
        if !visited.insert((pos.0, pos.1, dir_i)) {
            return true;
        }
    }

    return false;
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
        assert_eq!(result, "6".to_string());
    }
}

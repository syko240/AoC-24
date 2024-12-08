use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    y: isize,
    x: isize,
}

impl Point {
    fn new(y: isize, x: isize) -> Self {
        Point { y, x }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let antennas = parse(input);
    println!("{antennas:?}");
    let (n_y, n_x) = parse2(input);
    let mut anti: HashSet<Point> = HashSet::new();

    for (_freq, pos) in &antennas {
        for i in 0..pos.len() {
            for j in i+1..pos.len() {
                let p1 = pos[i];
                let p2 = pos[j];

                let a1 = Point::new(2 * p2.y - p1.y, 2 * p2.x - p1.x);
                let a2 = Point::new(2 * p1.y - p2.y, 2 * p1.x - p2.x);

                if a1.y >= 0 && a1.y < n_y && a1.x >= 0 && a1.x < n_x {
                    anti.insert(a1);
                }
                if a2.y >= 0 && a2.y < n_y && a2.x >= 0 && a2.x < n_x {
                    anti.insert(a2);
                }
            }
        }
    }

    anti.len().to_string()
}

fn parse(input: &str) -> HashMap<char, Vec<Point>> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.entry(char).or_default().push(Point::new(y as isize, x as isize));
            }
        }
    }
    antennas
}

fn parse2(input: &str) -> (isize, isize) {
    let lines= input.lines().collect::<Vec<&str>>();
    let n_y = lines.len();
    let n_x = lines.get(0).map(|line| line.len()).unwrap();
    (n_y as isize, n_x as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("\
        ............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............");
        assert_eq!(result, "14".to_string());
    }
}
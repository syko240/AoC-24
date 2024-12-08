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
        if pos.len() < 2 {
            continue;
        }
        for i in 0..pos.len() {
            for j in i+1..pos.len() {
                let p1 = pos[i];
                let p2 = pos[j];

                let dy = p2.y - p1.y;
                let dx = p2.x - p1.x;

                let gcd = gcd(dx, dy);
                let step = Point::new(dy / gcd, dx / gcd);

                let mut start = p1;
                loop {
                    let next = Point::new(start.y - step.y, start.x - step.x);
                    if next.y < 0 || next.y >= n_y || next.x < 0 || next.x >= n_x {
                        break;
                    }
                    start = next;
                }

                let mut current = start;
                loop {
                    if current.y < 0 || current.y >= n_y || current.x < 0 || current.x >= n_x {
                        break;
                    }
                    anti.insert(Point::new(current.y, current.x));
                    current = Point::new(current.y + step.y, current.x + step.x);

                }

                //println!("{dy} {dx}");
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

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
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
        assert_eq!(result, "34".to_string());
    }
}
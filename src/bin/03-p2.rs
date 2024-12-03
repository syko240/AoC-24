use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let vec = parse(input);
    let mut sum = 0;
    let mut skip = false;

    for val in vec {
        if val == "do()" {
            skip = false;
        } else if val == "don't()" {
            skip = true;
        } else {
            if skip {
                continue;
            }
            let operands: Vec<&str> = val[4..val.len()-1].split(',').collect();
            sum += operands[0].parse::<i32>().unwrap() * operands[1].parse::<i32>().unwrap();
        }
    }
    sum.to_string()
}

fn parse(input: &str) -> Vec<&str> {
    let mul_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut vec = Vec::new();
    for mat in mul_re.find_iter(input) {
        vec.push(mat.as_str());
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48".to_string());
    }
}
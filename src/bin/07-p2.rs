fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

enum Op {
    Mul(u64, u64),
    Add(u64, u64),
    Concat(u64, u64),
}

impl Op {
    fn eval(&self) -> u64 {
        match self {
            Op::Mul(left, right) => left * right,
            Op::Add(left, right) => left + right,
            Op::Concat(left, right) => (format!("{}{}", left, right)).parse::<u64>().unwrap(),
        }
    }
}

fn aoc_solve(input: &str) -> String {
    let (cal_vals, operands) = parse(input);
    let mut tot = 0;

    let operators: Vec<fn(u64, u64) -> Op> = vec![
        |a, b| Op::Add(a, b),
        |a, b| Op::Mul(a, b),
        |a, b| Op::Concat(a, b),
    ];

    for (i, &cal_val) in cal_vals.iter().enumerate() {
        let operand = &operands[i];
        let ops = operand.len() - 1;

        for k in 0..(operators.len().pow(ops as u32)) {
            let mut current = operand[0];
            let mut tmp_k = k;

            for j in 0..ops {
                let op_i = tmp_k % operators.len();
                tmp_k /= operators.len();
                let op = operators[op_i](current, operand[j+1]);
                current = op.eval();

                if current > cal_val {
                    break;
                }
            }

            if current == cal_val {
                tot += cal_val;
                break;
            }
        }
    }

    tot.to_string()
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut cal_vals: Vec<u64> = Vec::new();
    let mut operands: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once(':') {
            if let Ok(val) = left.trim().parse::<u64>() {
                cal_vals.push(val);
            }

            let operand = right.trim()
                .split_whitespace()
                .filter_map(|val| val.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            operands.push(operand);
        }
    }
    (cal_vals, operands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("\
        190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20");
        assert_eq!(result, "11387".to_string());
    }
}
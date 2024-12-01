fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("");
        assert_eq!(result, "".to_string());
    }
}
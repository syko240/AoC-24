use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = aoc_solve(input);
    dbg!(output);
}

fn aoc_solve(input: &str) -> String {
    let (ord_rules, updates) = parse(input);

    let mut sum = 0;

    for update in updates {
        let pos = update.iter().enumerate().map(|(i, &page)| (page, i)).collect::<HashMap<u32, usize>>();

        let is_valid = ord_rules.iter().filter(|&&(a, b)| pos.contains_key(&a) && pos.contains_key(&b)).all(|&(a, b)| pos[&a] < pos[&b]);
        if !is_valid {
            let reordered_update = reorder(&update, &ord_rules);

            sum += reordered_update[reordered_update.len()/2];
        }
    }

    sum.to_string()
}

fn reorder(pos: &Vec<u32>, ord_rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut depends = HashMap::new();

    for &(a, b) in ord_rules {
        *depends.entry(b).or_insert(0) += 1;
        depends.entry(a).or_insert(0);
    }

    let mut sorted = pos.clone();
    sorted.sort_by(|&x, &y| {
        let dx = depends.get(&x).copied().unwrap_or(0);
        let dy = depends.get(&y).copied().unwrap_or(0);

        if dx < dy {
            std::cmp::Ordering::Less
        } else if dx > dy {
            std::cmp::Ordering::Greater
        } else {
            if ord_rules.iter().any(|&(a, b)| a == x && b == y) {
                std::cmp::Ordering::Less
            } else if ord_rules.iter().any(|&(a, b)| a == y && b == x) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    });

    sorted
}

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let inp = input.replace("\r\n", "\n");
    let split = inp.trim().split("\n\n").collect::<Vec<&str>>();

    let ord_rules = split[0].lines().map(|line| line.trim()).filter(|line| !line.is_empty()).map(|line| {
        let rules = line.trim().split('|').collect::<Vec<&str>>();
        //println!("{rules:?}");
        (rules[0].parse().unwrap(), rules[1].parse().unwrap())
    }).collect::<Vec<(u32, u32)>>();

    let updates = split[1].lines().map(|line| {
        line.trim().split(',').map(|update| update.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    (ord_rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() {
        let result = aoc_solve("\
        47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        assert_eq!(result, "123".to_string());
    }
}
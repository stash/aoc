use itertools::Itertools;
use std::str::Split;

use anyhow::{bail, Result};

struct RuleIndex {
    rules: Vec<String>,
}

impl RuleIndex {
    pub fn new(mut in_rules: Vec<String>) -> Self {
        in_rules.sort();
        Self { rules: in_rules }
    }

    pub fn contains(&self, item: &String) -> bool {
        self.rules.binary_search_by(|x| x.cmp(item)).is_ok()
    }
}

/// returns middle item if all the pages are in correct "order" relative to rules
fn check_one_production<'a>(index: &RuleIndex, pages: Vec<&'a str>) -> Option<&'a str> {
    for (a, b) in pages.iter().tuple_combinations() {
        let c = format!("{}|{}", a, b);
        let has = index.contains(&c);
        println!("p {} -> {}", c, has);
        if !has {
            return None;
        }
    }

    let mid = pages.len() / 2;
    Some(pages[mid])
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let (to_produce, mut rules): (Vec<_>, Vec<_>) =
        lines.into_iter().partition(|x| x.contains(','));
    _ = rules.pop();
    let index = RuleIndex::new(rules);
    let mut total: usize = 0;
    for p in to_produce {
        println!("Checking: {}", p);
        let pages: Vec<&str> = p.split(',').collect();
        match check_one_production(&index, pages) {
            Some(n) => {
                println!("Some: {}", n);
                total += n.parse::<usize>()?
            }
            None => {
                println!("None!")
            }
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    bail!("incomplete")
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input: Vec<String> = indoc! {"
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
            97,13,75,29,47
        "}
        .lines()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part1(input)?, "143");
        Ok(())
    }
}

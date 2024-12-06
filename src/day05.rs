use anyhow::Result;
use std::cmp::Ordering;

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

fn prep(lines: Vec<String>) -> (Vec<String>, RuleIndex) {
    let (to_produce, mut rules): (Vec<_>, Vec<_>) =
        lines.into_iter().partition(|x| x.contains(','));
    _ = rules.pop();
    let index = RuleIndex::new(rules);
    (to_produce, index)
}

fn mid_num(pages: &Vec<&str>) -> Result<usize> {
    let mid = pages.len() / 2;
    Ok(pages[mid].parse::<usize>()?)
}

fn rule_sort<'a>(pages: &Vec<&'a str>, index: &RuleIndex) -> Vec<&'a str> {
    let mut sorted: Vec<&str> = pages.clone();
    sorted.sort_by(|a, b| {
        let c = format!("{}|{}", a, b);
        if index.contains(&c) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    sorted
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let (to_produce, index) = prep(lines);
    let mut total: usize = 0;
    for p in to_produce {
        println!("Checking: {}", p);
        let pages: Vec<&str> = p.split(',').collect();
        let sorted = rule_sort(&pages, &index);
        if pages == sorted {
            // count already sorted
            total += mid_num(&pages)?
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let (to_produce, index) = prep(lines);
    let mut total: usize = 0;
    for p in to_produce {
        println!("Checking: {}", p);
        let pages: Vec<&str> = p.split(',').collect();
        let sorted = rule_sort(&pages, &index);
        if pages != sorted {
            // count non-sorted
            total += mid_num(&sorted)?
        }
    }
    Ok(total.to_string())
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

    #[test]
    fn test_part2() -> Result<()> {
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

        assert_eq!(part2(input)?, "123");
        Ok(())
    }
}

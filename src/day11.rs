use anyhow::{anyhow, Result};
use std::collections::HashMap;

fn parse(lines: Vec<String>) -> Result<Vec<usize>> {
    let line = lines.first().ok_or_else(|| anyhow!("no lines?"))?;
    let stones = line
        .split_whitespace()
        .map(|s| s.parse().expect("int"))
        .collect();
    Ok(stones)
}

fn blink(s: usize) -> (usize, Option<usize>) {
    if s == 0 {
        return (1, None);
    }

    let digits = s.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 == 0 {
        let d = 10_usize.pow(digits >> 1);
        (s / d, Some(s % d))
    } else {
        (s * 2024, None)
    }
}

fn blink_all(stones: Vec<usize>) -> Result<Vec<usize>> {
    let mut next = Vec::new();
    for s in stones {
        let (first, second) = blink(s);
        next.push(first);
        if let Some(second) = second {
            next.push(second);
        }
    }
    Ok(next)
}

/// returns total stones after specified blinks and initial stone.
fn blink_n(s: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(count) = memo.get(&(s, blinks)) {
        return *count;
    }

    let mut count = 0;
    let (first, second) = blink(s);

    count += blink_n(first, blinks - 1, memo);
    if let Some(second) = second {
        count += blink_n(second, blinks - 1, memo);
    }

    memo.insert((s, blinks), count);
    count
}

pub fn part1(lines: Vec<String>, blinks: usize) -> Result<String> {
    let mut stones = parse(lines)?;
    for _ in 0..blinks {
        stones = blink_all(stones)?;
    }
    let total: usize = stones.into_iter().count();
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>, blinks: usize) -> Result<String> {
    let mut memo = HashMap::new();

    let mut total = 0;
    for stone in parse(lines)?.into_iter() {
        total += blink_n(stone, blinks, &mut memo);
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;
    use itertools::Itertools;

    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    fn ints_to_str(stones: &Vec<usize>) -> String {
        stones.into_iter().map(|s| s.to_string()).join(" ")
    }

    #[test]
    fn test_part1_a() -> Result<()> {
        let stones = parse(vec!["0 1 10 99 999".to_owned()])?;
        let result = blink_all(stones)?;
        assert_eq!(ints_to_str(&result), "1 2024 1 0 9 9 2021976");
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let lines = lines(indoc! {"
            253000 1 7
            253 0 2024 14168
            512072 1 20 24 28676032
            512 72 2024 2 0 2 4 2867 6032
            1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
            2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
        "});
        let mut stones = parse(vec!["125 17".to_owned()])?;

        for expect in lines {
            stones = blink_all(stones)?;
            assert_eq!(ints_to_str(&stones), expect);
        }
        Ok(())
    }

    #[test]
    fn test_part2_a() -> Result<()> {
        assert_eq!(part2(vec!["0 1 10 99 999".to_owned()], 1)?, "7");
        Ok(())
    }

    #[test]
    fn test_part2_b() -> Result<()> {
        assert_eq!(part2(vec!["253000 1 7".to_owned()], 1)?, "4");
        assert_eq!(part2(vec!["253000 1 7".to_owned()], 2)?, "5");
        assert_eq!(part2(vec!["253000 1 7".to_owned()], 3)?, "9");
        assert_eq!(part2(vec!["253000 1 7".to_owned()], 4)?, "13");
        assert_eq!(part2(vec!["253000 1 7".to_owned()], 5)?, "22");
        Ok(())
    }
}

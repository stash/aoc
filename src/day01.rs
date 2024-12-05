use anyhow::{bail, Result};
use std::iter::zip;

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts.len() > 2 {
            bail!("line too long")
        }
        a.push(parts[0].parse()?);
        b.push(parts[1].parse()?);
        //println!("{}", parts.join(","))
    }
    a.sort();
    b.sort();
    let c = zip(a, b);
    let sum_diff: u32 = c.map(|pair| pair.0.abs_diff(pair.1)).sum();
    Ok(sum_diff.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts.len() > 2 {
            bail!("line too long")
        }
        a.push(parts[0].parse()?);
        b.push(parts[1].parse()?);
        //println!("{}", parts.join(","))
    }
    let mut total: usize = 0;
    for x in &a {
        let mut count: usize = 0;
        for y in &b {
            if x == y {
                count += 1
            }
        }
        total += x * count;
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input: Vec<String> = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "}
        .lines()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part1(input)?, "11");
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input: Vec<String> = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "}
        .lines()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(part2(input)?, "31");
        Ok(())
    }
}

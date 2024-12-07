use anyhow::{anyhow, bail, Result};

fn parse(line: String) -> Result<(usize, Vec<usize>)> {
    let (cal_str, rest_str) = line.split_once(':').ok_or_else(|| anyhow!("no colon"))?;
    let cal = cal_str.parse::<usize>()?;
    let mut nums: Vec<usize> = Vec::new();
    for x in rest_str.split_whitespace() {
        nums.push(x.parse::<usize>()?)
    }
    if nums.len() < 2 {
        bail!("too short!")
    }
    Ok((cal, nums))
}

fn cat_base10(x: usize, y: usize) -> usize {
    let exp = y.checked_ilog10().unwrap_or(0);
    y + x * 10_usize.pow(exp + 1)
}

fn check_calibration(line: String, part_two: bool) -> Result<usize> {
    let (cal, nums) = parse(line)?;

    // accumulation of prior values:
    let mut acc: Vec<usize> = {
        let first = nums.first().ok_or_else(|| anyhow!("Too few numbers"))?;
        vec![*first]
    };
    for y in nums.iter().skip(1) {
        let mut next_acc = Vec::new();
        for x in acc {
            next_acc.push(x + y);
            next_acc.push(x * y);
            if part_two {
                next_acc.push(cat_base10(x, *y));
            }
        }
        acc = next_acc;
    }

    Ok(if acc.into_iter().any(|x| x == cal) {
        cal
    } else {
        0
    })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut total: usize = 0;
    for line in lines {
        total += check_calibration(line, false)?;
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut total: usize = 0;
    for line in lines {
        total += check_calibration(line, true)?;
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;
    fn input() -> Vec<String> {
        indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "}
        .lines()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(input())?, "3749");
        Ok(())
    }

    #[test]
    fn test_cat_base10() -> Result<()> {
        assert_eq!(cat_base10(22, 33), 2233);
        assert_eq!(cat_base10(22, 0), 220);
        assert_eq!(cat_base10(0, 0), 0);
        assert_eq!(cat_base10(0, 1), 1);
        assert_eq!(cat_base10(0, 11), 11);
        assert_eq!(cat_base10(1, 1), 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(input())?, "11387");
        Ok(())
    }
}

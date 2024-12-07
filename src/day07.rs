use anyhow::{bail, Result};

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut total: usize = 0;
    for line in lines {
        let (cal_str, rest_str) = line.split_once(':').expect("should have colon");

        let cal = cal_str.parse::<usize>()?;
        let mut nums: Vec<usize> = Vec::new();
        for x in rest_str.split_whitespace() {
            nums.push(x.parse::<usize>()?)
        }
        if nums.len() < 2 {
            bail!("too short!")
        }

        let mut acc: Vec<usize> = {
            let first = nums.first().expect("numbers!");
            vec![*first]
        };
        for y in nums.iter().skip(1) {
            let mut next_acc = Vec::new();
            for x in acc {
                // TODO: could filter z <= cal here
                next_acc.push(x + y);
                next_acc.push(x * y);
            }
            acc = next_acc;
        }
        println!("acc: {:?}", acc);

        if acc.into_iter().any(|x| x == cal) {
            println!("ok {}", cal);
            total += cal;
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    bail!("not implemented")
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input: Vec<String> = indoc! {"
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
        .collect();

        assert_eq!(part1(input)?, "3749");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input: Vec<String> = indoc! {"
        "}
        .lines()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part2(input)?, "");
        Ok(())
    }
}

use anyhow::{bail, Result};

const MASK_PRUNE: u64 = 0xFFFFFF;

fn turn(x: u64) -> u64 {
    let x = ((x << 6) ^ x) & MASK_PRUNE;
    let x = ((x >> 5) ^ x) & MASK_PRUNE;
    let x = ((x << 11) ^ x) & MASK_PRUNE;
    x
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut state: Vec<u64> = lines.into_iter().map(|y| y.parse().unwrap()).collect();
    for _n in 0usize..2000 {
        for i in 0usize..state.len() {
            state[i] = turn(state[i])
        }
    }
    println!("states: {:?}", state);
    Ok(state
        .into_iter()
        .map(|x| x as usize)
        .sum::<usize>()
        .to_string())
}

pub fn part2(_lines: Vec<String>) -> Result<String> {
    bail!("not done")
}

#[cfg(test)]
mod test {

    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn to_lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1_first_steps() -> Result<()> {
        let expect = to_lines(indoc! {"
            15887950
            16495136
            527345
            704524
            1553684
            12683156
            11100544
            12249484
            7753432
            5908254
        "})
        .into_iter()
        .map(|y| y.parse::<u64>().unwrap());
        let mut x = 123;
        for y in expect {
            x = turn(x);
            assert_eq!(y, x);
        }
        Ok(())
    }

    // #[test]
    // fn test_period() -> Result<()> {
    //     let mut seen = HashSet::new();
    //     let mut x = 123;
    //     seen.insert(x);
    //     for n in 0usize.. {
    //         x = turn(x);
    //         if !seen.insert(x) {
    //             println!("period {}", n);
    //             break;
    //         }
    //     }
    //     Ok(())
    // }

    #[test]
    fn test_part1() {
        let lines = to_lines(indoc! {"
            1
            10
            100
            2024
        "});
        assert_eq!(part1(lines).unwrap(), "37327623");
    }
}

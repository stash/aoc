use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Chal<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
    pat_idx: HashSet<&'a str>,
    max_len: usize,
}

impl<'a> Chal<'a> {
    fn parse(lines: &'a Vec<String>) -> Result<Chal> {
        let mut iter = lines.into_iter();
        let patterns: Vec<&str> = iter.next().unwrap().split(", ").collect();
        iter.next();
        let designs = iter.map(|s| s.as_ref()).collect();
        let mut pat_idx = HashSet::new();
        let mut max_len = 0;
        for p in &patterns {
            max_len = max_len.max(p.len());
            pat_idx.insert(*p);
        }
        Ok(Chal {
            patterns,
            designs,
            pat_idx,
            max_len,
        })
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let c = Chal::parse(&lines)?;
    let re = {
        let all_pats = c.patterns.join("|");
        let giga_re_str = format!("^({})+$", all_pats);
        Regex::new(&giga_re_str)?
    };
    let mut total = 0;
    for d in c.designs {
        if re.is_match(&d) {
            total += 1;
        }
    }
    Ok(total.to_string())
}

fn rec_find<'a>(d: &'a str, c: &Chal, cache: &mut HashMap<&'a str, usize>) -> usize {
    if d.len() == 1 && c.pat_idx.contains(d) {
        return 1;
    } else if d.len() == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(d) {
        // println!("cached {} {}", d, cached);
        return *cached;
    }

    // println!("fresh {}", d);
    let mut total = 0;
    let max = c.max_len.min(d.len()) - 1; // only look for prefixes up to the max pattern length
    for n in 0..=max {
        // shortest prefixes first?
        let prefix = &d[0..=n];
        if c.pat_idx.contains(prefix) {
            let suffix = &d[n + 1..];
            let add = rec_find(suffix, c, cache);
            // println!("found {} = {}|{}", add, prefix, suffix);
            total += add;
        }
    }
    // println!("done {} = {}", total, d);
    cache.insert(d, total);
    total
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let c = Chal::parse(&lines)?;
    let all: usize = c
        .designs
        .iter()
        .map(|d| {
            let mut cache = HashMap::new();
            rec_find(d, &c, &mut cache)
        })
        .sum();
    Ok(all.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        "});
        assert_eq!(part1(lines)?, "6");
        Ok(())
    }

    #[test]
    fn test_part2_a() -> Result<()> {
        let lines = lines(indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
        "});
        assert_eq!(part2(lines)?, "2");
        Ok(())
    }

    #[test]
    fn test_part2_b() -> Result<()> {
        let lines = lines(indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            bggr
        "});
        assert_eq!(part2(lines)?, "1");
        Ok(())
    }

    #[test]
    fn test_part2_c() -> Result<()> {
        let lines = lines(indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            gbbr
        "});
        assert_eq!(part2(lines)?, "4");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let lines = lines(indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        "});
        assert_eq!(part2(lines)?, "16");
        Ok(())
    }
}

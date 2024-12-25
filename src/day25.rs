use anyhow::{bail, Result};

pub struct LockOrKey {
    pins: Vec<usize>,
}
impl LockOrKey {
    pub fn default() -> Self {
        Self { pins: vec![0; 5] }
    }
    pub fn fits(&self, other: &LockOrKey) -> bool {
        for i in 0..5 {
            if self.pins[i] + other.pins[i] > 5 {
                // overlap
                return false;
            }
        }
        true
    }
    pub fn invert(&self) -> Self {
        Self {
            pins: self.pins.iter().map(|p| 5 - p).collect(),
        }
    }
}

pub struct Chal {
    locks: Vec<LockOrKey>,
    keys: Vec<LockOrKey>,
}

impl Chal {
    pub fn parse(lines: Vec<String>) -> Result<Chal> {
        let mut locks = vec![];
        let mut keys = vec![];
        let mut iter = lines.into_iter();
        loop {
            let mut cur = LockOrKey::default();
            let is_lock = if let Some(type_line) = iter.next() {
                type_line == "#####"
            } else {
                false
            };

            for _ in 0..5 {
                let line = iter.next().unwrap();
                for (i, c) in line.chars().enumerate() {
                    match c {
                        '.' => cur.pins[i] += 0,
                        '#' => cur.pins[i] += 1,
                        _ => bail!("invalid pin char"),
                    }
                }
            }
            iter.next(); // ignore bottom line

            if is_lock {
                locks.push(cur);
            } else {
                keys.push(cur);
            }
            if !iter.next().is_some_and(|x| x == "") {
                break;
            }
        }

        Ok(Chal { locks, keys })
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let chal = Chal::parse(lines)?;
    let mut total: usize = 0;
    for lock in chal.locks {
        for key in chal.keys.iter() {
            if lock.fits(key) {
                total += 1;
            }
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    bail!("not done")
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;

    fn part1_lines() -> Vec<String> {
        let text = indoc! {"
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....

            #####
            ##.##
            .#.##
            ...##
            ...#.
            ...#.
            .....

            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####

            .....
            .....
            #.#..
            ###..
            ###.#
            ###.#
            #####

            .....
            .....
            .....
            #....
            #.#..
            #.#.#
            #####
        "};
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1_parse() -> Result<()> {
        let chal = Chal::parse(part1_lines())?;
        assert_eq!(chal.keys.len(), 3);
        assert_eq!(chal.keys[0].pins, vec![5, 0, 2, 1, 3]);
        assert_eq!(chal.keys[1].pins, vec![4, 3, 4, 0, 2]);
        assert_eq!(chal.keys[2].pins, vec![3, 0, 2, 0, 1]);
        assert_eq!(chal.locks.len(), 2);
        assert_eq!(chal.locks[0].pins, vec![0, 5, 3, 4, 3]);
        assert_eq!(chal.locks[1].pins, vec![1, 2, 0, 5, 3]);

        assert!(!chal.locks[0].fits(&chal.keys[0]));
        assert!(!chal.locks[0].fits(&chal.keys[1]));
        assert!(chal.locks[0].fits(&chal.keys[2]));
        assert!(!chal.locks[1].fits(&chal.keys[0]));
        assert!(chal.locks[1].fits(&chal.keys[1]));
        assert!(chal.locks[1].fits(&chal.keys[2]));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(part1_lines())?, "3");
        Ok(())
    }
}

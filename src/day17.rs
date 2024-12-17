use anyhow::{bail, Result};
use itertools::join;

#[derive(Clone, Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    mem: Vec<u8>,
    out: Vec<u8>,
}
impl Computer {
    fn read(&mut self) -> Result<usize> {
        if let Some(v) = self.mem.get(self.ip) {
            self.ip += 1;
            Ok(*v as usize)
        } else {
            bail!("halt")
        }
    }

    fn combo(&mut self) -> Result<usize> {
        let v = self.read()?;
        Ok(match v {
            0 | 1 | 2 | 3 => v as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => bail!("invalid combo operand {}", v),
        })
    }

    fn shr(&mut self) -> Result<usize> {
        let v = self.combo()?;
        Ok(self.a >> v)
    }

    fn simulate(&mut self, two: bool) -> Result<()> {
        if self.ip >= self.mem.len() {
            bail!("halt");
        }

        let opcode = self.read()?;
        match opcode {
            0 => {
                // adv (really "right shift")
                self.a = self.shr()?;
                // println!("adv {:?}", self);
            }
            1 => {
                // bxl
                let x = self.b;
                let y = self.read()?;
                self.b = x ^ y;
                // println!("bxl {:?}", self);
            }
            2 => {
                // bst
                self.b = self.combo()? % 8;
                // println!("bst {:?}", self);
            }
            3 => {
                // jnz
                if self.a != 0 {
                    let ip = self.read()?;
                    self.ip = ip;
                }
                // println!("jnz {:?}", self);
            }
            4 => {
                // bxc
                self.b = self.b ^ self.c;
                _ = self.read()?; // "legacy reasons"
                                  // println!("bxc {:?}", self);
            }
            5 => {
                // out
                let v = self.combo()?;
                self.out.push((v % 8).try_into()?);
                // println!("out {:?}", self);
                if two && !self.mem.starts_with(&self.out) {
                    bail!("early")
                }
            }
            6 => {
                // bdv
                self.b = self.shr()?;
                // println!("bdv {:?}", self);
            }
            7 => {
                // cdv
                self.c = self.shr()?;
                // println!("cdv {:?}", self);
            }
            _ => {}
        }
        Ok(())
    }
}

fn parse(lines: Vec<String>) -> Result<Computer> {
    let mut iter = lines.into_iter();
    let a = iter.next().unwrap().split_once(": ").unwrap().1.parse()?;
    let b = iter.next().unwrap().split_once(": ").unwrap().1.parse()?;
    let c = iter.next().unwrap().split_once(": ").unwrap().1.parse()?;
    iter.next();
    let mut mem = Vec::new();
    for digit in iter.next().unwrap().split_once(": ").unwrap().1.split(',') {
        mem.push(digit.parse()?);
    }

    Ok(Computer {
        a,
        b,
        c,
        ip: 0,
        mem,
        out: Vec::new(),
    })
}

fn part1_sim(c: &mut Computer) -> Result<String> {
    loop {
        if let Err(e) = c.simulate(false) {
            println!("{:?}", e);
            break;
        }
    }
    Ok(join(c.out.iter().map(|c| c.to_string()), ","))
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut c = parse(lines)?;
    part1_sim(&mut c)
}

fn sim_one_loop_hardcoded(a: usize) -> u8 {
    // bst 4(a) - "take lower bits of a"
    let mut b = a % 8;

    // bxl b, 3
    b = b ^ 3;

    // cdv 5(b)
    let c = a >> b;

    // adv 3 - "shift off lower bits of a"
    //a = a >> 3;

    // bxc _
    b = b ^ c;

    // bxl b, 5
    b = b ^ 5;

    // out 5(b)
    (b % 8) as u8
    // then it jnz's back to start
}

fn part2_sim(orig_computer: &Computer, a: usize) -> bool {
    let mut c2 = orig_computer.clone();
    c2.a = a;
    loop {
        if c2.simulate(true).is_err() {
            break;
        }
    }
    return c2.out == orig_computer.mem;
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let c = parse(lines)?;
    let mut desired = c.mem.clone();
    desired.reverse();
    let mut candidates: Vec<usize> = vec![0];
    for d in desired {
        let mut found = vec![];
        for candidate in candidates {
            for lower_bits in 0..8 {
                let a = (candidate << 3) + lower_bits;
                if sim_one_loop_hardcoded(a) == d {
                    found.push(a);
                }
            }
        }
        candidates = found;
    }
    candidates.sort();
    let lowest = candidates.first().unwrap();

    if !part2_sim(&c, *lowest) {
        bail!("wtf")
    }

    Ok(lowest.to_string())
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
    fn test_part1_a() -> Result<()> {
        let mut c = Computer {
            a: 0,
            b: 0,
            c: 9,
            ip: 0,
            mem: vec![2, 6],
            out: Vec::new(),
        };
        assert_eq!(part1_sim(&mut c)?, "");
        assert_eq!(c.b, 1);
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let mut c = Computer {
            a: 10,
            b: 0,
            c: 0,
            ip: 0,
            mem: vec![5, 0, 5, 1, 5, 4],
            out: Vec::new(),
        };
        assert_eq!(part1_sim(&mut c)?, "0,1,2");
        Ok(())
    }

    #[test]
    fn test_part1_c() -> Result<()> {
        let mut c = Computer {
            a: 2024,
            b: 0,
            c: 0,
            ip: 0,
            mem: vec![0, 1, 5, 4, 3, 0],
            out: Vec::new(),
        };
        assert_eq!(part1_sim(&mut c)?, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(c.a, 0);
        Ok(())
    }

    #[test]
    fn test_part1_d() -> Result<()> {
        let mut c = Computer {
            a: 0,
            b: 29,
            c: 0,
            ip: 0,
            mem: vec![1, 7],
            out: Vec::new(),
        };
        assert_eq!(part1_sim(&mut c)?, "");
        assert_eq!(c.b, 26);
        Ok(())
    }

    #[test]
    fn test_part1_e() -> Result<()> {
        let mut c = Computer {
            a: 0,
            b: 2024,
            c: 43690,
            ip: 0,
            mem: vec![4, 0],
            out: Vec::new(),
        };
        assert_eq!(part1_sim(&mut c)?, "");
        assert_eq!(c.b, 44354);
        Ok(())
    }

    #[test]
    fn test_part1_big() -> Result<()> {
        let lines = lines(indoc! {"
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
        "});
        assert_eq!(part1(lines)?, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    // Didn't write a generic solution, so this example doesn't work:
    // But this one is super-easy to reverse engineer:
    //  shift A right 3 bits
    //  output lower 3 bits of A
    //  loop if A != 0
    // #[test]
    // fn test_part2() -> Result<()> {
    //     let lines = lines(indoc! {"
    //         Register A: 2024
    //         Register B: 0
    //         Register C: 0

    //         Program: 0,3,5,4,3,0
    //     "});
    //     assert_eq!(part2(lines)?, "117440");
    //     Ok(())
    // }

    #[test]
    fn test_part2_confirm() -> Result<()> {
        let a1 = 236581108670061;
        let c1 = Computer {
            a: a1,
            b: 0,
            c: 0,
            ip: 0,
            mem: vec![2, 4, 1, 3, 7, 5, 0, 3, 4, 1, 1, 5, 5, 5, 3, 0],
            out: Vec::new(),
        };
        let o1 = part2_sim(&c1, a1);
        assert_eq!(o1, true);
        Ok(())
    }
}

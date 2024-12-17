use itertools::join;

use anyhow::{anyhow, bail, Result};

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    mem: Vec<u8>,
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

    fn simulate(&mut self, output: &mut Vec<u8>) -> Result<()> {
        if self.ip >= self.mem.len() {
            bail!("halt");
        }

        let opcode = self.read()?;
        match opcode {
            0 => {
                // adv (really "right shift")
                self.a = self.shr()?;
            }
            1 => {
                // bxl
                let x = self.b;
                let y = self.read()?;
                self.b = x ^ y;
            }
            2 => {
                // bst
                self.b = self.combo()? % 8;
            }
            3 => {
                // jnz
                if self.a != 0 {
                    let ip = self.read()?;
                    self.ip = ip;
                }
            }
            4 => {
                // bxc
                self.b = self.b ^ self.c;
                _ = self.read()?; // "legacy reasons"
            }
            5 => {
                // out
                let v = self.combo()?;
                output.push((v % 8).try_into()?);
            }
            6 => {
                // bdv
                self.b = self.shr()?;
            }
            7 => {
                // cdv
                self.c = self.shr()?;
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
    })
}

fn part1_sim(c: &mut Computer) -> Result<String> {
    let mut output = Vec::new();
    loop {
        if let Err(e) = c.simulate(&mut output) {
            println!("{:?}", e);
            break;
        }
    }
    Ok(join(output.into_iter().map(|c| c.to_string()), ","))
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut c = parse(lines)?;
    part1_sim(&mut c)
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    bail!("not done")
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
}

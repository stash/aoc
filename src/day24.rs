use std::cell::RefCell;
use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use regex::Regex;

enum Operand {
    And,
    Or,
    Xor,
}

struct Gate<'a> {
    lhs: &'a str,
    op: Operand,
    rhs: &'a str,
}

struct Chal<'a> {
    values: RefCell<HashMap<&'a str, usize>>,
    gates: HashMap<&'a str, Gate<'a>>,
}
impl<'a> Chal<'a> {
    fn parse(lines: &'a Vec<String>) -> Result<Self> {
        let mut values: HashMap<&str, usize> = HashMap::new();
        let mut iter = lines.iter();
        while let Some(line) = iter.next() {
            if line == "" {
                break;
            }
            let (name, value) = line.split_once(": ").unwrap();
            values.insert(name, value.parse()?);
        }

        let mut gates: HashMap<&str, Gate> = HashMap::new();
        let gate_re: Regex =
            Regex::new(r"^([a-z0-9]+) (AND|X?OR) ([a-z0-9]+) -> ([a-z0-9]+)$").unwrap();
        while let Some(line) = iter.next() {
            let cap = gate_re
                .captures(line)
                .ok_or_else(|| anyhow!("didn't match regex"))?;
            let op = match &cap[2] {
                "AND" => Operand::And,
                "OR" => Operand::Or,
                "XOR" => Operand::Xor,
                e => bail!("invalid operand {}", e),
            };
            let gate = Gate {
                lhs: &cap.get(1).unwrap().as_str(),
                op,
                rhs: &cap.get(3).unwrap().as_str(),
            };
            gates.insert(&cap.get(4).unwrap().as_str(), gate);
        }
        Ok(Self {
            values: RefCell::new(values),
            gates,
        })
    }

    fn resolve(&self, name: &'a str) -> usize {
        if let Some(value) = self.values.borrow().get(name) {
            return *value;
        }
        let g = self.gates.get(name).unwrap();
        let lhs = self.resolve(g.lhs);
        let rhs = self.resolve(g.rhs);
        let value = match g.op {
            Operand::And => lhs & rhs,
            Operand::Or => lhs | rhs,
            Operand::Xor => lhs ^ rhs,
        };
        self.values.borrow_mut().insert(name, value);
        value
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let c = Chal::parse(&lines)?;
    let mut total = 0;
    for name in c.gates.keys() {
        if !name.starts_with('z') {
            continue;
        }
        let z = &name[1..].parse::<i32>()?;
        let value = c.resolve(name);
        total |= value << z;
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

    fn to_lines() -> Vec<String> {
        let text = indoc! {"
            x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1

            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj
        "};
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(to_lines())?, "2024");
        Ok(())
    }
}

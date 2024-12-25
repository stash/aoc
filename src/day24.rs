use std::collections::HashMap;
use std::fmt::Display;
use std::{cell::RefCell, collections::BTreeSet};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use regex::Regex;

// OK look...
// This is gross.
// This is ugly.
// But writing the code and gradually getting to the point where each circuit is
// labelled allowed me to filter out the outputs that looked "fine" (as in
// `q[n]^c[n-1]`)
//
// To make this "for real" probably just take the labelling approach all the way
// to its extreme. Eliminate all z-outputs that have "good" `q[n]^c[n-1]` shape,
// then brute-force the remainder, including possible x[n] <-> y[n] swaps (that
// was the final one for me.)
//
// Full adder has 5 gates and looks like:
//     q[n] = x[n] ^ y[n]    -- intermediary "q", partial sum
//     r[n] = x[n] & y[n]    -- intermediary "r", partial carry
//     p[n] = q[n] & c[n-1]  -- intermediary "p", partial carry
//     c[n] = p[n] | r[n]    -- c = full carry is "p OR r"
//     z[n] = q[n] ^ c[n-1]  -- z = full sum (carry + x + y)
//
// Base-case is:
//     r00 = x00 & y00
//     q00 = x00 ^ y00
//     p00 = q00 & 0 = 0
//     c00 = r00 | 0 = r00
//     z00 = q00 ^ 0 = q00
//
// Terminal case is (for 44-bit adder):
//     z45 = c44
//
// Good luck.

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operand {
    And,
    Or,
    Xor,
}
impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
        }
    }
}

#[derive(Clone)]
struct Gate<'a> {
    lhs: &'a str,
    op: Operand,
    rhs: &'a str,
    collapse: Option<String>,
}
impl Display for Gate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.collapse {
            write!(f, "{}", label)
        } else {
            write!(f, "({}{}{})", self.lhs, self.op, self.rhs)
        }
    }
}

struct Chal<'a> {
    values: RefCell<HashMap<&'a str, usize>>,
    gates: HashMap<&'a str, Gate<'a>>,
    z_names: Vec<&'a str>,
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
        let mut z_names = Vec::new();
        let gate_re: Regex =
            Regex::new(r"^([a-z0-9]+) (AND|X?OR) ([a-z0-9]+) -> ([a-z0-9]+)$").unwrap();
        while let Some(line) = iter.next() {
            let cap = gate_re
                .captures(line)
                .ok_or_else(|| anyhow!("didn't match regex"))?;

            let a = cap.get(1).unwrap().as_str();
            let op = match &cap[2] {
                "AND" => Operand::And,
                "OR" => Operand::Or,
                "XOR" => Operand::Xor,
                e => bail!("invalid operand {}", e),
            };
            let b = cap.get(3).unwrap().as_str();

            let (lhs, rhs) = if a < b { (a, b) } else { (b, a) };
            let gate = Gate {
                lhs,
                op,
                rhs,
                collapse: None,
            };
            let gate_name = cap.get(4).unwrap().as_str();
            if gate_name.starts_with('z') {
                z_names.push(gate_name);
            }
            gates.insert(gate_name, gate);
        }
        z_names.sort();
        Ok(Self {
            values: RefCell::new(values),
            gates,
            z_names,
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

    fn trace(&self, name: &'a str) -> String {
        let g = self.gates.get(name).unwrap();
        if let Some(label) = &g.collapse {
            return label.clone();
        }

        let lhs = if g.lhs.starts_with('x') || g.lhs.starts_with('y') {
            g.lhs.to_owned()
        } else {
            self.trace(g.lhs)
        };
        let rhs = if g.rhs.starts_with('x') || g.rhs.starts_with('y') {
            g.rhs.to_owned()
        } else {
            self.trace(g.rhs)
        };
        let infix = match g.op {
            Operand::And => "&",
            Operand::Or => "|",
            Operand::Xor => "^",
        };
        let result: String = format!("{name}({lhs}{infix}{rhs})");
        result
    }

    fn resolve_all(&self) -> usize {
        let mut total: usize = 0;
        for name in self.z_names.iter() {
            let z = &name[1..].parse::<i32>().expect("valid number suffix");
            let value = self.resolve(name);
            total |= value << z;
        }
        total
    }

    fn add_up_prefix(&self, prefix: char) -> usize {
        let mut total = 0;
        for (name, value) in self.values.borrow().iter() {
            if !name.starts_with(prefix) {
                continue;
            }
            let shift = &name[1..].parse::<i32>().expect("valid number");
            total |= value << shift;
        }
        total
    }

    fn set_label(&mut self, name: &'a str, label: String) {
        self.gates.get_mut(name).unwrap().collapse = Some(label);
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let c = Chal::parse(&lines)?;
    let total = c.resolve_all();
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut c = Chal::parse(&lines)?;
    let mut problems: BTreeSet<&str> = BTreeSet::new();

    let x = c.add_up_prefix('x');
    let y = c.add_up_prefix('y');
    let expect_z = x + y;
    println!("x : {x:045b} + \ny : {y:045b} = \nz: {expect_z:045b}");

    let mut p_n = vec!["___"; 45];
    let mut q_n = vec!["___"; 45];
    let mut r_n = vec!["___"; 45];
    let mut c_n = vec!["___"; 45];
    {
        // Check z00 is correct for adding
        let z00 = c.gates.get("z00").unwrap();
        assert!(z00.op == Operand::Xor);
        assert!(z00.lhs == "x00");
        assert!(z00.rhs == "y00");
        q_n[0] = "z00";
    }

    {
        // Locate c00 and r00. Gate p00 is never set
        let mut r00: Option<&str> = None;
        for (name, gate) in &c.gates {
            match (gate.op, gate.lhs, gate.rhs) {
                (Operand::And, "x00", "y00") => {
                    if r00.is_some() {
                        bail!("duplicate gate for r00")
                    }
                    r00 = Some(name)
                }
                _ => {}
            }
        }
        if let Some(name) = r00 {
            println!("r00/c00 is {name}");
            c_n[0] = name;
            r_n[0] = name;
            c.set_label(name, "r00/c00".to_owned());
            // p[0] is never used and not set
        } else {
            bail!("r00 not found")
        }
    }

    {
        // confirm z01 is correct for adding
        let z01 = c.gates.get("z01").unwrap();
        assert!(z01.op == Operand::Xor);
        let lhs = c.gates.get(z01.lhs).unwrap();
        let rhs = c.gates.get(z01.rhs).unwrap();
        let (c_name, _c, q_name, q) = match (&lhs.op, &rhs.op) {
            (Operand::And, Operand::Xor) => (z01.lhs, lhs, z01.rhs, rhs),
            (Operand::Xor, Operand::And) => (z01.rhs, rhs, z01.lhs, lhs),
            _ => bail!("z01 invalid for adder"),
        };
        println!("z01 = c00 {} ^ q01 {}", c_name, q_name);
        assert_eq!(c_name, r_n[0]);
        assert_eq!(c_name, c_n[0]);
        assert!(q.lhs == "x01");
        assert!(q.rhs == "y01");
        q_n[1] = q_name;
        c.set_label(c_name, "c00".to_owned());
        c.set_label(q_name, "q01".to_owned());
    }

    // confirm rest of base layer is correct
    for (name, gate) in &c.gates {
        // gate names get sorted, so x will always be lhs
        if gate.lhs.starts_with('x') {
            if gate.rhs.starts_with('x') {
                bail!("base layer x.x conflict: {}: {}", name, c.trace(name))
            } else if gate.rhs.starts_with('y') {
                let x = &gate.lhs[1..].parse::<usize>().expect("valid number suffix");
                let y = &gate.rhs[1..].parse::<usize>().expect("valid number suffix");
                if x != y {
                    bail!("base layer conflict: {}: {}", name, c.trace(name))
                }
                match gate.op {
                    Operand::Or => bail!("base layer OR: {}: {}", name, c.trace(name)),
                    Operand::Xor => q_n[*x] = name,
                    Operand::And => r_n[*x] = name,
                }
                if gate.op == Operand::Or {}
            }
        } else if gate.lhs.starts_with('y') {
            bail!("base layer y on lhs: {}: {}", name, c.trace(name))
        }
    }

    println!("q base layer: {}", q_n.iter().join(","));
    for (n, name) in q_n.iter().enumerate().skip(1) {
        // skip z00, which is always in sum_n[0]
        if name.starts_with('z') {
            println!(
                "base layer q-signal should not be output layer: {} needs swap",
                name
            );
            problems.insert(name);
        } else {
            c.set_label(name, format!("q{:02}", n));
        }
    }
    println!("r base layer: {}", r_n.iter().join(","));
    for (n, name) in r_n.iter().enumerate() {
        if name.starts_with('z') {
            println!(
                "base layer r-signal should not be output layer: {} needs swap",
                name
            );
            problems.insert(name);
        } else {
            c.set_label(name, format!("r{:02}", n));
        }
    }

    // AND gates are _either_ in the r-signal set (i.e., x[n]&y[n]) _or_ p-signal set (i.e., p[n] = q[n]&c[n-1])
    // Find AND gates that aren't in r:
    for (name, gate) in c
        .gates
        .iter()
        .filter(|(_name, gate)| gate.op == Operand::And)
        .filter(|(name, _gate)| !r_n.contains(*name))
    {
        let lhs = c.gates.get(gate.lhs).unwrap();
        let rhs = c.gates.get(gate.rhs).unwrap();
        let n_lhs = q_n.iter().position(|name| *name == gate.lhs);
        let n_rhs = q_n.iter().position(|name| *name == gate.rhs);
        if let Some(n) = n_lhs {
            // rhs should look like c[n-1]
            if rhs.op == Operand::Or {
                // fine, maybe add to p[n]?
            } else if gate.rhs == c_n[0] {
                // base-case, likely this is c[1]
                assert_eq!(q_n[1], gate.lhs);
            } else {
                println!(
                    "rhs of AND doesn't look like c (should be OR): n={} {}{} -> {}{} & {}{}",
                    n, name, gate, gate.lhs, lhs, gate.rhs, rhs
                );
                problems.insert(gate.rhs);
            }
        } else if let Some(n) = n_rhs {
            // lhs should look like c[n-1]
            if lhs.op == Operand::Or {
                // fine, maybe add to p[n]?
            } else if gate.lhs == c_n[0] {
                // base-case, likely this is c[1]
                assert_eq!(q_n[1], gate.rhs);
            } else {
                println!(
                    "lhs of AND doesn't look like c (should be OR): n={} {}{} -> {}{} & {}{}",
                    n, name, gate, gate.lhs, lhs, gate.rhs, rhs
                );
                problems.insert(gate.lhs);
            }
        }
    }

    // OR gates are used exclusively for the carry-out (c_n) bit
    let mut label_after: Vec<(&str, String)> = vec![];
    {
        for (c_name, c_gate) in c
            .gates
            .iter()
            .filter(|(_name, gate)| gate.op == Operand::Or)
        {
            // c[n] = p[n] | r[n]  -- we don't know p[n] yet
            let maybe_r_lhs = r_n.iter().position(|name| *name == c_gate.lhs);
            let maybe_r_rhs = r_n.iter().position(|name| *name == c_gate.rhs);
            match (maybe_r_lhs, maybe_r_rhs) {
                (Some(_n_l), Some(_n_r)) => {
                    // TODO: something? Doesn't happen for my input
                    bail!("do something with two r-signals in OR")
                }
                (None, None) => {
                    let lhs = c.gates.get(c_gate.lhs).unwrap();
                    let rhs = c.gates.get(c_gate.rhs).unwrap();
                    println!(
                        "degenerate OR gate: neither side is r: {} {} -> {}={} {}={}",
                        c_name, c_gate, c_gate.lhs, lhs, c_gate.rhs, rhs
                    );
                    // TODO: delve into sides
                }
                (None, Some(n)) => {
                    // well-formed, right is r-signal
                    let lhs = c.gates.get(c_gate.lhs).unwrap();
                    c_n[n] = c_name;
                    if lhs.op == Operand::And {
                        p_n[n] = c_gate.lhs; // found p-signal
                        label_after.push((c_name, format!("c{:02}", n)));
                        label_after.push((&c_gate.lhs, format!("p{:02}", n)));
                    } else {
                        println!(
                            "p-side of OR gate is not AND: {} {} n={}",
                            c_gate.lhs, lhs, n
                        );
                        problems.insert(c_gate.lhs);
                    }
                }
                (Some(n), None) => {
                    // well-formed, left is r-signal
                    let rhs = c.gates.get(c_gate.rhs).unwrap();
                    // check rhs for p-signal-ness
                    c_n[n] = c_name;
                    if rhs.op == Operand::And {
                        p_n[n] = c_gate.rhs; // found p-signal
                        label_after.push((c_name, format!("c{:02}", n)));
                        label_after.push((&c_gate.rhs, format!("p{:02}", n)));
                    } else {
                        println!(
                            "p-side of OR gate is not AND: {} {}, n={}",
                            c_gate.rhs, rhs, n
                        );
                        problems.insert(c_gate.rhs);
                    }
                }
            }
        }
    }
    for (name, label) in label_after {
        c.set_label(name, label);
    }

    // check carry-signals are not output
    println!("carry   : {}", c_n.iter().join(","));
    for z_name in c_n.iter().filter(|name| name.starts_with('z')) {
        if *z_name != "z45" {
            println!("z-signal should never be carry: {}", z_name);
            problems.insert(z_name);
        }
    }

    // check p-signals are not output (except p00, which doesn't exist)
    println!("p-signal: {}", p_n.iter().join(","));
    for z_name in p_n.iter().skip(1).filter(|name| name.starts_with('z')) {
        println!("z-signal should never be p-signal: {}", z_name);
        problems.insert(z_name);
    }

    println!("resolutions:");
    for (n, z_name) in c.z_names.iter().enumerate() {
        // z45 should be c44
        let trace = c.trace(z_name);
        if n > 0 {
            // filter-out fully-resolved gates:
            let expect_one = format!("z{:02}(q{:02}^c{:02})", n, n, n - 1);
            let expect_two = format!("z{:02}(c{:02}^q{:02})", n, n - 1, n);
            if trace == expect_one || trace == expect_two {
                continue;
            }
        }
        println!("{} = {}", z_name, trace);
    }
    for (name, gate) in c.gates.iter() {
        if let Some(label) = &gate.collapse {
            // TODO: these are specific to my input and were some swapped q-&r-signals under a carry signal
            if label == "r35" || label == "q35" {
                println!("odd duck: {} is {}", name, gate);
            }
        }
    }
    println!("problems: {}", problems.into_iter().join(","));
    bail!("now go figure it out");
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

    #[test]
    fn test_part2() -> Result<()> {
        for (i, j) in vec![1, 2, 3, 4, 5, 6, 7, 8].iter().tuple_combinations() {
            println!("{i}-{j}")
        }
        Ok(())
    }
}

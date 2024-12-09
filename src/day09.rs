use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone)]
struct FileBlocks {
    positions: Vec<usize>,
}

#[derive(Debug)]
struct Map {
    free: Vec<usize>, // positions
    alloc: Vec<FileBlocks>,
}

fn parse(lines: Vec<String>) -> Result<Map> {
    // Single very-long line, RLE pairs
    let mut free: Vec<usize> = Vec::new();
    let mut alloc: Vec<FileBlocks> = Vec::new();
    let mut position: usize = 0;
    let mut substrs = lines
        .first()
        .ok_or_else(|| anyhow!("missing line??"))?
        .split_terminator("")
        .skip(1); // split_terminator + skip will strip the first/last "" literals
    loop {
        if let Some(file_len_c) = substrs.next() {
            let file_len: usize = file_len_c.to_string().parse()?;
            let positions: Vec<usize> = (position..position + file_len).collect();
            position += file_len;
            alloc.push(FileBlocks { positions });
        } else {
            break;
        }

        if let Some(free_len_c) = substrs.next() {
            let mut free_len: usize = free_len_c.to_string().parse()?;
            while free_len > 0 {
                free.push(position);
                position += 1;
                free_len -= 1;
            }
        } else {
            break;
        }
    }

    Ok(Map { free, alloc })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;
    println!("before: {:?}", map);
    map.alloc.reverse();

    {
        let mut a_iter = map.alloc.iter_mut();
        let mut f_iter = map.free.into_iter();
        'outer: loop {
            if let Some(cur_alloc) = a_iter.next() {
                for i in (0..cur_alloc.positions.len()).rev() {
                    if let Some(free) = f_iter.next() {
                        // In the event there's more free space than allocated, this loop algorithm will try to move blocks _forward_ unless we check that here
                        if cur_alloc.positions[i] > free {
                            cur_alloc.positions[i] = free;
                        } else {
                            break 'outer;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    }
    map.alloc.reverse();
    println!("after: {:?}", map.alloc);
    let mut total: usize = 0;
    for (block_id, b) in map.alloc.into_iter().enumerate() {
        for p in b.positions {
            total += block_id * p;
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

    use super::*;
    fn ez_input() -> Vec<String> {
        "12345".lines().map(|x| x.to_string()).collect()
    }
    fn input() -> Vec<String> {
        "2333133121414131402"
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_part1_ez() -> Result<()> {
        assert_eq!(
            part1(ez_input())?,
            (0 * 0 + 1 * 2 + 2 * 2 + 3 * 1 + 4 * 1 + 5 * 1 + 6 * 2 + 7 * 2 + 8 * 2).to_string()
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(input())?, "1928");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(input())?, "");
        Ok(())
    }
}

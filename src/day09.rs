use std::collections::BTreeSet;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct FileBlocks {
    positions: Vec<usize>,
}

#[derive(Debug)]
struct Map {
    free: BTreeSet<usize>, // positions
    alloc: Vec<FileBlocks>,
}

fn parse(lines: Vec<String>) -> Result<Map> {
    // Single very-long line, RLE pairs
    let mut free: BTreeSet<usize> = BTreeSet::new();
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
                free.insert(position);
                position += 1;
                free_len -= 1;
            }
        } else {
            break;
        }
    }

    Ok(Map { free, alloc })
}

fn checksum(alloc: Vec<FileBlocks>) -> usize {
    let mut total: usize = 0;
    for (block_id, b) in alloc.into_iter().enumerate() {
        for p in b.positions {
            total += block_id * p;
        }
    }
    total
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;
    // println!("before: {:?}", map);
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
    // println!("after: {:?}", map.alloc);
    let total = checksum(map.alloc);
    Ok(total.to_string())
}

fn take_contiguous(free: &mut BTreeSet<usize>, need: usize, limit: usize) -> Option<Vec<usize>> {
    let mut found: Option<usize> = None;
    if free.len() == 0 { return None }
    if need == 1 {
        // fast case for single-width hole
        if let Some(first) = free.pop_first() {
            if first < limit { return Some(vec![first]) }
            else { return None }
        } else { return None }
    }
    {
        let mut iter = free.iter(); // closure for iterator
        let mut prev = *(iter.next().expect("at least one element needed"));
        let mut size: usize = 1;
        while let Some(next) = iter.next() {
            let cur = *next;
            if cur > limit {
                // prevent forward relocations. Past the point of no-return
                return None;
            }
            if cur == (prev + 1) {
                size += 1;
            } else {
                size = 1; // restart
            }
            
            if size == need {
                found = Some(cur - (size-1));
                break;
            }

            prev = cur;
        }
    }
    if let Some(found) = found {
        let taken: Vec<usize> = (found .. (found+need)).collect();
        for take in taken.iter() {
            free.remove(take);
        }
        return Some(taken);
    }
    None
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;
    // println!("before: {:?}", map);

    println!("before free {}", map.free.len());
    for file in map.alloc.iter_mut().rev() {
        // println!("prior: {:?}", file);
        let need = file.positions.len();

        let limit = file.positions.first().ok_or_else(|| anyhow!("zero width file??"))?;
        if let Some(mut relocations) = take_contiguous(&mut map.free, need, *limit) {
            assert_eq!(file.positions.len(), relocations.len());
            std::mem::swap(&mut file.positions, &mut relocations);
            // return original positions to the pool
            for pos in relocations {
                map.free.insert(pos);
            }
        }
        // println!("  after: {:?}", file);
    }

    // println!("after: {:?}", map.alloc);
    println!("after free {}", map.free.len());
    let total = checksum(map.alloc);
    Ok(total.to_string())
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
        assert_eq!(part2(input())?, "2858");
        Ok(())
    }
}

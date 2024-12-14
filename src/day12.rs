use std::rc::Rc;

use anyhow::{anyhow, bail, Result};

use crate::common::*;

struct Plot {
    plant: char,
    region: usize,
    fences: usize, // direction probably doesn't matter
}

struct Map {
    plots: Vec<Vec<Plot>>,
    bounds: Pos,
}
impl Map {
    fn in_bounds(&self, p:Pos) -> bool {
        p.x > 0 && p.x < self.bounds.x && p.y > 0 && p.y < self.bounds.y
    }
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let plots: Vec<Vec<Plot>> = lines.into_iter().map(|row| {
        row.chars().map(|c| {
            Plot { plant:c, region:0, fences:0 }
        }).collect()
    }).collect();
    let bounds = Pos::new(
        plots.first().ok_or_else(||anyhow!("empty map?"))?.len(),
        plots.len(),
    )?;
    
    Ok(Map { plots, bounds })
}


pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;

    // add fences to global perimiter
    for mut plot in map.plots[0] {
        plot.fences += 1;
    }
    for mut plot in map.plots[(map.bounds.y-1) as usize].iter() {
        plot.fences += 1;
    }
    for y in 0..map.bounds.y {
        let mut plot = map.plots[y as usize][0];
        plot.fences += 1;
        plot = map.plots[y as usize][(map.bounds.x-1) as usize];
        plot.fences += 1;
    }

    // scan horizontally, place fence between veg changes
    for y in 0..map.bounds.y {
        let prev_plant = map.plots[y as usize][0].plant;
        for x in 1..map.bounds.x {
            let mut cur_plant = ' ';
            {
                let mut plot: &mut Plot = &mut map.plots[y as usize][x as usize];
                cur_plant = plot.plant;
                if prev_plant != cur_plant {
                    plot.fences += 1;
                }
            }
            {
                let mut plot: &mut Plot = &mut map.plots[y as usize][(x-1) as usize];
                if prev_plant != cur_plant {
                    plot.fences += 1;
                }
            }
        }
    }

    // scan vertically & place fences
    for x in 0..map.bounds.x {
        let prev_plant = map.plots[0][x as usize].plant;
        for y in 1..map.bounds.y {
            let mut cur_plant = ' ';
            {
                let plot: &mut Plot = &mut map.plots[y as usize][x as usize];
                cur_plant = plot.plant;
                if prev_plant != cur_plant {
                    plot.fences += 1;
                }
            }
            {
                let plot: &mut Plot = &mut map.plots[(y-1) as usize][x as usize];
                if prev_plant != cur_plant {
                    plot.fences += 1;
                }
            }
        }   
    }

    // form connected groups
    let mut next_region = 0;
    {
        // force 0,0 to region 1
        next_region += 1;
        let plot: &mut Plot = &mut map.plots[0][0];
        plot.region = next_region;
    }
    for x in 0..map.bounds.x {
        for y in 1..map.bounds.y {
        }
    }

    // tally fences & area for each group (iterate over plots, accumulate by group ID)

    bail!("not done")
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
        text.lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_part1_ez() -> Result<()> {
        let lines = lines(indoc! {"
            RR
            RA
        "});
        assert_eq!(part1(lines)?, (3*8 + 1*1).to_string());
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "});
        assert_eq!(part1(lines)?, "1930");
        Ok(())
    }

}

use anyhow::{anyhow, bail, Result};

#[derive(Clone)]
struct Plot {
    plant: char,
    region: usize,
    fences: usize, // direction probably doesn't matter
}

struct Map {
    plots: Vec<Vec<Plot>>,
    width: usize,
    height: usize,
}
impl Map {
    fn merge_region(&mut self, from: usize, into: usize) {
        for row in self.plots.iter_mut() {
            for plot in row.iter_mut() {
                if plot.region == from {
                    plot.region = into;
                }
            }
        }
    }
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let plots: Vec<Vec<Plot>> = lines
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|c| Plot {
                    plant: c,
                    region: 0,
                    fences: 0,
                })
                .collect()
        })
        .collect();
    let width = plots.first().ok_or_else(|| anyhow!("empty map?"))?.len();
    let height = plots.len();
    Ok(Map {
        plots,
        width,
        height,
    })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;

    // add fences to global perimiter
    for plot in &mut map.plots[0] {
        plot.fences += 1; // north
    }
    for plot in &mut map.plots[map.height - 1].iter_mut() {
        plot.fences += 1; // south
    }
    for y in 0..map.height {
        let mut plot = &mut map.plots[y][0];
        plot.fences += 1; // east
        plot = &mut map.plots[y][map.width - 1];
        plot.fences += 1; // west
    }

    // scan horizontally, place fence between plant changes
    for y in 0..map.height {
        let prev_plant = map.plots[y][0].plant;
        for x in 1..map.width {
            let mut plot: &mut Plot = &mut map.plots[y][x];
            let cur_plant = plot.plant;
            if prev_plant != cur_plant {
                plot.fences += 1;
            }
            plot = &mut map.plots[y][x - 1];
            if prev_plant != cur_plant {
                plot.fences += 1;
            }
        }
    }

    // scan vertically & place fences
    for x in 0..map.width {
        let prev_plant = map.plots[0][x].plant;
        for y in 1..map.height {
            let mut plot: &mut Plot = &mut map.plots[y][x];
            let cur_plant = plot.plant;
            if prev_plant != cur_plant {
                plot.fences += 1;
            }
            plot = &mut map.plots[y - 1][x];
            if prev_plant != cur_plant {
                plot.fences += 1;
            }
        }
    }

    // form connected groups
    let mut next_region = 0;
    {
        // force 0,0 to region 1
        next_region += 1;
        let plot = &mut map.plots[0][0];
        plot.region = next_region;
    }
    for x in 0..map.width {
        for y in 0..map.height {
            if x > 0 {
                // try merge with left
                let left = map.plots[y][x - 1].clone();
                let plot = &mut map.plots[y][x];
                if plot.plant == left.plant {
                    plot.region = left.region;
                } else {
                    next_region += 1;
                    plot.region = next_region;
                }
            }
            if y > 0 {
                // try merge with above. Because of the order, may join regions
            }
        }
    }

    // tally fences & area for each region (iterate over plots, accumulate by group ID)

    bail!("not done")
}

pub fn part2(_lines: Vec<String>) -> Result<String> {
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
    fn test_part1_ez() -> Result<()> {
        let lines = lines(indoc! {"
            RR
            RA
        "});
        assert_eq!(part1(lines)?, (3 * 8 + 1 * 1).to_string());
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

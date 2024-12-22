use anyhow::Result;
use rayon::prelude::*;

const MASK_PRUNE: i32 = 0xFFFFFF;

fn turn(x: i32) -> i32 {
    let x = ((x << 6) ^ x) & MASK_PRUNE;
    let x = ((x >> 5) ^ x) & MASK_PRUNE;
    let x = ((x << 11) ^ x) & MASK_PRUNE;
    x
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut state: Vec<i32> = lines.into_iter().map(|y| y.parse().unwrap()).collect();
    for _n in 0usize..2000 {
        for i in 0usize..state.len() {
            state[i] = turn(state[i])
        }
    }
    println!("states: {:?}", state);
    Ok(state
        .into_iter()
        .map(|x| x as isize)
        .sum::<isize>()
        .to_string())
}

const N: usize = 2000;

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut state: Vec<i32> = lines.into_iter().map(|y| y.parse().unwrap()).collect();
    let k = state.len(); // # of sellers

    let mut bananas: Vec<Vec<i8>> = vec![state.iter().map(|x| (x % 10) as i8).collect()]; // n row, k col
    let mut delta_b: Vec<Vec<i8>> = vec![vec![0; k]]; // n row, k col
    for n in 1usize..=N {
        let mut banana_row: Vec<i8> = vec![0; k];
        let mut delta_b_row: Vec<i8> = vec![0; k];
        let prev_banana_row = &bananas[n - 1];
        for seller in 0..k {
            let x = turn(state[seller]);
            let b = (x % 10) as i8;
            state[seller] = x;
            banana_row[seller] = b;
            delta_b_row[seller] = b - prev_banana_row[seller];
        }
        bananas.push(banana_row);
        delta_b.push(delta_b_row);
    }
    // println!("zero row: {:?}", bananas[0]);
    // println!("first row: {:?}", bananas[1]);
    // println!("last row: {:?}", bananas[N]);
    // println!("delta-b zero row: {:?}", delta_b[0]);
    // println!("delta-b 1st row: {:?}", delta_b[1]);
    // println!("delta-b Nth row: {:?}", delta_b[N]);

    // transpose into rows per seller
    let mut seller_history: Vec<Vec<i8>> = std::iter::repeat_with(|| vec![0i8; N + 1])
        .take(k)
        .collect();
    for n in 1..=N {
        for seller in 0..k {
            seller_history[seller][n] = delta_b[n][seller];
        }
    }
    println!("generated price history");

    let mut best = 0;
    let mut best_seq = vec![];

    for a in -9..=9 {
        println!("scanning {a}");
        // build an index of where `a` occurs for each seller
        let mut a_index: Vec<Vec<usize>> = std::iter::repeat_with(|| vec![]).take(k).collect();
        for seller in 0..k {
            let hist = &seller_history[seller];
            for n in 1..=(N - 3) {
                if hist[n] == a {
                    a_index[seller].push(n);
                }
            }
        }

        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let trial: [i8; 3] = [b, c, d];
                    // fan out for each seller
                    let total: usize = (0..k)
                        .into_par_iter()
                        .map(|seller| {
                            let hist = &seller_history[seller];
                            let index = a_index[seller].iter();
                            for n in index {
                                let start = n + 1; // offset of `b` since `n` is known `a`
                                let end = n + 3;
                                let window = &hist[start..=end];
                                if window == &trial {
                                    // println!(
                                    //     "{:?} match {end} for {seller}: {}",
                                    //     trial, bananas[end][seller]
                                    // );
                                    return bananas[end][seller] as usize;
                                }
                            }
                            0usize
                        })
                        .sum();
                    // total is the # of bananas for this trial sequence
                    if total > best {
                        best = total;
                        best_seq = vec![a, b, c, d];
                    }
                }
            }
        }
    }

    println!("best seq: {:?}", best_seq);
    Ok(best.to_string())
}

#[cfg(test)]
mod test {

    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn to_lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1_first_steps() -> Result<()> {
        let expect = to_lines(indoc! {"
            15887950
            16495136
            527345
            704524
            1553684
            12683156
            11100544
            12249484
            7753432
            5908254
        "})
        .into_iter()
        .map(|y| y.parse::<i32>().unwrap());
        let mut x = 123;
        for y in expect {
            x = turn(x);
            assert_eq!(y, x);
        }
        Ok(())
    }

    // #[test]
    // fn test_period() -> Result<()> {
    //     let mut seen = HashSet::new();
    //     let mut x = 123;
    //     seen.insert(x);
    //     for n in 0usize.. {
    //         x = turn(x);
    //         if !seen.insert(x) {
    //             println!("period {}", n);
    //             break;
    //         }
    //     }
    //     Ok(())
    // }

    #[test]
    fn test_part1() {
        let lines = to_lines(indoc! {"
            1
            10
            100
            2024
        "});
        assert_eq!(part1(lines).unwrap(), "37327623");
    }

    #[test]
    fn test_part2() {
        let lines = to_lines(indoc! {"
            1
            2
            3
            2024
        "});
        assert_eq!(part2(lines).unwrap(), "23");
    }
}

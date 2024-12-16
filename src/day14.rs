use std::io;
use std::time::Duration;

use anyhow::{anyhow, Result};
use crossterm::{
    cursor,
    event::{
        poll, read, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent,
        KeyEventKind,
    },
    execute, queue, style,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use regex::Regex;

use crate::common::Pos;

struct Bot {
    p: Pos,
    v: Pos,
}
impl Bot {
    fn simulate(&mut self, bounds: Pos) {
        let mut p = self.p + self.v;
        if p.x < 0 {
            p.x += bounds.x;
        } else if p.x >= bounds.x {
            p.x -= bounds.x;
        }
        if p.y < 0 {
            p.y += bounds.y;
        } else if p.y >= bounds.y {
            p.y -= bounds.y;
        }
        self.p = p;
    }

    fn un_simulate(&mut self, bounds: Pos) {
        let mut p = self.p - self.v;
        if p.x < 0 {
            p.x += bounds.x;
        } else if p.x >= bounds.x {
            p.x -= bounds.x;
        }
        if p.y < 0 {
            p.y += bounds.y;
        } else if p.y >= bounds.y {
            p.y -= bounds.y;
        }
        self.p = p;
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Bot>> {
    let re = Regex::new(r"^p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut bots = Vec::new();
    for line in lines {
        let cap = re
            .captures(&line)
            .ok_or_else(|| anyhow!("failed to parse line: {}", &line))?;
        let p = Pos {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
        };
        let v = Pos {
            x: cap[3].parse()?,
            y: cap[4].parse()?,
        };
        bots.push(Bot { p, v });
    }
    Ok(bots)
}

pub fn part1(lines: Vec<String>, bounds: Pos) -> Result<String> {
    let mut bots = parse(lines)?;
    let cycles = 100;
    for _ in 0..cycles {
        for b in bots.iter_mut() {
            b.simulate(bounds);
        }
    }
    let x_part = bounds.x / 2;
    let y_part = bounds.y / 2;
    let mut quads: [isize; 4] = [0, 0, 0, 0];
    for b in bots {
        println!("{:?}", b.p);
        if b.p.x < x_part {
            if b.p.y < y_part {
                quads[0] += 1;
            } else if b.p.y > y_part {
                quads[1] += 1;
            }
        } else if b.p.x > x_part {
            if b.p.y < y_part {
                quads[2] += 1;
            } else if b.p.y > y_part {
                quads[3] += 1;
            }
        }
    }
    println!("{:?}", quads);
    let total: isize = quads.into_iter().fold(1, |acc, x| acc * x);
    Ok(total.to_string())
}

fn part2_sim<W>(w: &mut W, bots: &mut Vec<Bot>, bounds: Pos) -> Result<String>
where
    W: io::Write,
{
    let min_dur = Duration::from_millis(50);
    let mut dur = Duration::from_millis(250);
    let mut n: usize = 0;
    let at_a_time = 101; // noticed patterns at 23 + (k*101)
    let mut simulating = false;
    let mut forward = true;
    let mut once = false;

    queue!(w, Clear(ClearType::All),)?;

    'outer: loop {
        if poll(dur)? {
            let event = read()?;
            if let Event::Key(KeyEvent { code, kind, .. }) = event {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Backspace => break 'outer,
                        KeyCode::Down => {
                            dur = dur.checked_sub(Duration::from_millis(50)).unwrap_or(dur);
                            if dur < min_dur {
                                dur = min_dur;
                            }
                        }
                        KeyCode::Up => {
                            dur = dur.checked_add(Duration::from_millis(50)).unwrap_or(dur);
                        }
                        KeyCode::Left => {
                            forward = false;
                            once = true;
                        }
                        KeyCode::Right => {
                            forward = true;
                            once = true;
                        }
                        KeyCode::Char(c) => match c {
                            ' ' => simulating = !simulating,
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }

        if simulating || once {
            let times = if once { 1 } else { at_a_time };
            for _ in 0..times {
                for b in bots.iter_mut() {
                    if forward {
                        b.simulate(bounds);
                    } else {
                        if n >= 1 {
                            b.un_simulate(bounds);
                        }
                    }
                }

                if forward {
                    n += 1;
                } else {
                    if n >= 1 {
                        n -= 1;
                    }
                }
            }

            once = false;
        }

        queue!(w, style::ResetColor, cursor::Hide, cursor::MoveTo(0, 0),)?;

        let mut lines = Vec::new();
        for _ in 0..bounds.y {
            lines.push(vec!['.'; bounds.x as usize]);
        }
        for b in bots.iter() {
            lines[b.p.y as usize][b.p.x as usize] = '█';
        }

        queue!(
            w,
            style::SetBackgroundColor(if simulating {
                style::Color::DarkGreen
            } else {
                style::Color::DarkRed
            }),
            style::SetForegroundColor(style::Color::White),
            style::Print(format!(
                "Iteration: {}{} @ {}ms             ",
                n,
                if forward { "++" } else { "--" },
                dur.as_millis()
            )),
            cursor::MoveToNextLine(1),
            style::ResetColor,
        )?;
        for chars in lines {
            let line: String = chars.into_iter().collect();
            queue!(
                w,
                style::Print(line),
                cursor::MoveToNextLine(1),
                style::ResetColor,
            )?;
        }
        queue!(
            w,
            style::Print(format!("Iteration: {}", n)),
            cursor::MoveToNextLine(1),
        )?;

        w.flush()?;
    }

    Ok("done".to_owned())
}

pub fn part2(lines: Vec<String>, bounds: Pos) -> Result<String> {
    let mut bots = parse(lines)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnableBracketedPaste,
        DisableLineWrap,
        EnterAlternateScreen
    )?;

    let r = part2_sim(&mut stdout, &mut bots, bounds);

    execute!(
        stdout,
        DisableBracketedPaste,
        EnableLineWrap,
        LeaveAlternateScreen
    )?;
    disable_raw_mode()?;

    r
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
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "});
        assert_eq!(part1(lines, Pos::new(11, 7)?)?, "12");
        Ok(())
    }
}

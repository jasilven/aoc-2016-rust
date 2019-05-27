extern crate regex;
mod aoc;
use aoc::parse_ints;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
enum Rule {
    BotBot((usize, usize)),
    OutOut((usize, usize)),
    BotOut((usize, usize)),
    OutBot((usize, usize)),
    None,
}

#[derive(Debug, Clone)]
struct Bot {
    rule: Rule,
    chips: Vec<usize>,
}

impl Bot {
    pub fn new(rule: Rule, chips: Vec<usize>) -> Bot {
        Bot {
            rule: rule,
            chips: chips,
        }
    }
    pub fn max_chip(&self) -> Option<&usize> {
        self.chips.iter().max()
    }
    pub fn min_chip(&self) -> Option<&usize> {
        self.chips.iter().min()
    }
}

fn assign_val(b: usize, val: usize, bots: &mut HashMap<usize, Bot>) {
    if bots.contains_key(&b) {
        bots.get_mut(&b).unwrap().chips.push(val);
    } else {
        bots.insert(b, Bot::new(Rule::None, vec![val]));
    }
}

fn assign_rule(b: usize, rule: Rule, bots: &mut HashMap<usize, Bot>) {
    if bots.contains_key(&b) {
        bots.get_mut(&b).unwrap().rule = rule;
    } else {
        bots.insert(b, Bot::new(rule, vec![]));
    }
}

fn parse_input(fname: &str) -> HashMap<usize, Bot> {
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let bb = Regex::new(r"low to bot \d+ and high to bot \d+").unwrap();
    let oo = Regex::new(r"low to output \d+ and high to output \d+").unwrap();
    let bo = Regex::new(r"low to bot \d+ and high to output \d+").unwrap();
    let ob = Regex::new(r"low to output \d+ and high to bot \d+").unwrap();
    let val = Regex::new(r"^value").unwrap();

    for line in BufReader::new(File::open(fname).unwrap()).lines() {
        let line = line.unwrap();
        let ints = parse_ints(&line);
        match true {
            true if val.is_match(&line) => {
                assign_val(ints[1] as usize, ints[0] as usize, &mut bots)
            }
            true if bb.is_match(&line) => assign_rule(
                ints[0] as usize,
                Rule::BotBot((ints[1] as usize, ints[2] as usize)),
                &mut bots,
            ),
            true if oo.is_match(&line) => assign_rule(
                ints[0] as usize,
                Rule::OutOut((ints[1] as usize, ints[2] as usize)),
                &mut bots,
            ),
            true if bo.is_match(&line) => assign_rule(
                ints[0] as usize,
                Rule::BotOut((ints[1] as usize, ints[2] as usize)),
                &mut bots,
            ),
            true if ob.is_match(&line) => assign_rule(
                ints[0] as usize,
                Rule::OutBot((ints[1] as usize, ints[2] as usize)),
                &mut bots,
            ),
            _ => panic!("rule parse error "),
        }
    }
    bots
}

fn bot_bot(bots: &HashMap<usize, Bot>, b: usize, (l, h): (usize, usize)) -> HashMap<usize, Bot> {
    let mut bots = bots.clone();
    let mut bot = bots.get(&b).unwrap().clone();
    let mut bot_hi = bots.get(&h).unwrap().clone();
    let mut bot_low = bots.get(&l).unwrap().clone();
    bot_hi.chips.push(*bot.max_chip().unwrap());
    bot_low.chips.push(*bot.min_chip().unwrap());
    bots.insert(l, bot_low);
    bots.insert(h, bot_hi);
    bot.chips.clear();
    bots.insert(b, bot);
    bots
}

fn bot_out(
    bots: &HashMap<usize, Bot>,
    outs: &HashMap<usize, usize>,
    b: usize,
    (l, h): (usize, usize),
) -> (HashMap<usize, Bot>, HashMap<usize, usize>) {
    let mut bots = bots.clone();
    let mut outs = outs.clone();
    let mut bot = bots.get(&b).unwrap().clone();
    let mut bot_low = bots.get(&l).unwrap().clone();
    outs.insert(h, *bot.max_chip().unwrap());
    bot_low.chips.push(*bot.min_chip().unwrap());
    bot.chips.clear();
    bots.insert(l, bot_low);
    bots.insert(b, bot);
    (bots, outs)
}

fn out_bot(
    bots: &HashMap<usize, Bot>,
    outs: &HashMap<usize, usize>,
    b: usize,
    (l, h): (usize, usize),
) -> (HashMap<usize, Bot>, HashMap<usize, usize>) {
    let mut bots = bots.clone();
    let mut outs = outs.clone();
    let mut bot = bots.get(&b).unwrap().clone();
    let mut bot_hi = bots.get(&h).unwrap().clone();
    outs.insert(l, *bot.min_chip().unwrap());
    bot_hi.chips.push(*bot.max_chip().unwrap());
    bots.insert(h, bot_hi);
    bot.chips.clear();
    bots.insert(b, bot);
    (bots, outs)
}

fn out_out(
    bots: &HashMap<usize, Bot>,
    outs: &HashMap<usize, usize>,
    b: usize,
    (l, h): (usize, usize),
) -> (HashMap<usize, Bot>, HashMap<usize, usize>) {
    let mut bots = bots.clone();
    let mut outs = outs.clone();
    let mut bot = bots.get_mut(&b).unwrap().clone();
    outs.insert(l, *bot.min_chip().unwrap());
    outs.insert(h, *bot.max_chip().unwrap());
    bot.chips.clear();
    bots.insert(b, bot);
    (bots, outs)
}

fn solve1(bots: &HashMap<usize, Bot>) -> Option<usize> {
    match bots
        .iter()
        .filter(|(_, v)| {
            (v.chips.iter().max().unwrap_or(&0) == &61usize)
                & (v.chips.iter().min().unwrap_or(&0) == &17usize)
        })
        .nth(0)
    {
        Some(kv) => Some(*kv.0),
        _ => None,
    }
}

fn main() {
    let fname = "resources/day10-input.txt";
    let mut bots = parse_input(fname);
    let mut outs: HashMap<usize, usize> = HashMap::new();
    let mut part1 = None;

    loop {
        if part1 == None {
            part1 = solve1(&bots);
        }
        match bots.iter().filter(|(_, v)| v.chips.len() == 2).nth(0) {
            Some(kv) => match kv.1.rule {
                Rule::BotBot(lh) => bots = bot_bot(&bots, *kv.0, lh),
                Rule::OutBot(lh) => {
                    let (bs, os) = out_bot(&bots, &outs, *kv.0, lh);
                    bots = bs;
                    outs = os;
                }
                Rule::OutOut(lh) => {
                    let (bs, os) = out_out(&bots, &outs, *kv.0, lh);
                    bots = bs;
                    outs = os;
                }
                Rule::BotOut(lh) => {
                    let (bs, os) = bot_out(&bots, &outs, *kv.0, lh);
                    bots = bs;
                    outs = os;
                }
                Rule::None => {
                    panic!("No rule!!");
                }
            },
            None => break,
        }
    }
    println!("Part 1: {}", part1.unwrap());
    // correct answer: 141
    println!(
        "Part 2: {}",
        outs.get(&0).unwrap() * outs.get(&1).unwrap() * outs.get(&2).unwrap()
    );
    // correct answer: 1209
}

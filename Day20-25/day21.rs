use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Config = Vec<Vec<u8>>;

struct RuleMatcher {
    rules: HashMap<String, String>,
}

impl RuleMatcher {
    fn find_match(&mut self, cfg: String) -> String {
        if self.rules.contains_key(&cfg) {
            return self.rules.get(&cfg).unwrap().clone();
        }

        let mut chunk = cfg_to_chunk(cfg.clone());
        for _ in 0..4 {
            chunk = flip_chunk(&chunk);
            let flipped_cfg = chunk_to_cfg_string(&chunk);

            if self.rules.contains_key(&flipped_cfg) {
                let matched = self.rules.get(&flipped_cfg).unwrap().clone();
                self.add_rule(cfg.clone(), matched);
                return self.rules.get(&flipped_cfg).unwrap().clone();
            }

            chunk = transpose_chunk(&chunk);
            let transposed = chunk_to_cfg_string(&chunk);
            if self.rules.contains_key(&transposed) {
                let matched = self.rules.get(&transposed).unwrap().clone();
                self.add_rule(cfg.clone(), matched);
                return self.rules.get(&transposed).unwrap().clone();
            }
        }

        panic!("No match found")
    }

    fn new() -> RuleMatcher {
        RuleMatcher {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, old_cfg: String, new_cfg: String) {
        self.rules.insert(old_cfg, new_cfg);
    }
}

fn num_active(config: &Config) -> usize {
    let mut active = 0;
    for i in 0..config.len() {
        for j in 0..config.len() {
            if config[i][j] == '#' as u8 {
                active += 1;
            }
        }
    }
    active
}

fn flip_chunk(chunk: &Config) -> Config {
    let mut chunk = chunk.clone();
    let chunk_size = chunk.len();
    for i in 0..chunk.len() / 2 {
        chunk.swap(i, chunk_size - i - 1);
    }
    chunk
}

fn transpose_chunk(chunk: &Config) -> Config {
    let mut transposed = Vec::with_capacity(chunk.len());
    for i in 0..chunk.len() {
        let mut row = Vec::new();
        for j in 0..chunk.len() {
            row.push(chunk[j][i]);
        }
        transposed.push(row);
    }
    transposed
}

fn cfg_to_chunk(cfg: String) -> Config {
    cfg.split("/")
        .map(|x| x.chars().map(|y| y as u8).collect::<Vec<u8>>())
        .collect::<Config>()
}

fn chunk_to_cfg_string(chunk: &Config) -> String {
    let mut cfg_chars: Vec<char> = Vec::new();
    let chunk_len = chunk.len();
    for x in 0..chunk_len {
        for y in 0..chunk.len() {
            cfg_chars.push(chunk[x][y] as char);
        }
        if x + 1 != chunk_len {
            cfg_chars.push('/');
        }
    }
    cfg_chars.iter().collect()
}

fn extract_chunk_cfg_str(config: &Config, cx: usize, cy: usize, chunk_size: usize) -> String {
    let mut cfg_chars = Vec::new();
    for x in cx..cx + chunk_size {
        for y in cy..cy + chunk_size {
            cfg_chars.push(config[x][y] as char);
        }
        if x + 1 != cx + chunk_size {
            cfg_chars.push('/')
        }
    }
    cfg_chars.iter().collect()
}

fn iterate_production(config: &Config, matcher: &mut RuleMatcher) -> Config {
    let chunk_size = if config.len() % 2 == 0 { 2 } else { 3 };
    let new_size = (chunk_size + 1) * config.len() / chunk_size;

    let mut tmp_cfg = Vec::new();
    for _ in 0..new_size {
        tmp_cfg.push(vec![0; new_size]);
    }

    let mut x = 0;
    while x < config.len() {
        let cx = (x / chunk_size) * (chunk_size + 1);
        let mut y = 0;

        while y < config.len() {
            let cy = (y / chunk_size) * (chunk_size + 1);
            let chunk = extract_chunk_cfg_str(config, x, y, chunk_size);
            let chunk_match = matcher.find_match(chunk);
            let new_chunk = cfg_to_chunk(chunk_match);

            for nx in 0..chunk_size + 1 {
                for ny in 0..chunk_size + 1 {
                    tmp_cfg[nx + cx][ny + cy] = new_chunk[nx][ny];
                }
            }

            y += chunk_size;
        }
        x += chunk_size;
    }

    tmp_cfg
}

pub fn main() {
    let mut f = File::open("21.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut matcher = RuleMatcher::new();
    for line in contents.split("\n").map(String::from) {
        let mut rule = line.split_whitespace();
        let old = String::from(rule.next().unwrap());
        rule.next();
        let new = String::from(rule.next().unwrap());
        matcher.add_rule(old, new);
    }

    let mut config = cfg_to_chunk(String::from(".#./..#/###"));
    for i in 1..19 {
        config = iterate_production(&config, &mut matcher);
        if i == 5 {
            println!("Part 1: {}", num_active(&config));
        }
    }
    println!("Part 2: {}", num_active(&config));
}

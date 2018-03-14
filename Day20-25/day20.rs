use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq)]
struct Particle {
    x: i64,
    y: i64,
    z: i64,

    vx: i64,
    vy: i64,
    vz: i64,

    ax: i64,
    ay: i64,
    az: i64,
}

impl Particle {
    fn make_move(&mut self) {
        self.vx += self.ax;
        self.vy += self.ay;
        self.vz += self.az;

        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn distance_at_time(&self, time: f64) -> f64 {
        let t_squared = time * time;
        let x = time * (self.vx as f64) + (0.5 * t_squared * (self.ax as f64));
        let y = time * (self.vy as f64) + (0.5 * t_squared * (self.ay as f64));
        let z = time * (self.vz as f64) + (0.5 * t_squared * (self.az as f64));
        (x*x) + (y*y) + (z*z)
    }

    fn from_string(input: String) -> Particle {
        let fields = input
            .split(&['p', 'v', 'a', '=', '<', '>', ',', ' ', '\t'][..])
            .filter(|x| x.len() > 0)
            .map(|x| String::from(x).clone())
            .collect::<Vec<String>>();
        
        let coords = fields
            .iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
    
        Particle{
            x: coords[0],
            y: coords[1],
            z: coords[2],
            vx: coords[3],
            vy: coords[4],
            vz: coords[5],
            ax: coords[6],
            ay: coords[7],
            az: coords[8],
        }
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z)
    }
}

fn closest_particle(particles: Vec<Particle>) -> usize {
    let time: f64 = 1000.0;

    particles
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.distance_at_time(time)))
        .min_by(|&(_, dist1), &(_, dist2)| dist1.partial_cmp(&dist2).unwrap())
        .map(|(i, _)| i)
        .unwrap()
}

fn num_uncollided_particles(mut particles: Vec<Particle>) -> usize {
    let rounds = 40;
    let mut annihilated: HashSet<Particle> = HashSet::new();

    for _ in 0..rounds {
        let mut locations: HashMap<(i64, i64, i64), Particle> = HashMap::new();

        for i in 0..particles.len() {
            if annihilated.contains(&particles[i]) {
                continue;
            }
            particles[i].make_move();

            let p_loc = (particles[i].x, particles[i].y, particles[i].z);
            if locations.contains_key(&p_loc) {
                annihilated.insert(locations[&p_loc].clone());
                annihilated.insert(particles[i].clone());
            } else {
                locations.insert(p_loc, particles[i].clone());
            }
        }

    }

    particles
        .iter()
        .filter(|p| !annihilated.contains(p))
        .count()
}

pub fn main() {
    let mut f = File::open("20.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let particles = contents
        .split("\n")
        .map(|x| Particle::from_string(String::from(x)))
        .collect::<Vec<Particle>>();
    
    println!("Part 1: {}", closest_particle(particles.clone()));
    println!("Part 2: {}", num_uncollided_particles(particles));
}
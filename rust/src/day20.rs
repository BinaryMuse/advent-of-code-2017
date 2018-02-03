use common;
use std::ops::Add;
use std::collections::HashMap;
use regex::Regex;

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/20.txt").expect("expected input 20.txt");
    {
        let mut particles = input.trim().lines().map(|line| {
            Particle::parse(line)
        }).collect::<Vec<_>>();
        for _ in 1..1000 {
            for particle in particles.iter_mut() {
                particle.update();
            }
        }
        let closest = particles.iter().enumerate().min_by_key(|&(_i, p)| p.distance_from_center()).unwrap();
        println!("Part 1: Particle {:?}", closest.0);
    }

    {
        let particles = input.trim().lines().map(|line| {
            Particle::parse(line)
        }).collect::<Vec<_>>();
        let mut collider = Collider::new(particles);
        for _ in 1..1000 {
            collider.tick();
        }
        println!("Part 2: {} particles left", collider.count());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec3(i64, i64, i64);

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[test]
fn test_vec3() {
    let v1 = Vec3(1, 2, 3);
    let v2 = Vec3(5, 6, 7);
    assert_eq!(v1 + v2, Vec3(6, 8, 10));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Particle(Vec3, Vec3, Vec3);

impl Particle {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref PARTICLE_REGEX: Regex =
                Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
        }

        let captures = PARTICLE_REGEX.captures(s).expect("Regex didn't match line");
        let mut nums = vec![];
        for i in 1..10 {
            let num: i64 = captures[i].parse::<i64>().unwrap();
            nums.push(num);
        }
        let position = Vec3(nums[0], nums[1], nums[2]);
        let velocity = Vec3(nums[3], nums[4], nums[5]);
        let acceleration = Vec3(nums[6], nums[7], nums[8]);
        Particle(position, velocity, acceleration)
    }

    fn update(&mut self) {
        // Increase velocity by acceleration
        self.1 = self.1 + self.2;
        // Increase position by velocity
        self.0 = self.0 + self.1;
    }

    fn distance_from_center(&self) -> i64 {
        let Vec3(x, y, z) = self.0;
        x.abs() + y.abs() + z.abs()
    }
}

#[test]
fn test_particle() {
    assert_eq!(
        Particle::parse("p=<3,-2,1>, v=<2,-1,-3>, a=<-1,-2,-3>"),
        Particle(Vec3(3, -2, 1), Vec3(2, -1, -3), Vec3(-1, -2, -3))
    );

    let mut p1 = Particle::parse("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>");
    let mut p2 = Particle::parse("p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>");

    p1.update();
    p2.update();
    assert_eq!(p1, Particle(Vec3(4, 0, 0), Vec3(1, 0, 0), Vec3(-1, 0, 0)));
    assert_eq!(p2, Particle(Vec3(2, 0, 0), Vec3(-2, 0, 0), Vec3(-2, 0, 0)));

    assert_eq!(p1.distance_from_center(), 4);
    assert_eq!(p2.distance_from_center(), 2);
}

struct Collider {
    particles: Vec<Option<Particle>>,
}

impl Collider {
    fn new(particles: Vec<Particle>) -> Self {
        Self { particles: particles.iter().map(|p| Some(*p)).collect() }
    }

    fn tick(&mut self) -> usize {
        let mut removed = 0;
        let mut positions: HashMap<Vec3, Vec<usize>> = HashMap::new();

        for (i, particle_opt) in self.particles.iter_mut().enumerate() {
            if let &mut Some(ref mut particle) = particle_opt {
                particle.update();
                positions.entry(particle.0).or_insert(vec![]).push(i);
            }
        }

        for (_pos, particle_indicies) in positions.iter() {
            if particle_indicies.len() > 1 {
                for index in particle_indicies {
                    removed += 1;
                    self.particles[*index].take();
                }
            }
        }

        removed
    }

    fn count(&self) -> usize {
        self.particles.iter().filter(|maybe| maybe.is_some()).count()
    }
}

#[test]
fn test_collider() {
    let p1 = Particle::parse("p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>");
    let p2 = Particle::parse("p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>");
    let p3 = Particle::parse("p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>");
    let p4 = Particle::parse("p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>");
    let mut collider = Collider::new(vec![p1, p2, p3, p4]);

    assert_eq!(collider.tick(), 0);
    assert_eq!(collider.count(), 4);
    assert_eq!(collider.tick(), 3);
    assert_eq!(collider.count(), 1);
}

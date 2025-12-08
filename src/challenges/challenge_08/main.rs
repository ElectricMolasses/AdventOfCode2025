#[warn(warnings, unused_imports)]

use std::cmp::Ordering;
use std::{borrow::Borrow, collections::{HashMap, HashSet}};

use crate::util::read_by_line;

fn calc_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let JunctionBox { x: x1, y: y1, z: z1 } = a;
    let (x1, y1, z1) = (*x1 as f64, *y1 as f64, *z1 as f64);
    let JunctionBox { x: x2, y: y2, z: z2 } = b;
    let (x2, y2, z2) = (*x2 as f64, *y2 as f64, *z2 as f64);

    (((x2 - x1).powf(2.) + (y2 - y1).powf(2.) + (z2 - z1).powf(2.)) as f64).sqrt()
}

#[derive(Debug, Clone, Hash)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl Eq for JunctionBox {}

impl PartialEq for JunctionBox {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for JunctionBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JunctionBox {
    fn cmp(&self, other: &Self) -> Ordering {
        let zero_box = JunctionBox { x: 0, y: 0, z: 0 };
        calc_distance(self, &zero_box).partial_cmp(&calc_distance(other, &zero_box)).unwrap()
    }
}

fn parse_junction_input(input: &str) -> Vec<JunctionBox> {
    let mut jb: Vec<JunctionBox> = Vec::new();

    for line in input.lines() {
        let data: Vec<usize> = line.split(",").map(|d| d.parse::<usize>().unwrap()).collect();

        jb.push(JunctionBox { x: data[0], y: data[1], z: data[2] });
    }

    jb
}

#[derive(Debug)]
struct JunctionPair {
    a: JunctionBox,
    b: JunctionBox,
    distance: f64,
}

fn sort_junction_pairs(jbs: &Vec<JunctionBox>) -> Vec<JunctionPair> {
    let mut pairs: Vec<JunctionPair> = Vec::new();

    for (idx, jbox) in jbs.iter().enumerate() {
        for ojbox in jbs[idx+1..].iter() {
            pairs.push(JunctionPair { 
                a: jbox.clone(), 
                b: ojbox.clone(), 
                distance: calc_distance(jbox, ojbox) 
            });
        }
    }

    pairs
}


fn connect_n_nearest_jbs(pairs: &Vec<JunctionPair>, n: usize) -> Vec<HashSet<JunctionBox>> {
    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();

    for pair in pairs[..n].iter() {
        let mut has_existing_circuit: bool = false;

        let mut match_a: Option<usize> = None;
        let mut match_b: Option<usize> = None;

        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&pair.a) { match_a = Some(idx) }
            if circuit.contains(&pair.b) { match_b = Some(idx) }
        }

        if match_a.is_some() && match_b.is_some() {
            let mv_a = match_a.unwrap();
            let mv_b = match_b.unwrap();

            if mv_a == mv_b { continue }

            for jb in circuits[mv_b].clone() {
                circuits[mv_a].insert(jb);
            }

            circuits.remove(mv_b);
            
            continue
        }

        if match_a.is_none() && match_b.is_none() {
            circuits.push(HashSet::new());
            circuits.last_mut().unwrap().insert(pair.a.clone());
            circuits.last_mut().unwrap().insert(pair.b.clone());

            continue
        }

        if match_a.is_some() { 
            circuits.get_mut(match_a.unwrap()).unwrap().insert(pair.b.clone()); 
        }
        if match_b.is_some() { 
            circuits.get_mut(match_b.unwrap()).unwrap().insert(pair.a.clone()); 
        }
    }

    circuits
}

pub fn run_challenge_08_00(input_name: &str, n: usize) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_08/".to_owned() + input_name));

    let junction_boxes: Vec<JunctionBox> = parse_junction_input(&input);
    let mut sorted_jbs = sort_junction_pairs(&junction_boxes);
    sorted_jbs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    println!("=============================");
    println!("Ten nearest junction boxes.");
    println!("=============================");
    println!("{:?}", sorted_jbs.chunks(10).next());

    let circuits = connect_n_nearest_jbs(&sorted_jbs, n);
    println!("=============================");
    println!("Created Circuits");
    println!("=============================");
    println!("{:?}", circuits);

    println!("=============================");
    println!("Circuit Sizes");
    println!("=============================");
    println!("{:?}", circuits.iter().map(|s| s.len()).collect::<Vec<usize>>());

    let mut circuit_counts: Vec<usize> = circuits.iter()
                                          .map(|s| s.len())
                                          .collect::<Vec<usize>>();
    circuit_counts.sort();
    circuit_counts.reverse();
    println!("=============================");
    println!("Circuit Sizes Sorted");
    println!("=============================");
    println!("{:?}", circuit_counts);

    println!("=============================");
    println!("Number of Junction Boxes in all Circuits");
    println!("=============================");
    println!("{:?}", circuit_counts.iter().sum::<usize>());

    circuit_counts[..3].iter().product::<usize>() as u64
}

fn connect_jbs_until_one_circuit(pairs: &Vec<JunctionPair>, jb_count: usize) -> (
    Vec<HashSet<JunctionBox>>,
    JunctionBox, JunctionBox
    ) {
    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();

    let mut last_a: JunctionBox = pairs[0].a.clone();;
    let mut last_b: JunctionBox = pairs[0].b.clone();;

    for pair in pairs {
        println!("{:?}", circuits.len());
        // If the first set is the same size as the jb_count, we've
        //  created one circuit with every jb
        if let Some(first) = circuits.first() {
            if first.len() == jb_count {
                break;
            }
        }

        last_a = pair.a.clone();
        last_b = pair.b.clone();

        let mut match_a: Option<usize> = None;
        let mut match_b: Option<usize> = None;

        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&pair.a) { match_a = Some(idx) }
            if circuit.contains(&pair.b) { match_b = Some(idx) }
        }

        if match_a.is_some() && match_b.is_some() {
            let mv_a = match_a.unwrap();
            let mv_b = match_b.unwrap();

            if mv_a == mv_b { continue }

            for jb in circuits[mv_b].clone() {
                circuits[mv_a].insert(jb);
            }

            circuits.remove(mv_b);
            
            continue
        }

        if match_a.is_none() && match_b.is_none() {
            circuits.push(HashSet::new());
            circuits.last_mut().unwrap().insert(pair.a.clone());
            circuits.last_mut().unwrap().insert(pair.b.clone());

            continue
        }

        if match_a.is_some() { 
            circuits.get_mut(match_a.unwrap()).unwrap().insert(pair.b.clone()); 
        }
        if match_b.is_some() { 
            circuits.get_mut(match_b.unwrap()).unwrap().insert(pair.a.clone()); 
        }
    }

    (circuits, last_a, last_b)
}

pub fn run_challenge_08_01(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_08/".to_owned() + input_name));

    let junction_boxes: Vec<JunctionBox> = parse_junction_input(&input);
    let mut sorted_jbs = sort_junction_pairs(&junction_boxes);
    sorted_jbs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // println!("=========================");
    // println!("Junction Boxes by Distance");
    // println!("=========================");
    // println!("{:?}", sorted_jbs);

    let (circuits, jb_a, jb_b) = connect_jbs_until_one_circuit(&sorted_jbs, junction_boxes.len());
    println!("=========================");
    println!("Circuits");
    println!("=========================");
    println!("{:?}", circuits);
    println!("{}", circuits.len());
    println!("{}", circuits.first().unwrap().len());

    println!("=========================");
    println!("Final Junction Box Connection");
    println!("=========================");
    println!("Final A::{:?}", jb_a);
    println!("Final B::{:?}", jb_b);

    (jb_a.x * jb_b.x) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_08_00("input_sample_a", 10), 40);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_08_01("input_sample_a"), 25272);
    }
}

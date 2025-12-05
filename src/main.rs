mod challenges;
mod util;

use crate::challenges::{
    challenge_01::main::{run_challenge_00,run_challenge_01},
    challenge_02::main::{run_challenge_02_00,run_challenge_02_01},
    challenge_03::main::{run_challenge_03_00,run_challenge_03_01},
    challenge_04::main::{run_challenge_04_00,run_challenge_04_01},
    challenge_05::main::{run_challenge_05_00,run_challenge_05_01},
};

fn main() {
    // println!("{}", run_challenge_00());
    // println!("{}", run_challenge_01());
    // println!("{}", run_challenge_02_00("input"));
    // println!("{}", run_challenge_02_01("input"));
    // println!("{}", run_challenge_03_00("input"));
    // println!("{}", run_challenge_03_01("input"));
    // println!("{}", run_challenge_04_00("input"));
    // println!("{}", run_challenge_04_01("input"));
    // println!("{}", run_challenge_05_00("input"));
    println!("{}", run_challenge_05_01("input"));
}

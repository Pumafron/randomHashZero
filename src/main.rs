use rand::{distributions::Alphanumeric, Rng};
use sha256::digest;
use std::thread;
use std::time::Duration;
extern crate num_cpus;

const ZEROS: u8 = 7;
fn main() {
    let duration = Duration::from_secs(u64::MAX);
    let num = num_cpus::get();
    for ilo in 1..num {
        spawn(ilo)
    }
    println!("hi hashing {} from the main thread!", 2);
    thread::sleep(duration);
}
fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

fn compare_if_is_zero(hash: &String, value: &str) -> bool {
    hash.starts_with(value)
}

fn starthashing(value_as_str: &str) {
    loop {
        let string_random = random_string();
        let hash = digest(string_random.clone());
        if compare_if_is_zero(&hash, &value_as_str) {
            {
                println!("{}, {}",string_random,&hash);
            }
        }
    }
}

fn spawn(id: usize) {
    println!("hi number {} from the spawned thread!", id);
    thread::spawn(|| {
        let mut value = String::new();
        for _ in 0..ZEROS {
            value.push_str("0");
        }
        let value_as_str = value.as_str();
        starthashing(value_as_str)
    });
}

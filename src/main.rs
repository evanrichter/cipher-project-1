// these "mod" statements bring in ciphers/mod.rs, dict.rs, gen.rs, and utils.rs files
mod ciphers;
mod crack;
mod dict;
mod gen;
mod rng;
mod utils;

use ciphers::schedulers::{RandomBaseScheduler, RandomScheduler};
use rng::{FromRng, Rng};

/*
use ciphers::Encryptor;
use dict::{BytesDictionary, Dictionary};
use gen::Generator;
use ciphers::{Cipher, KeySchedule};
*/

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    // threads for cracking
    let cpus = num_cpus::get();
    let (schedules, results, _) = crack::worker::spawn_workers(cpus - 2);

    // thread for generating RandomSchedulers
    std::thread::spawn(move || {
        let mut rng = Rng::with_seed(37, 13);

        loop {
            let sched = RandomScheduler::from_rng(&mut rng);

            // repeat the same schedule 5 times
            for _ in 0..5 {
                schedules.send(sched.clone()).unwrap();
            }
        }
    });

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))?;

    let mut stats: Vec<(usize, f64)> = vec![(0, 0.0); 9];

    // (main) thread for printing results
    while running.load(Ordering::SeqCst) {
        // receive a single result
        let (sched, keylen, success) = results.recv().unwrap();

        // get success as a percentage
        let success = (1.0 - (success as f64).min(1.0)) * 100.0;

        // increase total attempt count and update total success
        stats[0].0 += 1;
        stats[0].1 += success;

        let (rand_ind, base_sched) = match sched {
            RandomScheduler::Zero(s) => (1, s),
            RandomScheduler::One(s, _) => (2, s),
            RandomScheduler::Two(s, _, _) => (3, s),
            RandomScheduler::Three(s, _, _, _) => (4, s),
        };

        // update the stats for number of PeriodicRand layers
        stats[rand_ind].0 += 1;
        stats[rand_ind].1 += success;

        let base_sched_ind = match base_sched {
            RandomBaseScheduler::Aab(_) => 5,
            RandomBaseScheduler::LengthMod(_) => 6,
            RandomBaseScheduler::OffsetReverse(_) => 7,
            RandomBaseScheduler::RepeatingKey(_) => 8,
        };

        // update the stats for base scheduler type
        stats[base_sched_ind].0 += 1;
        stats[base_sched_ind].1 += success;

        // print status line
        println!(
            "{:>3}% correct  {:?} keylen: {}",
            success as u8, sched, keylen
        );
    }

    // DONE LOOPING, CTRL-C was pressed
    println!("\n\n Now for some stats! ");

    // calculate the averages
    for (n, total) in stats.iter_mut() {
        *total = *total / *n as f64;
    }

    // print stats for # of PeriodicRand inserted
    for num_rand in 0..=3 {
        println!(
            "Attempted {:>5} ciphertexts with {} layers of PeriodicRand, on average {:>3}% successful",
            stats[num_rand + 1].0,
            num_rand,
            stats[num_rand + 1].1 as u8
        );
    }

    println!();

    // print stats for # of PeriodicRand inserted
    for sched_type in 0..=3 {
        let sched_str = match sched_type {
            0 => "Aab",
            1 => "LengthMod",
            2 => "OffsetReverse",
            3 => "RepeatingKey",
            _ => unreachable!(),
        };

        println!(
            "Attempted {:>5} ciphertexts with scheduler type {:>13}, on average {:>3}% successful",
            stats[sched_type + 5].0,
            sched_str,
            stats[sched_type + 5].1 as u8
        );
    }

    println!();

    // print overall # of tests, and overall average score
    println!(
        "OVERALL: Attempted {} ciphertexts, on average {:>3}% successful",
        stats[0].0, stats[0].1 as u8
    );

    Ok(())
}

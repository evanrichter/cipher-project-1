// these "mod" statements bring in ciphers/mod.rs, dict.rs, gen.rs, and utils.rs files
mod ciphers;
mod crack;
mod dict;
mod gen;
mod rng;
mod utils;

use ciphers::schedulers::RandomScheduler;
use rng::{FromRng, Rng};

/*
use ciphers::Encryptor;
use dict::{BytesDictionary, Dictionary};
use gen::Generator;
use ciphers::{Cipher, KeySchedule};
*/

fn main() -> anyhow::Result<()> {
    // threads for cracking
    let cpus = num_cpus::get();
    let (schedules, results, _) = crack::worker::spawn_workers(cpus - 2);

    // thread for generating RandomSchedulers
    std::thread::spawn(move || {
        let mut rng = Rng::with_seed(13, 37);

        loop {
            let sched = RandomScheduler::from_rng(&mut rng);

            // repeat the same schedule 5 times
            for _ in 0..5 {
                schedules.send(sched.clone()).unwrap();
            }
        }
    });

    // (main) thread for printing results
    loop {
        let (sched, keylen, success) = results.recv().unwrap();
        let success = ((1.0 - success.min(1.0)) * 100.0) as u8;
        println!("{:>3}% correct  {:?} keylen: {}", success, sched, keylen);
    }
}

#[cfg(test)]
#[test]
fn test_main() {
    main().expect("main threw an error");
}

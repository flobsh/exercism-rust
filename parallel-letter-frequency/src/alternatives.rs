//! Alternative solutions

use super::*;
use std::sync::mpsc;
use std::{collections::HashMap, thread};

/// Solution using one hashmap per thread, each thread returning their resulting hashmap
/// on `join`.
/// This is not as efficient as the hashmaps + channel solution on my computer:
/// test bench_large_parallel   ... bench:     382,605 ns/iter (+/- 9,332)
/// test bench_large_sequential ... bench:     511,258 ns/iter (+/- 9,316)
#[allow(dead_code)]
pub fn hash_maps_solution(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let mut workers = Vec::with_capacity(worker_count);

        for chunk in input.chunks(input.len() / worker_count + 1) {
            workers.push(s.spawn(move || letter_counter_hash_map(chunk)));
        }

        workers
            .into_iter()
            .map(|worker| worker.join().expect("Compute thread panicked"))
            .reduce(|mut letters_frequency, worker_frequencies| {
                worker_frequencies.into_iter().for_each(|(c, value)| {
                    letters_frequency
                        .entry(c)
                        .and_modify(|count| *count += value)
                        .or_insert(value);
                });

                letters_frequency
            })
            .unwrap_or(HashMap::new())
    })
}

/// Solution using threads that communicates each letter they encounter to the main thread
/// through a channel.
/// This is the least efficient solution on my computer.
/// test bench_large_parallel   ... bench:   1,360,860 ns/iter (+/- 13,528)
/// test bench_large_sequential ... bench:     515,937 ns/iter (+/- 4,038)
#[allow(dead_code)]
pub fn channel_solution(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut letters_frequency: HashMap<char, usize> = HashMap::new();

    thread::scope(|s| {
        let (sender, receiver) = mpsc::channel::<char>();

        for chunk in input.chunks(input.len() / worker_count + 1) {
            let sender = sender.clone();
            s.spawn(move || letter_counter_channel(chunk, sender));
        }

        drop(sender);

        for c in receiver.iter() {
            letters_frequency
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    });

    letters_frequency
}

#[allow(dead_code)]
fn letter_counter_channel(input: &[&str], sender: Sender<char>) {
    for s in input {
        for c in s
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            sender.send(c).expect("Compute thread failed to send data")
        }
    }
}

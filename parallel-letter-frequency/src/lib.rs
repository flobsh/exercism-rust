//! Computer used for benchmarks:
//! Intel Core i3-7020U @ 2.30GHz
//! 12 GB DDR4 @ 2400 MT/s

use std::sync::mpsc::{self, Sender};
use std::{collections::HashMap, thread};

mod alternatives;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Match pattern inspired by rsalmei's solution
    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => letter_counter_hash_map(input),
        _ => hash_maps_and_channels_solution(input, worker_count),
    }
}

/// Count occurences of letters in a list of strings.
fn letter_counter_hash_map(input: &[&str]) -> HashMap<char, usize> {
    let mut frequency = HashMap::<char, usize>::new();

    for s in input {
        for c in s
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            frequency
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    frequency
}

/// Solution using one hashmap per thread, each thread sending their result hashmap through a channel
/// once they have finished. Main thread then proceed to combine these hashmaps.
/// This is the most efficient solution on my computer:
/// test bench_large_parallel   ... bench:     363,582 ns/iter (+/- 6,258)
/// test bench_large_sequential ... bench:     499,023 ns/iter (+/- 4,903)
fn hash_maps_and_channels_solution(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let (sender, receiver) = mpsc::channel();

        // Create a sender side for each thread but keep the original sender for the last thread.
        let senders: Vec<_> = std::iter::repeat(sender.clone())
            .take(worker_count - 1)
            .chain(std::iter::once(sender))
            .collect();

        // Spawn threads.
        for (chunk, sender) in input.chunks(input.len() / worker_count + 1).zip(senders) {
            s.spawn(move || {
                sender
                    .send(letter_counter_hash_map(chunk))
                    .expect("Compute thread panicked")
            });
        }

        receiver
            .iter()
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

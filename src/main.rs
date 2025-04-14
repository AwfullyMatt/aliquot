//! Aliquot Sequence Classifier
//!
//! This program calculates and classifies aliquot sequences for numbers in a given range,
//! saving results to a CSV file. It features parallel processing and resume capabilities.

use crossbeam_channel::unbounded;
use crossbeam_utils::thread;
use csv::Writer;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;

/// Number of consecutive values to process in each run
const RUN_QUANTITY: u64 = 1000;

/// Main application structure holding processing state
struct AliquotApp {
    /// First number to process in this run
    start_number: u64,
    /// Last number to process in this run
    end_number: u64,
}

impl AliquotApp {
    /// Creates new app instance with calculated processing range
    fn new() -> Self {
        // Get last processed number from CSV or start from 0
        let last_processed = Self::get_last_processed_number();
        // Calculate processing range
        let start_number = last_processed + 1;
        let end_number = start_number + RUN_QUANTITY;

        AliquotApp {
            start_number,
            end_number,
        }
    }

    /// Main processing pipeline
    fn run(self) {
        // Early exit if range is already processed
        if self.start_number > self.end_number {
            println!("Processing complete up to {}", self.end_number);
            return;
        }

        // Create communication channel between workers and writer
        let (sender, receiver) = unbounded::<(u64, String, Vec<u64>)>();

        // Use scoped threads for safe resource management
        thread::scope(|s| {
            // WRITER THREAD: Handles CSV output
            s.spawn(|_| {
                // Open CSV file in append mode, create if missing
                let mut wtr = Writer::from_writer(
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("aliquot.csv")
                        .unwrap(),
                );

                // Process incoming results until channel closes
                for (n, class, seq) in &receiver {
                    // Convert sequence numbers to comma-separated string
                    let seq_str = seq
                        .iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>()
                        .join(",");

                    // Write record: [number, classification, sequence]
                    wtr.write_record(&[n.to_string(), class, seq_str]).unwrap();
                    // Flush after each write for immediate persistence
                    wtr.flush().unwrap();
                }
            });

            // WORKER THREADS: Parallel sequence calculation
            let num_workers = num_cpus::get(); // Use all available CPU cores
            let total_numbers = self.end_number - self.start_number + 1;
            // Split work into equal chunks per worker
            let chunk_size = (total_numbers as f64 / num_workers as f64).ceil() as u64;

            // Create worker for each chunk
            for i in 0..num_workers {
                let sender = sender.clone(); // Clone channel sender
                // Calculate chunk boundaries
                let start = self.start_number + i as u64 * chunk_size;
                let end = (start + chunk_size - 1).min(self.end_number);

                // Skip if chunk is empty (edge case handling)
                if start > end {
                    break;
                }

                // Spawn worker thread for this chunk
                s.spawn(move |_| {
                    // Process each number in the chunk
                    (start..=end).for_each(|n| {
                        // Calculate sequence and classification
                        let (classification, sequence) = classify_aliquot(n, 1000);
                        // Send result to writer thread
                        sender.send((n, classification, sequence)).unwrap();
                    });
                });
            }

            // Drop original sender after cloning for workers
            drop(sender);
        })
        .unwrap();

        println!("Processing completed successfully!");
    }

    /// Reads CSV to find last processed number
    fn get_last_processed_number() -> u64 {
        // Open CSV file or return 0 if not found
        let file = match OpenOptions::new().read(true).open("aliquot.csv") {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return 0,
            Err(e) => panic!("CSV read failed: {}", e),
        };

        let mut rdr = csv::Reader::from_reader(file);
        let mut max_num = 0u64;

        // Iterate through all records
        for result in rdr.records() {
            let record = match result {
                Ok(r) => r,
                Err(_) => continue, // Skip malformed records
            };

            // Parse first column as number
            if let Ok(n) = record.get(0).unwrap_or("").parse::<u64>() {
                // Track highest number found
                max_num = max_num.max(n);
            }
        }

        max_num
    }
}

/// Mathematical Functions -----------------------------------------------------

/// Calculates sum of proper divisors of a number
/// Proper divisors are numbers less than n that divide evenly into n
fn sum_proper_divisors(n: u64) -> u64 {
    if n < 2 {
        return 0; // Numbers < 2 have no proper divisors
    }
    let mut sum = 1u64; // 1 is always a proper divisor for n > 1
    let sqrt_n = (n as f64).sqrt() as u64; // Only check up to sqrt(n)

    for i in 2..=sqrt_n {
        if n % i == 0 {
            // Add both divisor pairs (i and n/i)
            sum = sum.saturating_add(i); // Prevent overflow with saturating math
            let other = n / i;
            if other != i {
                sum = sum.saturating_add(other);
            }
        }
    }
    sum
}

/// Sequence Classification ----------------------------------------------------

/// Generates aliquot sequence and classifies its behavior
///
/// A sequence is classified as:
/// - Perfect: Repeats immediately (e.g., 6 → 6)
/// - Amicable: 2-number cycle (e.g., 220 ↔ 284)
/// - Sociable: 3+ number cycle
/// - Aspiring: Ends in a perfect number
/// - Terminating: Reaches zero
/// - Non-terminating: No pattern found in max_steps
fn classify_aliquot(start: u64, max_steps: usize) -> (String, Vec<u64>) {
    let mut sequence = Vec::with_capacity(max_steps + 1); // Pre-allocate memory
    let mut seen = HashMap::<u64, usize>::new(); // Track number → index
    let mut cycle_check = HashSet::<u64>::new(); // Track potential cycles
    sequence.push(start);
    seen.insert(start, 0);

    for step in 0..max_steps {
        let current = sequence[step];
        let next = sum_proper_divisors(current);

        // Termination check: Sequence reached zero
        if next == 0 {
            sequence.push(0);
            return ("terminating".to_string(), sequence);
        }

        // Known non-terminating sequences (https://oeis.org/A131884)
        if cycle_check.contains(&next) || [276, 552, 564, 660, 966, 1074].contains(&next) {
            return ("non-terminating".to_string(), sequence);
        }

        // Check for repeating patterns
        if let Some(&first_occurrence) = seen.get(&next) {
            let cycle = &sequence[first_occurrence..];

            // Found cycle containing starting number
            if cycle.contains(&start) {
                return match cycle.len() {
                    1 => ("perfect".to_string(), cycle.to_vec()),
                    2 => ("amicable".to_string(), cycle.to_vec()),
                    _ => ("sociable".to_string(), cycle.to_vec()),
                };
            }
            // Check for aspiring numbers (end in perfect cycle)
            else if cycle.len() == 1 && sum_proper_divisors(cycle[0]) == cycle[0] {
                return ("aspiring".to_string(), sequence);
            }

            // Record cycle elements to detect non-terminating patterns
            cycle_check.extend(cycle.iter().copied());
        }

        // Record next number and continue sequence
        seen.insert(next, sequence.len());
        sequence.push(next);
    }

    // No classification found within max steps
    ("non-terminating".to_string(), sequence)
}

/// Entry Point -----------------------------------------------------------------

fn main() {
    // Create and run application with automatic range calculation
    AliquotApp::new().run();
}

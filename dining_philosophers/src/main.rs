use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};

// Solution implemented using shared-data, resource ordering
// Resource ordering: 1, 2, 3, 4, 5
// Philosopher i takes fork i and (i+1)%6

// main launches 5 philosopher threads
fn main() {
    // Create a shared flag to indicate ^C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // ^C Handler
    ctrlc::set_handler(move || {
        println!("Ctrlc pressed, stopping philosophers.");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrlc handler");
    
    // forks are shared data - just mutexes
    let forks = Arc::new([
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
        Mutex::new(0),
    ]);

    // Create vector to hold thread handles
    let mut handles = vec![];

    // Spawn philosopher threads
    for i in 0..5 {
        let forks_clone = Arc::clone(&forks);
        let running_clone = Arc::clone(&running);
        let handle = thread::spawn(move || {
            philosopher(i, running_clone, forks_clone)
        });
        handles.push(handle);
    }

    // Wait for all threads to finish - allows program to end gracefully
    let mut results = vec![];
    for handle in handles {
        let result: (i32, i32, i32) = handle.join().unwrap();
        results.push(result);
    }

    results.sort_by_key(|tuple| tuple.0);
    println!("\nResults:");
    for result in results {
        println!("Philosopher {} thought {} times and ate {} times.", result.0 + 1, result.1, result.2);
    }

    println!("\nAll philosophers done, program ending");
}


// each philosopher is a thread
// want to eat at random time intervals, must aquire fork locks in order
fn philosopher(number: i32, running: Arc<AtomicBool>, forks_clone: Arc<[Mutex<i32>]>) -> (i32, i32, i32) {
    let mut think_count:i32 = 0;
    let mut eat_count:i32 = 0;

    let left: usize = number.try_into().unwrap();
    let right: usize = ((number+1)%5).try_into().unwrap();
    let first = std::cmp::min(left, right);
    let second = std::cmp::max(left, right);

    while running.load(Ordering::SeqCst) {
        {
            // Accquire forks in order
            let _lock_1 = forks_clone[first].lock().unwrap();
            let _lock_2 = forks_clone[second].lock().unwrap();

            // Eat for random amount of time between 0-3 seconds
            println!("Philosopher {} is eating", number + 1);
            eat_count += 1;
            let mut rng = rand::thread_rng();
            let delay_seconds = rng.gen_range(0..=3);
            thread::sleep(Duration::from_secs(delay_seconds));
        }

        // Think for random amount of time between 0-5 seconds
        println!("Philosopher {} is thinking", number + 1);
        think_count += 1;
        let mut rng = rand::thread_rng();
        let delay_seconds = rng.gen_range(0..=5);
        thread::sleep(Duration::from_secs(delay_seconds));
    }
    
    (number, think_count, eat_count)
}


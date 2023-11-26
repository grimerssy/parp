use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    (0..10)
        .map(|_| Arc::clone(&counter))
        .map(|pointer| move || *pointer.lock().unwrap() += 1)
        .map(std::thread::spawn)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("Result: {}", *counter.lock().unwrap());
}

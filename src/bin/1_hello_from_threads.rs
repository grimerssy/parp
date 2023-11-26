fn main() {
    (1..=10)
        .map(|number| move || println!("Hello from thread {number}"))
        .map(std::thread::spawn)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}

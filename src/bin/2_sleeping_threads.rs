fn main() {
    helper("Sequential", sequential);
    helper("Parallel", parallel);
}

fn helper(name: &str, f: impl FnOnce()) {
    println!("{}:", name);
    let start = std::time::Instant::now();
    f();
    println!("Time elapsed: {:?}\n", start.elapsed());
}

fn sequential() {
    (1..=10).for_each(|number| {
        println!("Thread {number} started");
        std::thread::sleep(std::time::Duration::from_millis(300));
        println!("Thread {number} finished");
    })
}

fn parallel() {
    (1..=10)
        .map(|number| {
            move || {
                println!("Thread {number} started");
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread {number} finished");
            }
        })
        .map(std::thread::spawn)
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}

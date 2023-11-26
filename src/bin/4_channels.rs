fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();
    let _ = (1..=10)
        .map(|number| (sender.clone(), number))
        .map(|(sender, number)| move || sender.send(number).unwrap())
        .map(std::thread::spawn)
        .collect::<Vec<_>>();
    (1..=10)
        .map(|_| receiver.recv().unwrap())
        .for_each(|number| println!("Received: {}", number));
}

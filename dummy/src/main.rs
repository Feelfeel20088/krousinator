fn main() {
    println!("Target running");
    loop {
        println!("running");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    // todo!();
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    println!("data: {}", timestamp);
}
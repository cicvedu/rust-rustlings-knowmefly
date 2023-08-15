// build.rs

// use std::env;

fn main() {
    // let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    // env::set_var("TEST_FOO", timestamp.to_string());
    // Set the TEST_FOO environment variable to a specific value
    // env::set_var("TEST_FOO", "1692107550");
    
    let time=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=TEST_FOO={}",time.to_string());
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
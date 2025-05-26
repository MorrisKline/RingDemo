use ring::digest;

fn main() {

    let data = b"Hello, world!";
    
    let hash: digest::Digest = digest::digest(&digest::SHA256, data);

    let hex_string: String = hash.as_ref().iter().map(|byte| format!("{:02x}", byte)).collect();

    println!("SHA-256 hash: {}", hex_string);
}

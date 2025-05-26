use ring::digest;

fn main() {
    
    let data = b"Hello, world!";
    
    let hash: digest::Digest = digest::digest(&digest::SHA256, data);

    print!("SHA-256 hash: ");
    for byte in hash.as_ref() {
        print!("{:02x}", byte);
    }
    println!();
}

use ring::digest;

fn main() {
    // 待哈希的数据
    let data = b"Hello, world!";
    
    // 计算 SHA-256 哈希值
    let hash: digest::Digest = digest::digest(&digest::SHA256, data);

    // 将字节切片转为十六进制字符串
    let hex_string: String = hash.as_ref().iter().map(|byte| format!("{:02x}", byte)).collect();

    // 打印哈希值
    println!("SHA-256 hash: {}", hex_string);
}

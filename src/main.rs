//Simple Rust cryptorgraphy (encryption, ciphers)
mod base64;
mod rsa;
fn main() {
    //String to put cipher
    let s : &str = "based64";

    //Encode with base64
    let encoded = base64::encode(s);

    //Decoding base64
    let decoded = base64::decode(&encoded);

    //Print results
    println!("Encoded: {}\nDecoded: {}", encoded, decoded);

    rsa::rsa_test1();
}

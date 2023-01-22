//Simple Rust cryptorgraphy (encryption, ciphers)
mod base64;
fn main() {
    //String to put cipher
    let s : &str = "based64";

    //Encode with base64
    let encoded = base64::encode(s);

    //Print results
    println!("{}", encoded);
}

/*
RSA asymmetric encryption algorithm. This algorithm requires use of prime numbers (p and q)
and obtaining the product of those two prime numbers (also known as the co-prime, N).
The totient (T), also known as Phi(n), is a value that is also required for the algorithm.
Phi(n) can be found from finding the number of non-common factors counting up to n.
A common rule is that Phi(n) where n is prime usually equals to n - 1. 
Phi is also multiplicitive i.e
Phi(a * b) = (a - 1)(b - 1) <- a and b must be primes

With these values a public key and private key can be calculated. These keys follow some rules:
Public Key must:
- Be prime
- Be less than Phi(p * q)
- Cannot be a factor of Phi(p * q)

Private Key must:
- (Private Key * Public Key) % Phi(p * q) = 1

The encryption algorithm goes as:
Message ** Public key % N = Cipher

Where decryption goes as:
Cipher ** Private Key % N = Message

Most modern use cases of the RSA algorithm use values that go up to 1024, 2048 or 4096 bits
*/


//Find the Greatest Common Divisor
fn gcd(e : i32, z : i32) -> i32 {
    if e == 0 { z }
    else {
        gcd(z % e, e)
    }
}

//Extended Euclidean Algorithm
//https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn egcd(a : i64, b : i64) -> (i64, i64, i64) {
    let (mut s, mut _s) = (0, 1);
    let (mut t, mut _t) = (1, 0);
    let (mut r, mut _r) = (b, a);

    while r != 0 {
        let quotient = _r / r;

        (_r, r) = (r, _r - quotient * r);
        (_s, s) = (s, _s - quotient * s);
        (_t, t) = (t, _t - quotient * t);
    }

    //Return tuple
    (_r, _s, _t)
}

//Modulo Inverse
fn modularInv(a : i64, b : i64) -> i64 {
    let (gcd, mut x, y) = egcd(a, b);

    if x < 0 {
        x += b;
    }

    x
}

//Encryption method
pub fn encrypt<T: Into<String> + Copy>(e : i64, N : i64, _s : T) -> Vec<i64> {
    let mut cipher = String::new();

    let mut test : Vec<i64> = Vec::new();

    let msg : Vec<u8> = _s.into().as_bytes().to_vec();

    for m in msg {
        let m_ = i64::pow(m as i64, e.try_into().unwrap()) % N;
        cipher.push(m as char);
        test.push(m_);
    }

    test
}

//Decryption method
pub fn decrypt<T: Into<String> + Copy>(e : i64, N : i64, _s : T) {

}

//So far only have encryption.
pub fn rsa_test1() {
    let p = 7;
    let q = 19;

    let N = p * q;
    let phiN = (p - 1) * (q - 1);

    let e = 5;
    let d = modularInv(e, phiN);

    let enc = encrypt(e.into(), N.into(), "<");

    dbg!(d, phiN, enc);
    //println!("{enc}");
}

//Not a full implementation or finished implementation
//Mainly just a simple and small test implementation
#[test]
pub fn rsa_test2() {
    //Start with a basic message
    let msg : i32 = 19;

    //Initialize p and q (starting small for now)
    let (p, q) = (3, 7);

    //Initialize the totient
    let totient = (p - 1) * (q - 1);

    //Public key
    let mut E = 2;

    //Find the value of the Public key (with the rules specified)
    for i in 2..totient {
        if gcd(E, totient) == 1 {
            break;
        }
        E += 1;
    }

    //Private key. Find the value of the Private key (with the rules specified)
    let mut D = 0;

    for i in (0..=9) {
        let x = 1 + (i * totient);

        //D private key
        if x % E == 0 {
            D = x / E;
            break;
        }
    }

    //Encrypted message
    let encrypted = i32::pow(msg, E.try_into().unwrap()) % (p * q);

    let decrypted = i32::pow(encrypted, D.try_into().unwrap()) % (p * q);
    
    println!("E = {E}, D = {D}, Phi(p*q) = {totient}");

    println!("Encrypted: {encrypted}\nDecrypted: {decrypted}");

}
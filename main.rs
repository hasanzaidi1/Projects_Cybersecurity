// Import necessary crates and modules
use rand::Rng;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive; 


// Function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to generate random prime numbers
fn generate() -> (u32, u32) {
    let mut random_number_gen = rand::thread_rng();
    let p: u32 = random_number_gen.gen_range(1000..5000);  // Adjust the range as needed
    let q: u32 = random_number_gen.gen_range(1000..5000);  // Adjust the range as needed
    (p, q)
}

// Function to perform modular exponentiation (base^exponent mod modulus)
fn mod_pow(base: BigInt, exponent: BigInt, modulus: BigInt) -> BigInt {
    base.modpow(&exponent, &modulus)
}



// Function to encrypt a string
fn encrypt(e: &BigInt, n: &BigInt, p: &str) -> Vec<BigInt> {
    let mut encrypted: Vec<BigInt> = Vec::new();
    for ascii_val in p.bytes() {
        let c = BigInt::from(ascii_val);
        let encrypted_val = mod_pow(c.clone(), e.clone(), n.clone());
        encrypted.push(encrypted_val);
    }
    encrypted
}

// Function to decrypt a string and convert to Unicode characters
fn decrypt(d: &BigInt, n: &BigInt, encrypted_ascii: Vec<BigInt>) -> String {
    let mut decrypted_chars = String::new();

    for c in encrypted_ascii {
        let decrypted_val = mod_pow(c.clone(), d.clone(), n.clone());

        // Ensure the decrypted value is within the valid ASCII range
        if let Some(ascii_char) = decrypted_val.to_u32() {
            // Convert valid ASCII values to characters
            if ascii_char <= 127 {
                decrypted_chars.push(ascii_char as u8 as char);
            } else {
                // Handle out-of-range ASCII values
                decrypted_chars.push('?');
            }
        } else {
            // Handle invalid ASCII values
            decrypted_chars.push('?');
        }
    }

    decrypted_chars
}





fn main() {
    let n: u32;
    let t: u32;
    let e: u32;
    let d: u32;

    // Generate random prime numbers and calculate public and private keys
    loop {
        let (p_val, q_val) = generate();
        if is_prime(p_val) && is_prime(q_val) {
            println!("p = {}, q = {}", p_val, q_val);
            n = p_val * q_val;
            println!("n = {}", n);
            t = (p_val - 1) * (q_val - 1);
            println!("totient(t) = {}", t);
            e = 65537;  // Using commonly used public key value
            println!("public key e = {}", e);
            d = t - 1;  // d is modular inverse of e mod t
            println!("private key d = {}", d);
            break;
        } else {
            println!("Generated p = {}, q = {}. Retrying...", p_val, q_val);
        }
    }

    let p = "The greatest discovery of all time is that a person can change his future by merely changing his attitude";

    // Converting n, e, d to BigInt
    let n_bigint = BigInt::from(n);
    let e_bigint = BigInt::from(e);
    let d_bigint = BigInt::from(d);

    // Encrypt and decrypt using BigInt
    let encrypted = encrypt(&e_bigint, &n_bigint, p);
    println!("Encrypted: {:?}", encrypted);
    println!("Decrypted: {:?}", decrypt(&d_bigint, &n_bigint, encrypted));
}

fn main() {
    println!("{:?}",gen_primes(100000).len())
}

// gen_primes returns a vector of primes up to the specified limit
fn gen_primes(limit: u64) -> Vec<u64> {
    let mut res = Vec::new();
    for i in 2..limit {
        if is_prime(i) {
            res.push(i);
        }
    }
    return res;
}

// is_prime checks wether or not a number is prime
fn is_prime(n: u64) -> bool {
    for current in 2..n {
        if n % current == 0 {
            return false
        }
    }
    return true;
}

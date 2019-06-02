use crate::factor::pollard;
use ramp::Int;

#[cfg(test)]
mod tests;

pub fn totient(n: &Int) -> Int {
    let zero = Int::zero();
    let one = Int::one();
    if *n == zero {
        zero
    } else if *n == one {
        one
    } else {
        let pes = pollard::factor(&n);
        let mut tot = Int::one();
        
        for (p, e) in pes.into_iter() {
            tot *= p.pow(e-1) * (&p - &one);
        }
        tot
    }
}

pub fn sigma(n: &Int) -> Int {
    let pes = pollard::factor(n);
    let one = Int::one();
    let mut result = Int::one();
    for (p, e) in pes.into_iter() {
        result *= (p.pow(e + 1) - &one) / (p - &one);
    }
    result
}


extern crate ramp;
use ramp::Int;
use std::vec::Vec;
use primes;

fn remove(n: &mut Int, p: &Int) -> usize {
    let mut e = 0;
    let mut pair = n.divmod(p);    
    while pair.1 == 0 {
        e += 1;
        *n = pair.0;
        pair = n.divmod(p);
    }
    e
}

pub fn remove_primes(n: &mut Int, upto: &Int) -> Vec<(Int, usize)> {
    let ps = primes::primes(&Int::from(2), upto);
    let mut result = Vec::new();
    let mut e;
    for p in ps.into_iter() {
        e = remove(n, &p);
        if e > 0 {
            result.push((p, e));
        }
    }
    result
}

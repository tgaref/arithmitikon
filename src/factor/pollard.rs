extern crate rand;
use ramp::Int;
use ramp::RandomInt;
use std::vec::Vec;
use super::common;
use crate::primes;

fn g(x: &Int, n: &Int) -> Int {
    (x * x + 1) % n
}

fn split(n: &Int) -> (Int, Int) {
    let mut rng = rand::thread_rng();
    let high = Int::from(100);
    let low = Int::from(2);
    let mut x = rng.gen_int_range(&low, &high);
    let mut y = x.clone();
    let mut d = Int::one();
    while d == 1 {
        x = g(&x, n);
        y = g(&g(&y, n), n);
        if x == y {
            x = rng.gen_int_range(&low, &high);
            y = x.clone();
        } else {
            d = (&x - &y).abs().gcd(n);
        }
    }
    (n / &d, d)
}

fn pr_fact(n: &Int) -> Vec<Int> {
    if *n == 1 {
        return Vec::new()
    }
    let mut ps = Vec::new();
    let mut ns = vec![n.clone()];
    let mut m;
    while !ns.is_empty() {
        m = ns.pop().unwrap();
        if primes::is_prime(&m, 6) {
            ps.push(m);
        } else {
            let (n1, n2) = split(&m);
            ns.push(n1);
            ns.push(n2);
        }     
    }
    ps.sort();
    ps
}

fn compute_pairs(ps: Vec<Int>) -> Vec<(Int,usize)> {
    if ps.is_empty() {
        return vec![]
    }
    let mut prev = 0;
    let mut e = 1;
    let mut result = Vec::new();
    for i in 1 .. ps.len() {
        if ps[i] == ps[prev] {
            e += 1;
        } else {
            result.push((ps[prev].clone(), e));
            e = 1;
            prev = i;
        }
        
    }
    result.push((ps[prev].clone(), e));
    result
}
                           
pub fn factor(n: &Int) -> Vec<(Int,usize)> {
    if *n < 0 {
        let m = -n;
        return factor(&m)
    }
    
    if *n == 0 || *n == 1 {
        return vec![]
    }
    let mut m = n.clone();
    let mut ps1 = common::remove_primes(&mut m, &Int::from(100));
    let mut ps2 = compute_pairs(pr_fact(&mut m));
    ps1.append(&mut ps2);
    ps1
}


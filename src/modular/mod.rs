use crate::factor::pollard;
use crate::arithfunc;
use ramp::Int;
use std::vec::Vec;

pub fn extgcd(a: &Int, b: &Int) -> (Int, Int, Int) {
    if *a < 0 {
        let (d, s, t) = extgcd(&(-a), b);
        return (d, -s, t);
    }

    if *b < 0 {
        let (d, s, t) = extgcd(a, &(-b));
        return (d, s, -t);
    }
        
    if *b > *a {
        let (d, t, s) = extgcd(b, a);
        return (d, s, t);
    }

    if *b == 0 {
        return (a.clone(), Int::one(), Int::zero());
    }

    let mut s_old = Int::one();
    let mut s_new = Int::zero();
    let mut t_old = Int::zero();
    let mut t_new = Int::one();
    let mut pair = (Int::one(), b.clone());
    let mut tmp: Int;
    let mut r = a.clone();

    while pair.1 > 0 {
        tmp = pair.1;
        pair = r.divmod(&tmp);
        r = tmp;
        
        tmp = s_new.clone();
        s_new = s_old - &pair.0 * &tmp;
        s_old = tmp;

        tmp = t_new.clone();
        t_new = t_old - &pair.0 * &tmp;
        t_old = tmp;
    }
    (r, s_old, t_old)
}

pub fn invert_mod(a: &Int, n: &Int) -> Option<Int> {
    if *n == 0 {
        return None;
    }
    let (d, _, mut t) = extgcd(n, a);
    if d != 1 {
        return None;
    }
    t = t % n;
    if t< 0 {
        Some(t + n)
    } else {
        Some(t)
    }
}

pub fn crt(avec: &Vec<Int>, nvec: &Vec<Int>) -> Int {
    let len = nvec.len();
    if len != avec.len() {
        panic!("Number of RHS values != number of moduli");
    }
    let mut n = Int::one();
    for ni in nvec.iter() {
        n *= ni;
    }
    let mut s;
    let mut mi;
    let mut result = Int::zero();
    for i in 0 .. len {
        mi = &n / &nvec[i];
        s = match invert_mod(&mi, &nvec[i]) {
            Some(v) => v,
            None    => panic!("The moduli are not pairwise coprime!"),
        };
        result += (&avec[i] * &mi) * s;
    }
    println!("{:?}", &n);
    result = result % &n;
    if result < 0 {
        result + n
    } else {
        result
    }
}

pub fn solve_linear(a: &Int, b: &Int, n: &Int) -> Vec<Int> {
    let d = a.gcd(n);
    
    if b % &d != 0 {
        return vec![]
    } else {
        let n_ = n / &d;
        let a_ = a / &d;
        let b_ = b / &d;
        let mut x = (b_ * invert_mod(&a_,n).unwrap()) % &n_;
        if x < 0 {
            x = x + &n_;
        }
        let mut result = Vec::new();
        result.push(x.clone());
        let mut i = Int::one();
        while i < d {
            x += &n_;
            result.push(x.clone());
            i += 1;
        }
        result.into_iter().map(|v| if v < 0 {v + n} else {v}).collect()
    }
}

pub fn order(a: &Int, n: &Int) -> Int {
    if *a == 0 || *n == 0 {
        panic!("No order exists!");
    } else {
        let m = arithfunc::totient(n);
        let pes = pollard::factor(&m);
        let mut b;
        let mut q;
        let mut result = Int::one();
        for (p, e) in pes.into_iter() {
            q = Int::one();
            b = a.pow_mod(&(&m / &p.pow(e)) , n);
            while b != 1 {
                q *= &p;
                b = b.pow_mod(&p, n);
            }
            result *= q;
        }
        result
    }
}                    

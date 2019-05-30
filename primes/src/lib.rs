extern crate ramp;
extern crate rand;
use ramp::Int;
use ramp::RandomInt;
use std::vec::Vec;

fn remove_2(n: &Int) -> (u32, Int) {
    let mut s = 0;
    let mut d = n.clone();

    while &d % 2 == 0 {
        d = d / 2;
        s += 1;
    }
    (s,d)
}
    
pub fn is_prime(n: &Int, times: u32) -> bool {
    let ps = vec![Int::from(2), Int::from(3), Int::from(5), Int::from(7), Int::from(11), Int::from(13), Int::from(17), Int::from(19), Int::from(23), Int::from(29), Int::from(31), Int::from(37), Int::from(41), Int::from(43), Int::from(47), Int::from(53), Int::from(59), Int::from(61), Int::from(67), Int::from(71), Int::from(73), Int::from(79), Int::from(83), Int::from(89), Int::from(97)];
    
    for p in ps.iter() {
        if n % p == 0 {
            if n == p {
                return true;
            } else {
                return false;
            }
        }         
    }
      
    let (s,d) = remove_2(&(n - 1));
    let mut rng = rand::thread_rng();
    let mut a: Int;
    let mut k: u32;
    let low = Int::from(2);
    let high = Int::from(n-2);

    'main: for _i in 0 .. times {
        a = rng.gen_int_range(&low, &high);
        if a.gcd(n) != 1 {
            return false;
        }
        a = a.pow_mod(&d, n);
        if a == 1 {
            continue 'main;
        }
        k = 0;                            
        while k < s {
            if a == n - 1 {
                continue 'main;
            } else {
                a = (&a * &a) % n;
                k += 1;
            }
        }     
        return false;
    }
    true
}
        

pub fn primes(init: &Int, upto: &Int) -> Vec<Int> {
    let mut n: Int;
    let mut pr_vec: Vec<Int>;
    
    if *init > 2 {
        pr_vec = vec![];
        if init % 2 == 0 {
            n = init.clone() + 1;
        }
        else {
            n = init.clone();
        }
    } else {
        pr_vec = vec![Int::from(2)];
        n = Int::from(3);
    }

    while n <= *upto {
        if is_prime(&n, 7) {
            pr_vec.push(n.clone());
        }
        n += 2;
    }
    pr_vec
}

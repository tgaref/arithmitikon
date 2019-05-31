extern crate ramp;
extern crate factor;
extern crate arithfunc;
use ramp::Int;
use factor::pollard;
use std::vec::Vec;


pub fn extgcd(a: &Int, b: &Int) -> (Int, Int, Int) {
    if *a < 0 {
        let (d, s, t) = extgcd(-a, b);
        return (d, -s, t);
    }

    if *b < 0 {
        let (d, s, t) = extgcd(a, -b);
        return (d, s, -t);
    }
        
    if *b > *a {
        let (d, t, s) = extgcd(a, b);
        return (d, s, t);
    }

    if *b == 0 {
        return (a, Int::one(), Int::zero());
    }

    let mut s_old = Int::zero();
    let mut s_new = Int::one();
    let mut t_old = Int::one();
    let mut t_new = Int::zero();
    let mut pair = (Int::one(), b.clone());
    let mut tmp: Int;
    let mut r = a;

    while pair.1 > 0 {
        r = pair.1;
        pair = r.divmod(&pair.1);     
        
        tmp = s_old.clone();
        s_old = s_new;
        s_new = tmp - pair.0 * s_new;

        tmp = t_old.clone();
        t_old = t_new;
        t_new = tmp - pair.0 * t_new;
    }
    (pair.1, s_old, t_old)
}

use super::*;

#[test]
fn is_prime_1() {
    let n = Int::from_str_radix("275836748687533950911", 10).unwrap();
    assert!(is_prime(&n, 7), "{:?} is prime!", n);
}

#[test]
fn is_prime_2() {
    let n = Int::from_str_radix("69027111760726712738746210123", 10).unwrap();
    assert!(is_prime(&n, 7), "{:?} is prime!", n);
}

#[test]
fn is_prime_3() {
    let n = Int::from_str_radix("6902711176072612738746210123", 10).unwrap();
    assert!(!is_prime(&n, 7), "{:?} is not prime!", n);
}

#[test]
fn is_prime_4() {
    let n = Int::from_str_radix("34983258327486756345341", 10).unwrap();
    assert!(!is_prime(&n, 7), "{:?} is not prime!", n);
}

#[test]
fn is_prime_5() {
    let n = Int::from_str_radix("34983258327486756345346", 10).unwrap();
    assert!(!is_prime(&n, 7), "{:?} is not prime!", n);
}

#[test]
fn primes_1() {
    let low = Int::from(1207);
    let high = Int:: from(15829);
    assert_eq!(primes(&low, &high).len(), 1650);
}

#[test]
fn primes_2() {
    let low = Int::from(-20);
    let high = Int:: from(1009);
    assert_eq!(primes(&low, &high).len(), 169);
}  

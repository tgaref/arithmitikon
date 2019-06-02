use super::*;
use ramp::Int;

#[test]
fn totient_1() {
    let n = Int::from_str_radix("2758321686748687533950911", 10).unwrap();
    assert_eq!(totient(&n), Int::from_str_radix("2687190653566865900160000",10).unwrap());
}

#[test]
fn totient_2() {
    let n = Int::from_str_radix("34983258327486756345341", 10).unwrap();
    assert_eq!(totient(&n), Int::from_str_radix("34978106912624808649500",10).unwrap());
}

#[test]
fn totient_3() {
    let n = Int::from_str_radix("239", 10).unwrap();
    assert_eq!(totient(&n), Int::from_str_radix("238",10).unwrap());
}

#[test]
fn sigma_1() {
    let n = Int::from_str_radix("2758321686748687533950911", 10).unwrap();
    assert_eq!(sigma(&n), Int::from_str_radix("2829645512751178129934016",10).unwrap());
}

#[test]
fn sigma_2() {
    let n = Int::from_str_radix("34983258327486756345341", 10).unwrap();
    assert_eq!(sigma(&n), Int::from_str_radix("34988409742348704041184",10).unwrap());
}

#[test]
fn sigma_3() {
    let n = Int::from_str_radix("239", 10).unwrap();
    assert_eq!(sigma(&n), Int::from_str_radix("240",10).unwrap());
}


    

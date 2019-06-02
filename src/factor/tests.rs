use super::*;
use ramp::Int;

#[test]
fn pollard_factor_1() {
    let n = Int::from_str_radix("2758321686748687533950911", 10).unwrap();
    assert_eq!(pollard::factor(&n), vec![(Int::from(41), 1), (Int::from(701), 1), (Int::from(180437), 1), (Int::from(2393011), 1), (Int::from(222265853), 1),]);
}

#[test]
fn pollard_factor_2() {
    let n = Int::from(13) * Int::from(13) * Int::from(101) * Int::from(101) * Int::from(101) * Int::from(5101);
    assert_eq!(pollard::factor(&n), vec![(Int::from(13), 2), (Int::from(101), 3), (Int::from(5101), 1)]);
}

#[test]
fn pollard_factor_3() {
    let n = Int::pow(&Int::from(2),15);
    assert_eq!(pollard::factor(&n), vec![(Int::from(2), 15)]);
}

#[test]
fn pollard_factor_4() {
    let n = Int::one();
    assert_eq!(pollard::factor(&n), vec![]);
}

#[test]
fn pollard_factor_5() {
    let n = Int::zero();
    assert_eq!(pollard::factor(&n), vec![]);
}

#[test]
fn pollard_factor_6() {
    let n = Int::from(-12345678);
    assert_eq!(pollard::factor(&n), vec![(Int::from(2), 1), (Int::from(3), 2), (Int::from(47), 1), (Int::from(14593), 1)]);
}

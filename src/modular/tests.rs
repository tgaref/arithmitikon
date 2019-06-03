use super::*;
use ramp::Int;

#[test]
fn extgcd_1() {
    assert_eq!(extgcd(&Int::from(15), &Int::from(13)), (Int::one(), Int::from(-6), Int::from(7)));
}

#[test]
fn extgcd_2() {
    assert_eq!(extgcd(&Int::from(1502), &Int::from(1378)), (Int::from(2), Int::from(-100), Int::from(109)));
}

#[test]
fn extgcd_3() {
    assert_eq!(extgcd(&Int::from(502), &Int::from(1379)), (Int::from(1), Int::from(-228), Int::from(83)));
}

#[test]
fn invert_mod_1() {
    assert_eq!(invert_mod(&Int::from(3), &Int::from(11)).unwrap(), Int::from(4))
}

#[test]
fn invert_mod_2() {
    assert_eq!(invert_mod(&Int::from(20), &Int::from(33)).unwrap(), Int::from(5))
}

#[test]
fn invert_mod_3() {
    assert_eq!(invert_mod(&Int::from(7), &Int::from(1009)).unwrap(), Int::from(865)) 
}

#[test]
fn invert_mod_4() {
    assert_eq!(invert_mod(&Int::from(6), &Int::from(20)), None)
}

    
#[test]
fn crt_1() {

}

#[test]
fn solve_linear_1() {

}

#[test]
fn order_1() {

}

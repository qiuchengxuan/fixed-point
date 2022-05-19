use fixed_point::FixedPoint;
use macros::fixed;

#[test]
fn test_fixed_point() {
    let decimal: FixedPoint<i32, 0> = "0".parse().unwrap();
    assert_eq!("0.0", format!("{decimal}"));
    let decimal: FixedPoint<i32, 1> = "0.0".parse().unwrap();
    assert_eq!("0.0", format!("{decimal}"));
    let decimal: FixedPoint<i32, 1> = "0.1".parse().unwrap();
    assert_eq!("0.1", format!("{decimal}"));
    let decimal: FixedPoint<i32, 2> = "0.01".parse().unwrap();
    assert_eq!("0.01", format!("{decimal}"));
    let decimal: FixedPoint<i32, 2> = "0.11".parse().unwrap();
    assert_eq!("0.11", format!("{decimal}"));
    let decimal: FixedPoint<i32, 2> = "0.1".parse().unwrap();
    assert_eq!("0.1", format!("{decimal}"));
    let decimal: FixedPoint<i32, 2> = "1".parse().unwrap();
    assert_eq!("1.0", format!("{decimal}"));
    let decimal: FixedPoint<i32, 2> = "1.001".parse().unwrap();
    assert_eq!("1.0", format!("{decimal}"));
    let decimal: FixedPoint<i32, 3> = "0.001".parse().unwrap();
    assert_eq!("0.001", format!("{decimal}"));
    let decimal: FixedPoint<i32, 3> = "0.0001".parse().unwrap();
    assert_eq!("0.0", format!("{decimal}"));
    let decimal: FixedPoint<i32, 3> = "-0.1".parse().unwrap();
    assert_eq!("-0.1", format!("{decimal}"));
    let decimal: FixedPoint<i32, 3> = "-1.1".parse().unwrap();
    assert_eq!("-1.1", format!("{decimal}"));
}

#[test]
fn test_fixed_point_macro() {
    let decimal = fixed!(0.0u16, 2);
    assert_eq!("0.0", format!("{decimal}"));
    let decimal = fixed!(0.1u16, 2);
    assert_eq!("0.1", format!("{decimal}"));
    let decimal = fixed!(0.11u16);
    assert_eq!(
        "0.11 2",
        format!("{} {}", decimal, decimal.decimal_length())
    );
    let decimal = fixed!(1.0u16, 2);
    assert_eq!("1.0", format!("{decimal}"));
    let decimal = fixed!(1_1.0_1u16);
    assert_eq!("11.01", format!("{decimal}"));
    let decimal = fixed!(1.10u16);
    assert_eq!("1.1", format!("{decimal}"));
    let decimal = fixed!(-0.1i16, 2);
    assert_eq!("-0.1", format!("{decimal}"));
    let decimal = fixed!(-1.1i16, 2);
    assert_eq!("-1.1", format!("{decimal}"));
}

#[test]
fn test_malformed() {
    assert_eq!(Err(()), "".parse::<FixedPoint<u16, 4>>());
    assert_eq!(Err(()), "1.".parse::<FixedPoint<u16, 4>>());
    assert_eq!(Err(()), ".1".parse::<FixedPoint<u16, 4>>());
    assert_eq!(Err(()), "-1.0".parse::<FixedPoint<u16, 4>>());
    assert_eq!(Err(()), "10.0".parse::<FixedPoint<u16, 4>>());
}

#[test]
fn test_i8() {
    let decimal = fixed!(0.0i8);
    assert_eq!("0.0", format!("{decimal}"));
    let decimal = fixed!(0.0u8);
    assert_eq!("0.0", format!("{decimal}"));
}

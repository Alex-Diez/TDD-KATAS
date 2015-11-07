extern crate calc;

use calc::Calculator;

#[test]
fn test_evaluate_diff_numbers() {
    let calc1 = Calculator::new("1");
    let calc2 = Calculator::new("2");

    assert_eq!(calc1.evaluate(), 1);
    assert_eq!(calc2.evaluate(), 2);
}

#[test]
fn test_evaluate_one_plus_two() {
    let calc = Calculator::new("1+2");

    assert_eq!(calc.evaluate(), 3);
}

#[test]
fn test_evaluate_two_minus_one() {
    let calc = Calculator::new("2-1");

    assert_eq!(calc.evaluate(), 1);
}

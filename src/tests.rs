#![cfg(test)]

use super::*;

/// The tolerance for f64 comparisons
const EPSILON: f64 = 1e-9;

#[test]
fn test_positive_values() {
    // Test with standard positive base and height
    assert_eq!(calculate_triangle_area(10.0, 5.0), 25.0);
}

#[test]
fn test_negative_values() {
    // Test with standard negative values
    // The expected behavior will be returning a negative result
    assert_eq!(calculate_triangle_area(10.0, -5.0), -25.0)
}

#[test]
fn test_decimal_values() {
    // Test with decimal values
    assert_eq!(calculate_triangle_area(1.5, 3.0), 2.25);
}

#[test]
fn test_zero_base() {
    // Test with zero base
    assert_eq!(calculate_triangle_area(0.0, 20.0), 0.0)
}

#[test]
fn test_zero_height() {
    // Test with zero height
    assert_eq!(calculate_triangle_area(20.0, 0.0), 0.0)
}

#[test]
fn test_large_values() {
    // Test with large values
    assert_eq!(calculate_triangle_area(1000.0, 2000.0), 1_000_000.0);
}

#[test]
fn test_small_values() {
    // Test with small values, using a tolerance for floating-points comparison
    // See also: IEEE 754
    let base = 0.1;
    let height = 0.2;
    let expected_area = 0.01;
    let calculated_area = calculate_triangle_area(base, height);

    // Test that the absolute difference between the
    // calculated and expected area is less than EPSILON
    assert!(
        (calculated_area - expected_area).abs() < EPSILON,
        "Assertion failed: calculated_area = {}, expected_area = {}",
            calculated_area,
        expected_area
    );
}

/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = doctests_exercise::div(10,2);  // TODO: finish this test!
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2: 6 / 3 = 2
///
/// ```
/// let result = doctests_exercise::div(6,3);
/// assert_eq!( result,2)
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// let result = doctests_exercise::div(6,0);
/// assert_eq!( result,2)
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// let result = doctests_exercise::div(6,3);
/// assert_eq!( result,2)
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// let result = doctests_exercise::div(6,3);
/// assert_eq!( result,2)
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
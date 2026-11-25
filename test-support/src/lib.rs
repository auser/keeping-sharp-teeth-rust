/// Generate a separate named test for each equality assertion.
///
/// Pass a function or closure before `;` to apply it to every input:
///
/// ```
/// use test_support::test_cases;
///
/// test_cases! {
///     i32::abs;
///     positive: 3 => 3,
///     negative: -3 => 3,
/// }
/// ```
///
/// Use complete expressions for multiple arguments or setup blocks:
///
/// ```
/// use test_support::test_cases;
///
/// test_cases! {
///     larger: std::cmp::max(2, 3) => 3,
///     sorted: {
///         let mut values = vec![3, 1, 2];
///         values.sort();
///         values
///     } => vec![1, 2, 3],
/// }
/// ```
///
/// Each case has its own `#[test]` function, so Cargo can report and filter it
/// by name. An empty invocation adds no tests.
#[macro_export]
macro_rules! test_cases {
    ($function:expr; $($name:ident: $input:expr => $expected:expr),* $(,)?) => {
        $crate::test_cases! {
            $($name: ($function)($input) => $expected),*
        }
    };
    ($($name:ident: $actual:expr => $expected:expr),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                ::std::assert_eq!($actual, $expected);
            }
        )*
    };
}

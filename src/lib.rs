#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use num_traits::{One, Zero};

pub trait Factorial: num_integer::Integer + Clone {
  /// Calculates the factorial of an integer.
  ///
  /// # Returns
  /// An integer, representing n!.
  ///
  /// # Note
  /// `0.factorial() == 1`
  ///
  /// # Examples:
  /// Basic usage:
  /// ```rust
  /// use factorial::Factorial;
  ///
  /// let a = 10;
  /// assert_eq!(a.factorial(), 3_628_800);
  /// ```
  fn factorial(&self) -> Self {
    assert!(*self >= Zero::zero());

    let mut fact: Self = One::one();
    let mut i: Self = One::one();
    while i <= *self {
      fact = fact * i.clone();
      i = i + One::one();
    }
    fact
  }
}

// -----------------------------------------------------------------------------
// Blanket implementations for rust primitives, num-bigint::{BigUint, BigInt}.
// -----------------------------------------------------------------------------

#[cfg(feature = "num-bigint")]
impl Factorial for num_bigint::BigInt {}
#[cfg(feature = "num-bigint")]
impl Factorial for num_bigint::BigUint {}

impl Factorial for i8 {}
impl Factorial for i16 {}
impl Factorial for i32 {}
impl Factorial for i64 {}
impl Factorial for i128 {}
impl Factorial for isize {}
impl Factorial for u8 {}
impl Factorial for u16 {}
impl Factorial for u32 {}
impl Factorial for u64 {}
impl Factorial for u128 {}
impl Factorial for usize {}

#[cfg(test)]
mod tests {
  use super::Factorial;

  #[test]
  #[allow(clippy::needless_range_loop)]
  fn factorial() {
    let facts = vec![1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880, 3_628_800];
    for i in 0..=10 {
      assert_eq!(i.factorial(), facts[i]);
    }
  }

  #[cfg(feature = "num-bigint")]
  #[test]
  fn factorial_num_bigint() {
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    let a = BigUint::from_u32(30).unwrap();
    assert_eq!(
      a.factorial().to_string(),
      "265252859812191058636308480000000"
    );
  }
}

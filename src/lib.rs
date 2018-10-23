//! `EnumUnitary` trait and `enum_unitary!` macro.
//!
//! [Repository](https://github.com/spearman/enum-unitary)
//!
//! The following `num_traits` traits are re-exported in the local crate:
//!
//! ```text
//! use enum_unitary::{Bounded, FromPrimitive, ToPrimitive};
//! ```
//!
//! The following macros are also re-exported because they are used in the
//! `enum_unitary!` macro implementation and do not need to be used directly:
//!
//! ```text
//! macro_attr::{macro_attr};
//! enum_derive::{enum_derive_util, IterVariants, NextVariant, PrevVariant};
//! ```

#![cfg_attr(test, feature(const_fn))]

#[cfg_attr(test, macro_use)]
extern crate enum_derive;
#[cfg_attr(test, macro_use)]
extern crate macro_attr;

extern crate num_traits;

// NOTE: macro documentation is not hidden (Rust 1.27.0)
#[doc(hidden)]
pub use num_traits::{Bounded, FromPrimitive, ToPrimitive};
#[doc(hidden)]
pub use enum_derive::{enum_derive_util, IterVariants, NextVariant, PrevVariant};
#[doc(hidden)]
pub use macro_attr::{macro_attr_impl, macro_attr};

//
//  trait EnumUnitary
//
/// A collection of constraints and methods for unitary enums.
///
/// See the `enum_unitary!` macro for defining instances of this trait.
// TODO: expose more constraints ?
pub trait EnumUnitary : Copy + Clone + Eq + Ord + PartialEq + PartialOrd
  + Send + Sync + std::fmt::Debug
  // NB: as of Rust 1.22, the *last* of the following `Into` constraints
  // seems to be chosen by default and using `.into` for one of the other
  // types requires using disambiguation syntax; we choose `usize` here since
  // it is commonly used as an index type.
  + Into <i64> + Into <u64> + Into <isize> + Into <usize>
  + Bounded + ToPrimitive + FromPrimitive
{
  fn count_variants() -> usize;
  fn iter_variants() -> Box <Iterator <Item=Self>>;
  // TODO: expose more methods from enum_derive ?
}

//
//  enum_unitary!
//
/// Wraps "unitary" enums (i.e. enums where variants do not have payloads) with
/// `enum_derive` derives (`IterVariants`, `NextVariant`, `PrevVariant`) and
/// implements the `EnumUnitary` trait.
///
/// Also defines a `count` const non-trait method that returns the number of
/// variants in the enum.
///
/// Currently the deriving attribute is fixed and can not be overridden, and
/// explicit discriminators are not allowed: enum variants will be numbered
/// starting from `0`.
///
/// # Examples
///
/// ```
/// #![feature(const_fn)]
/// #[macro_use] extern crate enum_unitary;
/// fn main () {
///   enum_unitary! {
///     pub enum E (EVariants) {
///       A, B, C
///     }
///   }
///   use enum_unitary::{EnumUnitary, Bounded};
///   assert_eq!(E::count(), 3);
///   assert_eq!(Into::<usize>::into (E::A), 0);
///   assert_eq!(Into::<usize>::into (E::B), 1);
///   assert_eq!(Into::<usize>::into (E::C), 2);
///   assert_eq!(E::min_value(), E::A);
///   assert_eq!(E::max_value(), E::C);
///   let mut i = E::iter_variants();
///   assert_eq!(i.next(), Some (E::A));
///   assert_eq!(i.next(), Some (E::B));
///   assert_eq!(i.next(), Some (E::C));
///   assert_eq!(i.next(), None);
///   assert_eq!(E::A.next_variant(), Some (E::B));
///   assert_eq!(E::A.prev_variant(), None);
///   assert_eq!(E::B.next_variant(), Some (E::C));
///   assert_eq!(E::B.prev_variant(), Some (E::A));
///   assert_eq!(E::C.next_variant(), None);
///   assert_eq!(E::C.prev_variant(), Some (E::B));
/// }
/// ```

#[macro_export]
macro_rules! enum_unitary {
  //
  //  nullary: private
  //
  (
    $(#[$attrs:meta])*
    enum $enum:ident ($iter:ident) { }
  ) => {

    macro_attr!{
      $(#[$attrs])*
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      enum $enum {
        Void = ::std::isize::MAX
      }
    }

    impl From <$enum> for isize {
      fn from (x : $enum) -> Self {
        x as isize
      }
    }
    impl From <$enum> for usize {
      fn from (x: $enum) -> Self {
        x as usize
      }
    }
    impl From <$enum> for i64 {
      fn from (x : $enum) -> Self {
        x as i64
      }
    }
    impl From <$enum> for u64 {
      fn from (x: $enum) -> Self {
        x as u64
      }
    }

    impl $crate::Bounded for $enum {
      fn min_value() -> Self {
        $enum::Void
      }
      fn max_value() -> Self {
        $enum::Void
      }
    }

    impl $crate::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        match x as isize {
          ::std::isize::MAX => Some ($enum::Void),
          _ => None
        }
      }
      fn from_u64 (x: u64) -> Option <Self> {
        match x as isize {
          ::std::isize::MAX => Some ($enum::Void),
          _ => None
        }
      }
    }

    impl $crate::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl $crate::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        0
      }
    }

  };

  //
  //  nullary: public
  //
  (
    $(#[$attrs:meta])*
    pub enum $enum:ident ($iter:ident) { }
  ) => {

    macro_attr!{
      $(#[$attrs])*
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      pub enum $enum {
        Void = ::std::isize::MAX
      }
    }

    impl From <$enum> for isize {
      fn from (x : $enum) -> Self {
        x as isize
      }
    }
    impl From <$enum> for usize {
      fn from (x: $enum) -> Self {
        x as usize
      }
    }
    impl From <$enum> for i64 {
      fn from (x : $enum) -> Self {
        x as i64
      }
    }
    impl From <$enum> for u64 {
      fn from (x: $enum) -> Self {
        x as u64
      }
    }

    impl $crate::Bounded for $enum {
      fn min_value() -> Self {
        $enum::Void
      }
      fn max_value() -> Self {
        $enum::Void
      }
    }

    impl $crate::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        match x as isize {
          ::std::isize::MAX => Some ($enum::Void),
          _ => None
        }
      }
      fn from_u64 (x: u64) -> Option <Self> {
        match x as isize {
          ::std::isize::MAX => Some ($enum::Void),
          _ => None
        }
      }
    }

    impl $crate::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl $crate::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        0
      }
    }

  };

  //
  //  singleton: private
  //
  (
    $(#[$attrs:meta])*
    enum $enum:ident ($iter:ident) { $singleton:ident$(,)* }
  ) => {

    macro_attr!{
      $(#[$attrs])*
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      enum $enum {
        $singleton=0
      }
    }

    impl From <$enum> for isize {
      fn from (x : $enum) -> Self {
        x as isize
      }
    }
    impl From <$enum> for usize {
      fn from (x: $enum) -> Self {
        x as usize
      }
    }
    impl From <$enum> for i64 {
      fn from (x : $enum) -> Self {
        x as i64
      }
    }
    impl From <$enum> for u64 {
      fn from (x: $enum) -> Self {
        x as u64
      }
    }

    impl $crate::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$singleton
      }
      fn max_value() -> Self {
        $enum::$singleton
      }
    }

    impl $crate::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
      fn from_u64 (x: u64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
    }

    impl $crate::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl $crate::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        1
      }
    }

  };

  //
  //  singleton: public
  //
  (
    $(#[$attrs:meta])*
    pub enum $enum:ident ($iter:ident) { $singleton:ident$(,)* }
  ) => {

    macro_attr!{
      $(#[$attrs])*
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      pub enum $enum {
        $singleton=0
      }
    }

    impl From <$enum> for isize {
      fn from (x : $enum) -> Self {
        x as isize
      }
    }
    impl From <$enum> for usize {
      fn from (x: $enum) -> Self {
        x as usize
      }
    }
    impl From <$enum> for i64 {
      fn from (x : $enum) -> Self {
        x as i64
      }
    }
    impl From <$enum> for u64 {
      fn from (x: $enum) -> Self {
        x as u64
      }
    }

    impl $crate::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$singleton
      }
      fn max_value() -> Self {
        $enum::$singleton
      }
    }

    impl $crate::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
      fn from_u64 (x: u64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
    }

    impl $crate::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl $crate::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        1
      }
    }

  };

  //
  //  2 or more variants: private
  //
  (
    $(#[$attrs:meta])*
    enum $enum:ident ($iter:ident) { $first:ident$(, $variant:ident$(,)*)+ }
  ) => {
    enum_unitary!{
      $(#[$attrs])*
      enum $enum ($iter) {$first} {$($variant),+}
    }
  };

  (
    $(#[$attrs:meta])*
    enum $enum:ident ($iter:ident)
      {$($variant:ident),+} {$more:ident$(, $tail:ident)+}
  ) => {
    enum_unitary!{
      $(#[$attrs])*
      enum $enum ($iter) {$($variant,)+ $more} {$($tail),+}
    }
  };

  (
    $(#[$attrs:meta])*
    enum $enum:ident ($iter:ident)
      {$min:ident$(, $variant:ident)*} {$max:ident}
  ) => {

    macro_attr!{
      $(#[$attrs])*
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      enum $enum {
        $min=0$(, $variant)*, $max
      }
    }

    impl From <$enum> for isize {
      fn from (x : $enum) -> Self {
        x as isize
      }
    }
    impl From <$enum> for usize {
      fn from (x: $enum) -> Self {
        x as usize
      }
    }
    impl From <$enum> for i64 {
      fn from (x : $enum) -> Self {
        x as i64
      }
    }
    impl From <$enum> for u64 {
      fn from (x: $enum) -> Self {
        x as u64
      }
    }

    impl $crate::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$min
      }
      fn max_value() -> Self {
        $enum::$max
      }
    }

    impl $crate::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
      fn from_u64 (x: u64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
    }

    impl $crate::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl $crate::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <(Iterator <Item=Self>)> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        $enum::$max as usize + 1
      }
    }

  };

  //
  //  2 or more variants: public
  //
  (
    $(#[$attrs:meta])*
    pub enum $enum:ident ($iter:ident) { $first:ident$(, $variant:ident$(,)*)+ }
  ) => {
    enum_unitary!{
      $(#[$attrs])*
      pub enum $enum ($iter) {$first} {$($variant),+}
    }
  };

  (
    $(#[$attrs:meta])*
    pub enum $enum:ident ($iter:ident)
      {$($variant:ident),+} {$more:ident$(, $tail:ident)+}
  ) => {
    enum_unitary!{
      $(#[$attrs])*
      pub enum $enum ($iter) {$($variant,)+ $more} {$($tail),+}
    }
  };

  (
    $(#[$attrs:meta])*
    pub enum $enum:ident ($iter:ident)
      {$min:ident$(, $variant:ident)*} {$max:ident}
  ) => {

    macro_attr!{
      $(#[$attrs])*
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      pub enum $enum {
        $min=0$(, $variant)*, $max
      }
    }

    impl From <$enum> for isize {
      fn from (x : $enum) -> Self {
        x as isize
      }
    }
    impl From <$enum> for usize {
      fn from (x: $enum) -> Self {
        x as usize
      }
    }
    impl From <$enum> for i64 {
      fn from (x : $enum) -> Self {
        x as i64
      }
    }
    impl From <$enum> for u64 {
      fn from (x: $enum) -> Self {
        x as u64
      }
    }

    impl $crate::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$min
      }
      fn max_value() -> Self {
        $enum::$max
      }
    }

    impl $crate::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
      fn from_u64 (x: u64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
    }

    impl $crate::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl $crate::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        $enum::$max as usize + 1
      }
    }

  };

}

//
//  mod tests
//
#[cfg(test)]
mod tests {
  use {std};

  #[test]
  fn test_unit() {
    use {EnumUnitary, Bounded, FromPrimitive,ToPrimitive};

    // private enum
    enum_unitary!{
      enum Myenum1 (Myenum1Variants) {
        A, B, C
      }
    }
    assert_eq!(Myenum1::count(), 3);
    assert_eq!(Myenum1::count_variants(), 3);
    assert_eq!(Into::<usize>::into (Myenum1::A), 0);
    assert_eq!(Into::<usize>::into (Myenum1::B), 1);
    assert_eq!(Into::<usize>::into (Myenum1::C), 2);
    assert_eq!(Some (Myenum1::A), Myenum1::from_usize (0));
    assert_eq!(Some (Myenum1::B), Myenum1::from_usize (1));
    assert_eq!(Some (Myenum1::C), Myenum1::from_usize (2));
    assert_eq!(None, Myenum1::from_usize (3));
    assert_eq!(Some (0), Myenum1::A.to_usize());
    assert_eq!(Some (1), Myenum1::B.to_usize());
    assert_eq!(Some (2), Myenum1::C.to_usize());
    assert_eq!(Myenum1::min_value(), Myenum1::A);
    assert_eq!(Myenum1::max_value(), Myenum1::C);
    let mut i = Myenum1::iter_variants();
    assert_eq!(i.next(), Some (Myenum1::A));
    assert_eq!(i.next(), Some (Myenum1::B));
    assert_eq!(i.next(), Some (Myenum1::C));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum1::A.next_variant(), Some (Myenum1::B));
    assert_eq!(Myenum1::A.prev_variant(), None);
    assert_eq!(Myenum1::B.next_variant(), Some (Myenum1::C));
    assert_eq!(Myenum1::B.prev_variant(), Some (Myenum1::A));
    assert_eq!(Myenum1::C.next_variant(), None);
    assert_eq!(Myenum1::C.prev_variant(), Some (Myenum1::B));

    // public enum
    enum_unitary!{
      pub enum Myenum2 (Myenum2Variants) {
        A, B, C
      }
    }
    assert_eq!(Myenum2::count(), 3);
    assert_eq!(Myenum2::count_variants(), 3);
    assert_eq!(Into::<usize>::into (Myenum2::A), 0);
    assert_eq!(Into::<usize>::into (Myenum2::B), 1);
    assert_eq!(Into::<usize>::into (Myenum2::C), 2);
    assert_eq!(Some (Myenum2::A), Myenum2::from_usize (0));
    assert_eq!(Some (Myenum2::B), Myenum2::from_usize (1));
    assert_eq!(Some (Myenum2::C), Myenum2::from_usize (2));
    assert_eq!(None, Myenum2::from_usize (3));
    assert_eq!(Some (0), Myenum2::A.to_usize());
    assert_eq!(Some (1), Myenum2::B.to_usize());
    assert_eq!(Some (2), Myenum2::C.to_usize());
    assert_eq!(Myenum2::min_value(), Myenum2::A);
    assert_eq!(Myenum2::max_value(), Myenum2::C);
    let mut i = Myenum2::iter_variants();
    assert_eq!(i.next(), Some (Myenum2::A));
    assert_eq!(i.next(), Some (Myenum2::B));
    assert_eq!(i.next(), Some (Myenum2::C));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum2::A.next_variant(), Some (Myenum2::B));
    assert_eq!(Myenum2::A.prev_variant(), None);
    assert_eq!(Myenum2::B.next_variant(), Some (Myenum2::C));
    assert_eq!(Myenum2::B.prev_variant(), Some (Myenum2::A));
    assert_eq!(Myenum2::C.next_variant(), None);
    assert_eq!(Myenum2::C.prev_variant(), Some (Myenum2::B));

    // private singleton enum
    enum_unitary!{
      enum Myenum3 (Myenum3Variants) {
        X
      }
    }
    assert_eq!(Myenum3::count(), 1);
    assert_eq!(Myenum3::count_variants(), 1);
    assert_eq!(Into::<usize>::into (Myenum3::X), 0);
    assert_eq!(Some (Myenum3::X), Myenum3::from_usize (0));
    assert_eq!(None, Myenum3::from_usize (1));
    assert_eq!(Some (0), Myenum3::X.to_usize());
    assert_eq!(Myenum3::min_value(), Myenum3::X);
    assert_eq!(Myenum3::max_value(), Myenum3::X);
    let mut i = Myenum3::iter_variants();
    assert_eq!(i.next(), Some (Myenum3::X));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum3::X.next_variant(), None);
    assert_eq!(Myenum3::X.prev_variant(), None);

    // public singleton enum
    enum_unitary!{
      pub enum Myenum4 (Myenum4Variants) {
        X
      }
    }
    assert_eq!(Myenum4::count(), 1);
    assert_eq!(Myenum4::count_variants(), 1);
    assert_eq!(Into::<usize>::into (Myenum4::X), 0);
    assert_eq!(Some (Myenum4::X), Myenum4::from_usize (0));
    assert_eq!(None, Myenum4::from_usize (1));
    assert_eq!(Some (0), Myenum4::X.to_usize());
    assert_eq!(Myenum4::min_value(), Myenum4::X);
    assert_eq!(Myenum4::max_value(), Myenum4::X);
    let mut i = Myenum4::iter_variants();
    assert_eq!(i.next(), Some (Myenum4::X));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum4::X.next_variant(), None);
    assert_eq!(Myenum4::X.prev_variant(), None);

    // private nullary enum
    enum_unitary!{
      enum Myenum5 (Myenum5Variants) { }
    }
    assert_eq!(Myenum5::count(), 0);
    assert_eq!(Myenum5::count_variants(), 0);
    assert_eq!(Myenum5::Void as isize, std::isize::MAX);
    assert_eq!(Some (Myenum5::Void),
      Myenum5::from_usize (std::isize::MAX as usize));
    assert_eq!(None, Myenum5::from_usize (0));
    assert_eq!(Some (std::isize::MAX as usize), Myenum5::Void.to_usize());
    assert_eq!(Myenum5::min_value(), Myenum5::Void);
    assert_eq!(Myenum5::max_value(), Myenum5::Void);
    let mut i = Myenum5::iter_variants();
    assert_eq!(i.next(), Some (Myenum5::Void));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum5::Void.next_variant(), None);
    assert_eq!(Myenum5::Void.prev_variant(), None);

    // public nullary enum
    enum_unitary!{
      pub enum Myenum6 (Myenum6Variants) { }
    }
    assert_eq!(Myenum6::count(), 0);
    assert_eq!(Myenum6::count_variants(), 0);
    assert_eq!(Myenum6::Void as isize, std::isize::MAX);
    assert_eq!(Some (Myenum6::Void),
      Myenum6::from_usize (std::isize::MAX as usize));
    assert_eq!(None, Myenum6::from_usize (0));
    assert_eq!(Some (std::isize::MAX as usize), Myenum6::Void.to_usize());
    assert_eq!(Myenum6::min_value(), Myenum6::Void);
    assert_eq!(Myenum6::max_value(), Myenum6::Void);
    let mut i = Myenum6::iter_variants();
    assert_eq!(i.next(), Some (Myenum6::Void));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum6::Void.next_variant(), None);
    assert_eq!(Myenum6::Void.prev_variant(), None);
  }
} // end mod tests

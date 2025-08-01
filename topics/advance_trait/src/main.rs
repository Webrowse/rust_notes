//! Advanced traits and generics notes with examples.
//! To run examples, use `cargo test` or `rustc --test advanced_traits_and_generics.rs && ./advanced_traits_and_generics`.
//! Concepts covered:
//! 1. Generic functions and bounds
//! 2. Trait definitions, associated types, and default implementations
//! 3. Trait objects and dynamic dispatch
//! 4. Higher-ranked trait bounds (HRTB)
//! 5. PhantomData for zero-sized type tracking
//! 6. Const generics
//! 7. Generic associated types (GAT) (requires Rust 1.65+ stable support for simple use cases)
//! 8. Where clauses and complex bounds
//! 9. Newtype pattern and opaque types
//! 10. Marker traits and sealed traits pattern
//! 11. Specialization note (nightly only, shown as commented)
//! 12. Lifetime generic interplay

use std::marker::PhantomData;
use std::fmt::Debug;

/// 1. Generic function with trait bounds.
/// Returns the maximum of two items that implement PartialOrd + Copy.
fn max_generic<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

/// 2. Trait with associated type and default implementation.
trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn count(&self) -> usize;

    // default method: requires Item: Debug for the default to work
    fn debug_contents(&self)
    where
        Self::Item: Debug,
    {
        println!("Container has {} items", self.count());
    }
}

/// Simple Vec-like container implementing Container.
struct MyBoxVec<T> {
    inner: Vec<T>,
}

impl<T> Container for MyBoxVec<T> {
    type Item = T;

    fn add(&mut self, item: T) {
        self.inner.push(item);
    }

    fn count(&self) -> usize {
        self.inner.len()
    }
}

/// 3. Trait object example: dynamic dispatch.
/// Trait for formatting.
trait Formatter {
    fn format(&self) -> String;
}

struct JsonFormatter;
struct PlainFormatter;

impl Formatter for JsonFormatter {
    fn format(&self) -> String {
        "{\"status\":\"ok\"}".to_string()
    }
}

impl Formatter for PlainFormatter {
    fn format(&self) -> String {
        "status: ok".to_string()
    }
}

/// Accept any formatter via trait object.
fn apply_format(f: &dyn Formatter) -> String {
    f.format()
}

/// 4. Higher-ranked trait bound example.
/// Function that accepts any reference-producing function that works for all lifetimes.
fn call_with_ref<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = "hello";
    // The closure must work for any lifetime so we can pass s safely.
    let result = f(s);
    result.len()
}

/// 5. PhantomData usage to associate type without storing it.
/// This example tracks ownership of some logical type without runtime cost.
struct TypedId<T> {
    id: u64,
    _marker: PhantomData<T>,
}

impl<T> TypedId<T> {
    fn new(id: u64) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

/// 6. Const generics: array wrapper with compile-time size.
struct FixedBuffer<T, const N: usize> {
    data: [T; N],
}

impl<T: Copy + Default, const N: usize> FixedBuffer<T, N> {
    fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }

    fn len(&self) -> usize {
        N
    }
}

/// 7. Generic Associated Types (GAT) example.
/// Simple iterator trait with per-lifetime associated type.
trait MyIterator {
    type Item<'a>
    where
        Self: 'a;

    fn get<'a>(&'a self) -> Self::Item<'a>;
}

// Example implementing a reference wrapper.
struct Holder(String);

impl MyIterator for Holder {
    type Item<'a> = &'a str;

    fn get<'a>(&'a self) -> Self::Item<'a> {
        &self.0
    }
}

/// 8. Complex bounds via where clause.
/// Function that requires iterator over items that implement Debug and Clone.
fn process_items<I, T>(iter: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
    T: Debug + Clone,
{
    let mut v = vec![];
    for item in iter {
        // we can clone because of bound
        v.push(item.clone());
        // we can debug-print
        println!("processing: {:?}", item);
    }
    v
}

/// 9. Newtype for opaque wrapping and implementing traits separately.
struct Meters(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

impl Debug for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m", self.0)
    }
}

/// 10. Sealed trait pattern to prevent external implementation.
mod sealed {
    pub trait Sealed {}
}

pub trait MySealedTrait: sealed::Sealed {
    fn secret(&self) -> &'static str;
}

struct Internal;

impl sealed::Sealed for Internal {}

impl MySealedTrait for Internal {
    fn secret(&self) -> &'static str {
        "only internal can implement"
    }
}

/// 11. Specialization (nightly, unstable). Shown as comment because it is not stable.
/// #![feature(specialization)]
/// trait Speak {
///     fn speak(&self) -> &'static str {
///         "generic sound"
///     }
/// }
/// default impl for all types
/// impl<T> Speak for T {}
/// // specialized for u8
/// impl Speak for u8 {
///     fn speak(&self) -> &'static str {
///         "byte sound"
///     }
/// }

/// 12. Lifetime and generic interplay.
/// Struct holding a reference with generic lifetime and type.
struct RefHolder<'a, T: 'a> {
    reference: &'a T,
}

impl<'a, T> RefHolder<'a, T> {
    fn new(reference: &'a T) -> Self {
        RefHolder { reference }
    }

    fn get(&self) -> &T {
        self.reference
    }
}

// Unit tests / examples
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_generic() {
        assert_eq!(max_generic(3, 5), 5);
        assert_eq!(max_generic(10.0, 2.5), 10.0);
    }

    #[test]
    fn test_container_and_debug() {
        let mut c = MyBoxVec { inner: vec![] };
        c.add(1);
        c.add(2);
        assert_eq!(c.count(), 2);
        // debug_contents requires Debug bound; should not panic
        c.debug_contents();
    }

    #[test]
    fn test_formatter_dispatch() {
        let json = JsonFormatter;
        let plain = PlainFormatter;
        assert_eq!(apply_format(&json), "{\"status\":\"ok\"}");
        assert_eq!(apply_format(&plain), "status: ok");
    }

    #[test]
    fn test_hrtb() {
        // closure that returns its input works for any lifetime
        let len = call_with_ref(|s| s);
        assert_eq!(len, 5); // "hello" length
    }

    #[test]
    fn test_phantom_typed_id() {
        struct User;
        let id: TypedId<User> = TypedId::new(42);
        // Type is tracked only at compile time, no runtime difference
        assert_eq!(id.id, 42);
    }

    #[test]
    fn test_const_generic_buffer() {
        let buf: FixedBuffer<u8, 8> = FixedBuffer::new();
        assert_eq!(buf.len(), 8);
    }

    #[test]
    fn test_gat_holder() {
        let h = Holder("rust".to_string());
        let s: &str = h.get();
        assert_eq!(s, "rust");
    }

    #[test]
    fn test_process_items() {
        let source = vec![1, 2, 3];
        let processed = process_items(source);
        assert_eq!(processed, vec![1, 2, 3]);
    }

    #[test]
    fn test_newtype_debug() {
        let m = Meters::new(12.5);
        let s = format!("{:?}", m);
        assert!(s.contains("12.5"));
        assert!(s.contains("m"));
    }

    #[test]
    fn test_sealed_trait() {
        let i = Internal;
        assert_eq!(i.secret(), "only internal can implement");
    }

    #[test]
    fn test_ref_holder() {
        let x = 100;
        let holder = RefHolder::new(&x);
        assert_eq!(*holder.get(), 100);
    }
}

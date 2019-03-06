//! # ArcGuard
//!
//! A Guard around `Arc<Mutex<T>>` allowing you to write less boilerplate code.
//!
//! # Example
//!
//! Before:
//! ```
//! use std::sync::{Arc, Mutex};
//!
//! let indicator = Arc::new(Mutex::new(Indicator::new()));
//! let indicator_clone = indicator.clone();
//! let indicator_clone = indicator_clone.lock().expect("Unable to lock indicator.");
//!
//! indicator_clone.do_something();
//!
//! drop(indicator_clone);
//! ```
//!
//! After:
//!
//! ```
//! use arc_guard::ArcGuard;
//!
//! let indicator = ArcGuard::new(Indicator::new());
//!
//! indicator.execute(|indicator| {
//!     let indicator = indicator.lock().expect("Unable to lock indicator.");
//!     indicator.do_something();
//! });
//! ```
//!

use std::sync::{Arc, Mutex};

pub struct ArcGuard<T> {
    arc: Arc<Mutex<T>>,
}

impl<T> ArcGuard<T> {
    /// Constructs a new `ArcGuard<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use arc_guard::ArcGuard;
    ///
    /// let indicator = ArcGuard::new(Indicator::new());
    /// ```
    pub fn new(t: T) -> Self {
        ArcGuard{arc: Arc::new(Mutex::new(t))}
    }

    /// Executes a closure passed as an argument.
    ///
    /// This is exactly what helps us avoid the boilerplate code,
    /// `execute` passes an `Arc<Mutex<T>>` clone and when the Closure finishes,
    /// the clone is automatically dropped.
    ///
    /// # Example
    ///
    /// ```
    /// use arc_guard::ArcGuard;
    ///
    /// let indicator = ArcGuard::new(Indicator::new());
    ///
    /// indicator.execute(|indicator| {
    ///     let indicator = indicator.lock().expect("Unable to lock indicator.");
    ///     indicator.do_something();
    /// });
    /// ```
    ///
    /// `execute` takes the return type of the Closure as its own,
    /// so you are able to return from your closure into a variable.
    ///
    /// # Example
    ///
    /// ```
    /// use arc_guard::ArcGuard;
    ///
    /// let indicator = ArcGuard::new(Indicator::new());
    ///
    /// let some_string: String = indicator.execute(|indicator| -> String {
    ///     let indicator = indicator.lock().expect("Unable to lock indicator.");
    ///     return indicator.something();
    /// });
    /// ```
    pub fn execute<R>(&self, mut callback: impl FnMut(Arc<Mutex<T>>) -> R) -> R {
        callback(self.arc.clone())
    }

    /// In some cases it is convenient to use `Arc<Mutex<T>>`, instead of `ArcGuard<T>`.
    ///
    /// With this method you are able to get a clone of the inner `Arc<Mutex<T>>`.
    ///
    /// # Example
    ///
    /// ```
    /// use arc_guard::ArcGuard;
    ///
    /// let indicator = ArcGuard::new(Indicator::new());
    ///
    /// let inner_arc = indicator.arc();
    /// ```
    pub fn arc(&self) -> Arc<Mutex<T>> {
        self.arc.clone()
    }

    /// Returns new `ArcGuard` with a clone of the inner `Arc<Mutex<T>>`.
    ///
    /// # Example
    ///
    /// ```
    /// use arc_guard::ArcGuard;
    ///
    /// let indicator = ArcGuard::new(Indicator::new());
    ///
    /// let indicator_clone = indicator.clone();
    /// ```
    pub fn clone(&self) -> Self {
        ArcGuard{arc: self.arc.clone()}
    }
}


#[cfg(test)]
mod tests {
    use super::ArcGuard;
    struct Indicator;

    impl Indicator {
        pub fn new() -> Self {Indicator}
    }

    #[test]
    fn it_works() {
        let indicator = ArcGuard::new(Indicator::new());

        let string = indicator.execute(|indicator| -> String {
            String::from("5")
        });

        assert_eq!(string, "5");
    }
}

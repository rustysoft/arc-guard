ArcGuard
========

Guard around `Arc<Mutex<T>>` allowing you to write less boilerplate code.

Full Documentation can be read [here](https://docs.rs/arc-guard/latest/arc_guard/).

Example
=======

Before:
```rust
use std::sync::{Arc, Mutex};

let indicator = Arc::new(Mutex::new(Indicator::new()));
let indicator_clone = indicator.clone();
let indicator_clone = indicator_clone.lock().expect("Unable to lock indicator.");

indicator_clone.do_something();

drop(indicator_clone);
```

After:

```rust
use arc_guard::ArcGuard;

let indicator = ArcGuard::new(Indicator::new());

indicator.execute(|indicator| {
    let indicator = indicator.lock().expect("Unable to lock indicator.");
    indicator.do_something();
});
```

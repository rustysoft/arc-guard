use std::sync::{Arc, Mutex};

pub struct ArcGuard<T> {
    arc: Arc<Mutex<T>>,
}

impl<T> ArcGuard<T> {
    pub fn new(t: T) -> Self {
        ArcGuard{arc: Arc::new(Mutex::new(t))}
    }

    pub fn execute<R>(&self, mut callback: impl FnMut(Arc<Mutex<T>>) -> R) -> R {
        callback(self.arc.clone())
    }

    pub fn arc(&self) -> Arc<Mutex<T>> {
        self.arc.clone()
    }

    pub fn clone(&self) -> Self {
        ArcGuard{arc: self.arc.clone()}
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

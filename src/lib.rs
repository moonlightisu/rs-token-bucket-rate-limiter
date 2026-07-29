//! Token-bucket rate limiter. Not thread-safe; wrap in a Mutex if shared.

pub struct Bucket<F: Fn() -> f64> {
    rate: f64,
    capacity: f64,
    tokens: f64,
    last: f64,
    now: F,
}

impl<F: Fn() -> f64> Bucket<F> {
    pub fn new(rate_per_sec: f64, capacity: f64, now: F) -> Self {
        let last = now();
        Bucket { rate: rate_per_sec, capacity, tokens: capacity, last, now }
    }

    /// Refill by elapsed time, then take 'cost' tokens if available.
    pub fn allow(&mut self, cost: f64) -> bool {
        let t = (self.now)();
        self.tokens = (self.tokens + (t - self.last) * self.rate).min(self.capacity);
        self.last = t;
        if self.tokens >= cost {
            self.tokens -= cost;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    #[test]
    fn refills_over_time() {
        let clock = Cell::new(0.0);
        let mut b = Bucket::new(1.0, 2.0, || clock.get());
        assert!(b.allow(2.0));
        assert!(!b.allow(1.0));
        clock.set(1.0);
        assert!(b.allow(1.0));
    }
}

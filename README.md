# In-memory token-bucket rate limiter

```
src/lib.rs
```
Run `cargo test` to see the bucket in action. Tests live next to the source. Check them for concrete usage.

This is a time-injectable token bucket. It operates strictly in memory. No Redis. No external state stores.

The crate uses only the Rust standard library. Zero extra dependencies. Zero background services to provision. The real gotcha is process boundaries. State lives in RAM, so it only tracks limits for a single local process.
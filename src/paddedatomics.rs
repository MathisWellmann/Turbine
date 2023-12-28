use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

//------------------------- Padded 64 -------------------------//

/// AtomicInt padded with 64 bytes
pub struct Padded64 {
    _p: [u64; 7],
    counter: AtomicU64,
}

impl Padded64 {
    pub fn new(x: u64) -> Padded64 {
        Padded64 {
            _p: [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64],
            counter: AtomicU64::new(x),
        }
    }
    #[inline]
    pub fn add(&self, x: u64) -> u64 {
        self.counter.fetch_add(x, SeqCst)
    }

    #[inline]
    pub fn load(&self) -> u64 {
        self.counter.load(SeqCst)
    }

    #[inline]
    pub fn store(&self, x: u64) {
        self.counter.store(x, SeqCst);
    }

    #[inline]
    pub fn reset(&self) {
        self.store(0);
    }

    #[inline]
    pub fn or(&self, x: u64) -> u64 {
        self.counter.fetch_or(x, SeqCst)
    }

    #[inline]
    pub fn and(&self, x: u64) -> u64 {
        self.counter.fetch_and(x, SeqCst)
    }
}

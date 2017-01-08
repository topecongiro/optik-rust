use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::{Acquire,Relaxed, Release};
use super::{OPTIKLock, Version};

const LOCKED: Version = 1;

pub struct Mutex {
    body: AtomicU64,
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex {
            body: AtomicU64::new(0),
        }
    }
}

impl OPTIKLock for Mutex {
    fn try_lock_version(&mut self, tv: Version) -> bool {
        if self.is_locked(tv) || self.body.load(Relaxed) != tv {
            false
        } else {
            self.body.compare_and_swap(tv, tv+1, Relaxed) == tv
        }
    }

    fn lock_version(&mut self, tv: Version) -> bool {
        let mut cur = self.body.load(Relaxed);
        while self.body.compare_and_swap(cur, cur+1, Relaxed) != cur {
            while self.is_locked(cur) {
                cur = self.body.load(Relaxed)
            }
        }
        cur == tv
    }

    fn unlock(&mut self) {
        self.body.fetch_add(1, Release);
    }

    fn revert(&mut self) {
        self.body.fetch_sub(!1, Release);
    }

    fn get_version(&self) -> Version {
        self.body.load(Acquire)
    }

    fn get_version_wait(&self) -> Version {
        loop {
            let oldv = self.body.load(Acquire);
            if !self.is_locked(oldv) {
                return oldv;
            }
        }
    }

    fn is_locked(&self, v: Version) -> bool {
        v & LOCKED == LOCKED
    }
}

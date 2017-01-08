#![feature(integer_atomics)]
pub mod version;
pub mod spin;

pub type Version = u64;

trait OPTIKLock {
    fn try_lock_version(&mut self, tv: Version) -> bool;
    fn lock_version(&mut self, tv: Version) -> bool;
    fn unlock(&mut self);
    fn revert(&mut self);
    fn get_version(&self) -> Version;
    fn get_version_wait(&self) -> Version;
    fn is_locked(&self, v: Version) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

use ahash::AHashMap;
use lazy_static::lazy_static;
use smallvec::SmallVec;
use std::sync::Mutex;

pub const LIMIT: usize = 2001;

lazy_static! {
    pub static ref SEQUENCE_CACHE: Mutex<AHashMap<i64, SmallVec<[i64; LIMIT]>>> =
        Mutex::new(AHashMap::new());
}

#[derive(Clone, Debug, PartialEq)]
pub enum Errors {
    NoFile,
    InvalidInput,
}

pub fn get_secrets(number: i64) -> SmallVec<[i64; LIMIT]> {
    {
        let cache = SEQUENCE_CACHE.lock().unwrap();
        if let Some(cached) = cache.get(&number) {
            return cached.clone();
        }
    }

    let mut result = SmallVec::with_capacity(LIMIT);
    let mut tmp_num = number;
    for _ in 0..LIMIT {
        result.push(tmp_num);
        tmp_num = build_secret(tmp_num);
    }

    let mut cache = SEQUENCE_CACHE.lock().unwrap();
    cache.insert(number, result.clone());
    result
}

fn build_secret(number: i64) -> i64 {
    let mut n = number;
    n = (n ^ (n << 6)) % 0x1000000;
    n = (n ^ (n >> 5)) % 0x1000000;
    n = (n ^ (n << 11)) % 0x1000000;
    n
}

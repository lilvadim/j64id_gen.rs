use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait IdGen<T> {
    fn next(&mut self) -> T;
}

pub struct J64IdGen {
    rand_seed_cnt: u16,
    timestamp_cnt: u64,
}

impl IdGen<u64> for J64IdGen {
    fn next(&mut self) -> u64 {
        self.rand_seed_cnt = self.rand_seed_cnt.overflowing_add(1).0;
        self.timestamp_cnt = u64::max(unix_timestamp(), self.timestamp_cnt + 1);

        let random = rand::rng().random::<u16>();
        self.timestamp_cnt << 24
            | (((self.rand_seed_cnt & 0x0FFF_u16) as u64) << 12)
            | ((random & 0x0FFF_u16) as u64)
    }
}

impl J64IdGen {
    pub fn new() -> Self {
        Self {
            rand_seed_cnt: rand::rng().random::<u16>(),
            timestamp_cnt: unix_timestamp(),
        }
    }
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        thread,
    };

    use super::*;

    #[test]
    fn test_regular() {
        let mut id_gen = J64IdGen::new();
        assert_all_unique(&mut id_gen);
    }

    #[test]
    fn test_threaded() {
        let id_gen = Arc::new(Mutex::new(J64IdGen::new()));
        thread::scope(move |s| {
            for _ in 0..10 {
                let id_gen_clone = id_gen.clone();
                s.spawn(move || {
                    assert_all_unique_thread(id_gen_clone);
                });
            }
        });
    }

    fn assert_all_unique_thread(id_gen: Arc<Mutex<impl IdGen<u64>>>) {
        let id_vec: Vec<u64> = (0..10_000_000)
            .map(|_| id_gen.lock().unwrap().next())
            .collect();
        let mut id_cnt = HashMap::<u64, u64>::new();
        for id in id_vec {
            *id_cnt.entry(id).or_default() += 1;
        }
        assert!(id_cnt.values().all(|cnt| *cnt == 1))
    }

    fn assert_all_unique(id_gen: &mut impl IdGen<u64>) {
        let id_vec: Vec<u64> = (0..10_000_000).map(|_| id_gen.next()).collect();
        let mut id_cnt = HashMap::<u64, u64>::new();
        for id in id_vec {
            *id_cnt.entry(id).or_default() += 1;
        }
        assert!(id_cnt.values().all(|cnt| *cnt == 1))
    }
}

use std::{
    cell::RefCell,
    sync::atomic::{AtomicU8, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use crate::data::snowflake::Snowflake;

const LOCAL_EPOCH: u64 = 1_700_000_000;
const SEQUENCE_BITS: u8 = 8;
const WORKER_BITS: u8 = 5;
const CLUSTER_BITS: u8 = 5;

const MAX_SEQUENCE: u16 = (1 << SEQUENCE_BITS) - 1; // 255
const MAX_WORKER: u8 = (1 << WORKER_BITS) - 1;      // 31
const MAX_CLUSTER: u8 = (1 << CLUSTER_BITS) - 1;    // 31

static THREAD_COUNTER: AtomicU8 = AtomicU8::new(0);

fn next_worker_id() -> u8 {
    let id = THREAD_COUNTER.fetch_add(1, Ordering::Relaxed);
    id & MAX_WORKER
}

thread_local! {
    static WORKER: RefCell<SnowflakeWorker> =
        RefCell::new(SnowflakeWorker::from_thread().expect("Init worker"));
}

pub struct SnowflakeWorker {
    cluster_id: u8,
    worker_id: u8,
    sequence: u16,
    last_timestamp: u64,
}

impl SnowflakeWorker {
    pub fn from_thread() -> Result<Self, String> {
        let cluster_id: u8 = std::env::var("CLUSTER_ID")
            .map_err(|_| "Missing CLUSTER_ID".to_string())?
            .parse()
            .map_err(|_| "Invalid CLUSTER_ID".to_string())?;

        if cluster_id > MAX_CLUSTER {
            return Err(format!("CLUSTER_ID {} exceeds max {}", cluster_id, MAX_CLUSTER));
        }

        Ok(Self {
            cluster_id,
            worker_id: next_worker_id(),
            sequence: 0,
            last_timestamp: 0,
        })
    }

    pub fn generate(&mut self) -> Snowflake {
        let mut timestamp = current_millis() - LOCAL_EPOCH;
        if timestamp < self.last_timestamp {
            timestamp = self.wait_until(self.last_timestamp);
        }

        if timestamp == self.last_timestamp {
            if self.sequence as u16 >= MAX_SEQUENCE {
                timestamp = self.wait_until(self.last_timestamp + 1);
                self.sequence = 0;
                self.last_timestamp = timestamp;
            } else {
                self.sequence += 1;
            }
        } else {
            self.sequence = 0;
            self.last_timestamp = timestamp;
        }

        pack_snowflake(timestamp, self.cluster_id, self.worker_id, self.sequence as u8)
    }

    fn wait_until(&self, target: u64) -> u64 {
        let mut ts = current_millis() - LOCAL_EPOCH;
        while ts < target {
            std::hint::spin_loop();
            ts = current_millis() - LOCAL_EPOCH;
        }
        ts
    }
}

fn current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

fn pack_snowflake(timestamp: u64, cluster: u8, worker: u8, sequence: u8) -> Snowflake {
    let mut id = 0u64;
    id |= timestamp << (WORKER_BITS + SEQUENCE_BITS + CLUSTER_BITS);
    id |= (cluster as u64) << (WORKER_BITS + SEQUENCE_BITS);
    id |= (worker as u64) << SEQUENCE_BITS;
    id |= sequence as u64;
    Snowflake(id)
}

pub fn next_snowflake() -> Snowflake {
    WORKER.with(|w| w.borrow_mut().generate())
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: String,
    pub data: String,
    pub priority: String, // "high", "medium", "low"
    pub status: String,   // "queued", "processing", "complete"
    pub timestamp: u64,
}
use crate::task::Task;
use std::collections::VecDeque;
use ic_cdk_macros::*;

static mut QUEUE: Option<VecDeque<Task>> = None;

#[init]
fn init() {
    unsafe { QUEUE = Some(VecDeque::new()) }
}

#[update]
fn enqueue_task(id: String, data: String, priority: String) {
    let task = Task {
        id,
        data,
        priority,
        status: "queued".to_string(),
        timestamp: ic_cdk::api::time(),
    };
    unsafe { QUEUE.as_mut().unwrap().push_back(task) }
}

#[update]
fn dequeue_task() -> Option<Task> {
    unsafe { QUEUE.as_mut().unwrap().pop_front() }
}

#[query]
fn peek_task() -> Option<Task> {
    unsafe { QUEUE.as_ref().unwrap().front().cloned() }
}

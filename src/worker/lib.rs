use ic_cdk_macros::*;

#[update]
fn process_task() {
    let task: Option<Task> = ic_cdk::call("queue_canister_id", "dequeue_task", ()).unwrap();
    if let Some(mut t) = task {
        t.status = "processing".to_string();
        // Simulasi proses (print payload)
        ic_cdk::println!("Processing task: {}", t.data);
        t.status = "complete".to_string();
    }
}

// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 用 Arc 包裹 Mutex，保证多线程安全修改共享数据
    let status = Arc::new(std::sync::Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 线程安全地获取锁，修改 jobs_completed
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    // 必须 join 所有线程，确保所有任务都完成
    for handle in handles {
        handle.join().unwrap();
        // 每次 join 后都可以打印当前已完成的任务数
        // 这里用 lock 获取最新值
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}

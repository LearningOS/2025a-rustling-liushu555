// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    // 用于保存每个线程的运行时间（毫秒）
    let mut results: Vec<u128> = vec![];
    // 遍历所有线程句柄，等待线程结束并收集返回值
    for handle in handles {
        // handle 是 JoinHandle，调用 join() 等待线程结束并获取返回值
        // join() 返回 Result<T, E>，T 是线程返回值，这里是 u128
        let elapsed = handle.join().expect("Thread panicked");
        results.push(elapsed);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}

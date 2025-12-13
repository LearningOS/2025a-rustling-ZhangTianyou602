use std::sync::{Arc, Mutex}; // 新增 Mutex 导入
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 用 Arc + Mutex 包裹共享值，实现线程安全的共享修改
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // 克隆 Arc，增加引用计数
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 加锁并修改共享值（lock() 返回 Result，unwrap 处理可能的锁中毒）
            let mut status_guard = status_shared.lock().unwrap();
            status_guard.jobs_completed += 1;
            // 锁会在 status_guard 离开作用域时自动释放
        });
        handles.push(handle);
    }

    // 等待所有线程执行完成（必须 join 所有 handle，否则主线程可能提前退出）
    for handle in handles {
        handle.join().unwrap();
    }

    // 加锁并打印最终的 jobs_completed 值
    let final_status = status.lock().unwrap();
    println!("jobs completed {}", final_status.jobs_completed);
}
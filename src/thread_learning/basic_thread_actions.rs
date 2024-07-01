use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn spawn_thread() -> JoinHandle<()> {
    return thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
}
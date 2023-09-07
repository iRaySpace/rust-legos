use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();
    
    let v = vec![1, 2, 3];
    let v_handle = thread::spawn(move || {
        println!("Spawned thread: {:?}", v);
    });

    for i in 1..=5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
}

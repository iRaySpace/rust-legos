use std::thread;
use std::sync::Mutex;


fn main() {
    let big_data = vec![1, 2, 3, 4, 5];
    let mutex = Mutex::new(big_data);

    let new_big_data = vec![];
    let new_mutex = Mutex::new(new_big_data);
    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let data = mutex.lock().unwrap().pop();
                if data.is_none() {
                    break;
                }
                let new_data = data.unwrap() * 100;
                thread::sleep(std::time::Duration::from_millis(300));
                println!("First thread inserting {}", new_data);
                new_mutex.lock().unwrap().push(new_data);
            }
        });
        s.spawn(|| {
            loop {
                let data = mutex.lock().unwrap().pop();
                if data.is_none() {
                    break;
                }
                let new_data = data.unwrap() * 100;
                thread::sleep(std::time::Duration::from_millis(300));
                println!("Second thread inserting {}", new_data);
                new_mutex.lock().unwrap().push(new_data);
            }
        });
    });
    println!("{:?}", new_mutex);
}

use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    println!("{:?}", mutex);

    let mut mutex_lock = mutex.lock().expect("Lock is expected");
    println!("{:?}", mutex_lock);
    println!("{:?}", mutex);

    *mutex_lock = 10;
    println!("{:?}", mutex_lock);
    println!("{:?}", mutex);

    std::mem::drop(mutex_lock);
    // println!("{:?}", mutex_lock); - error because mutex_lock is dropped
    println!("{:?}", mutex);

    let mut ivan_lock = mutex.lock().expect("Lock is expected");
    println!("{:?}", ivan_lock);
    println!("{:?}", mutex);

    *ivan_lock = 15;
    std::thread::sleep(std::time::Duration::from_millis(5000));
    std::mem::drop(ivan_lock);

    let mut sher_lock = mutex.lock().expect("Lock is expected");
    println!("{:?}", sher_lock);
    println!("{:?}", mutex);

    *sher_lock = 20;
    std::mem::drop(sher_lock);

    println!("{:?}", mutex);

    *mutex.lock().unwrap() = 25;
    println!("{:?}", mutex);
}

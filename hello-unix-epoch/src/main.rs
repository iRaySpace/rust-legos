use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut unix_timestamp = 0;
    loop {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Can't retrieve time");
        if unix_timestamp != since_the_epoch.as_secs() {
            println!("{}", unix_timestamp);
        }
        unix_timestamp = since_the_epoch.as_secs();
    }
}

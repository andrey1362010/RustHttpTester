
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::thread::JoinHandle;
use curl::easy::Easy;
use curl::Error;

fn main() {
    let req_count = 20000;
    const threads_count: u128 = 10;
    let start_time = time_millis();
    let mut join_array: Vec<JoinHandle<()>> = vec![];
    for t in 0..threads_count {
        let mut handle =
            thread::spawn(move || {
                let mut handle = Easy::new();
                for i in 0..req_count {
                    handle.url("http://127.0.0.1:13265/").unwrap();
                    match handle.perform(){
                        Ok(_) => {}
                        Err(err) => {print!("{}", err)}
                    }
                    //let resp = reqwest::blocking::get("http://127.0.0.1:13265/").unwrap();
                }
                println!("Thread done:{}", t);
            });
        join_array.push(handle);
    }
    for handle in join_array {
        handle.join().unwrap();
    }
    let end_time = time_millis();
    println!("Total speed req/sec:{}", threads_count * (1000 * req_count / (end_time - start_time)));
    return ();
}

fn time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // thread_basics();
    // messaging();
    shared_concurrency();
}

fn thread_basics() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} in spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number {i} in main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn messaging() {
    let (transmitter, receiver) = mpsc::channel();

    let clone = transmitter.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            transmitter.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            clone.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for received in receiver {
        println!("Received: {received}");
    }
}

fn shared_concurrency() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result= {:?}", counter.lock().unwrap());
}

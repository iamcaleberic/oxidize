use std::sync::{mpsc,Mutex};
use std::thread;

fn sync_inc(counter: &Mutex<i32>) {
    let mut data =  counter.lock().unwrap();
    *data += 1;

    println!("{:?}", *data)
}


pub fn mx() {
    let counter = Mutex::new(8);
    sync_inc(&counter);

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send(5)
    });

    thread::spawn(move || {
        tx2.send(4)
    });

    // prints 4 and 5 in an unspecified order
    println!("{:?}", rx.recv());
    println!("{:?}", rx.recv());
}


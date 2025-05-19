use core::hash;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use std::sync::{Arc, Mutex, mpsc};
fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(2));
    // }

    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     // println!("val is {}", val);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("tx1 - hi"),
    //         String::from("tx1 - from"),
    //         String::from("tx1 - the"),
    //         String::from("tx1 - thread"),
    //     ];
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("tx2 - hi"),
    //         String::from("tx2 - from"),
    //         String::from("tx2 - the"),
    //         String::from("tx2 - thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap())

    let a = Arc::new(Mutex::new(5));
    let b = Arc::new(Mutex::new(6));
    let a_clone = a.clone();
    let b_clone = b.clone();
    let a_clone2 = a.clone();
    let b_clone2 = b.clone();

    let handle1 = thread::spawn(move || {
        loop {
            let lock_a = a_clone.lock().unwrap();
            println!("Thread 1: a = {}", lock_a);
            match b_clone.try_lock() {
                Ok(lock_b) => {
                    println!("Thread 1: b = {}", lock_b);
                    break;
                }
                Err(_) => {
                    println!("Thread 1: b is locked, retrying...");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    });

    let handle2 = thread::spawn(move || {
        loop {
            let lock_b = b_clone2.lock().unwrap();
            println!("Thread 2: b = {}", lock_b);
            match a_clone2.try_lock() {
                Ok(lock_a) => {
                    println!("Thread 2: a = {}", lock_a);
                    break;
                }
                Err(_) => {
                    println!("Thread 2: a is locked, retrying...");
                    drop(lock_b);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

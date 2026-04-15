use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;
    
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];
    
    for producer_id in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(producer_id, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        });
        handles.push(handle);
    }
    
    for consumer_id in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(consumer_id, rx_clone);
        });
        handles.push(handle);
    }
    
    for _ in 0..NUM_PRODUCERS {
        handles.remove(0).join().expect("Producer thread panicked");
    }
    
    println!("[Main] All producers finished. Sending termination signals...");
    
    for i in 0..NUM_CONSUMERS {
        match tx.send(TERMINATION_SIGNAL) {
            Ok(_) => {
                println!("[Main] Sent termination signal {} of {}", i + 1, NUM_CONSUMERS);
            }
            Err(_) => {
                println!("[Main] Error sending termination signal (channel closed)");
                break;
            }
        }
    }
    
    for handle in handles {
        handle.join().expect("Consumer thread panicked");
    }
    
    println!("\nAll items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    for i in 0..item_count {
        let num = (id as i32 * 100) + (i as i32) + 1;
        
        match tx.send(num) {
            Ok(_) => {
                println!("[Producer {}] Sent item {}/{}: {}", id, i + 1, item_count, num);
            }
            Err(_) => {
                println!("[Producer {}] Channel closed, stopping production", id);
                break;
            }
        }
    }
    
    println!("[Producer {}] Finished producing {} items", id, item_count);
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    let mut count = 0;
    
    loop {
        let receiver = rx.lock().unwrap();
        
        match receiver.recv() {
            Ok(num) => {
                if num == TERMINATION_SIGNAL {
                    println!("[Consumer {}] Received termination signal. Processed {} items total.", id, count);
                    drop(receiver);
                    break;
                }
                
                println!("[Consumer {}] Received: {}", id, num);
                count += 1;
            }
            Err(_) => {
                println!("[Consumer {}] Channel closed unexpectedly. Processed {} items.", id, count);
                drop(receiver);
                break;
            }
        }
    }
}
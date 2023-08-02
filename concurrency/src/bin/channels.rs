use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel::<f32>();
    let tx2 = tx1.clone();
    // Send value through channel from first thread.
    thread::spawn(move || {
        let values = vec![32.23, 232.55, 80.2390, 2032.023];

        for value in values {
            tx1.send(value * value).unwrap();
            thread::sleep(Duration::from_millis(500));
        }

        // Receiver will stop listening for new message.
        println!("Dropping 1st sender...");
    });

    // Send value through channel from second thread.
    thread::spawn(move || {
        let values = vec![20.23, 225.53, 323.023];

        for value in values {
            tx2.send(value * value).unwrap();
            thread::sleep(Duration::from_millis(500));
        }

        // Receiver will stop listening for new message.
        println!("Dropping 2nd sender...");
    });

    // Receiver will wait for message till sender is not dropped.
    for value in rx {
        println!("{:}", value);
    }
    println!("End of receiver iterator because no sender exists");
}

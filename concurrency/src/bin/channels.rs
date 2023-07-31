use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (send, receive) = mpsc::channel::<f32>();

    // Send value through channel.
    thread::spawn(move || {
        let values = vec![32.23, 232.55, 80.2390, 2032.023];

        for value in values {
            send.send(value * value).unwrap();
            thread::sleep(Duration::from_millis(500));
        }

        // Receiver will stop listening for new message.
        println!("Dropping sender...");
    });

    // Receiver will wait for message till sender is not dropped.
    for value in receive {
        println!("{:}", value);
    }
    println!("End of receiver iterator because no sender exists");
}

use colored::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Some heavy task.
    thread::spawn(|| {
        for i in 0..5 {
            println!("{}", format!("First Thread  -  {i}").blue());
        }
    });

    // Some heavy task 2.
    thread::spawn(|| {
        for i in 0..5 {
            println!("{}", format!("Second Thread -  {i}").green());
        }
    });

    thread::sleep(Duration::from_secs(1));
    println!("End of main thread");
}

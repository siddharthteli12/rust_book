use colored::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Main thread variable.
    let students = vec!["Foo", "Too", "Loo"];
    // Some heavy task.
    let thread1 = thread::spawn(|| {
        for i in 0..5 {
            println!("{}", format!("First Thread  -  {i}").blue());
        }

        // Below line will throw error, because this thread can outlive current
        // functions. Hence, students can be invalid.
        // Thread needs to take ownership.
        // println!("Student info {:?}", students);
    });

    // Some heavy task 2.
    let thread2 = thread::spawn(move || {
        for i in 0..5 {
            println!("{}", format!("Second Thread -  {i}").green());
        }
        // stuendts is owned by current thread.
        println!("Student info {:?}", students);
    });

    // Sleep main thread for 1 sec.
    thread::sleep(Duration::from_secs(1));

    // Explicitly finish threads.
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("End of main thread");
}

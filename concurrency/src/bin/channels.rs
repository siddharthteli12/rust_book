use std::sync::mpsc;
use std::thread;
struct StudentMarks {
    english: f32,
    maths: f32,
    history: f32,
}
fn main() {
    let (send, receive) = mpsc::channel::<f32>();

    // Send value through channel.
    let send_thread = thread::spawn(move || {
        send.send(calculate_marks(StudentMarks {
            english: 50.0,
            maths: 80.0,
            history: 90.0,
        }))
        .unwrap();
    });

    // Receive value through channel.
    let receive_thread = thread::spawn(move || {
        let value = receive.recv().unwrap();
        println!("Student marks cal. - {:?}", value);
    });

    // Finish both threads.
    send_thread.join().unwrap();
    receive_thread.join().unwrap();
}

// Simple utility function.
fn calculate_marks(student_info: StudentMarks) -> f32 {
    (student_info.english + student_info.history + student_info.maths) / 3_f32
}

mod closures_eg;
use closures_eg::{closure_refernece, closure_thread, closure_trait};
fn main() {
    closure_refernece::reference_ownership();
    closure_thread::thread1();
    closure_trait::fn_trait();
}

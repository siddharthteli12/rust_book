mod closures_eg;
use closures_eg::{closure_refernece, closure_sorting, closure_thread, closure_trait, option_impl};

fn main() {
    closure_refernece::reference_ownership();
    closure_thread::thread1();
    closure_trait::fn_trait();
    option_impl::custom_option();
    closure_sorting::closure_sort_fnmut();
}

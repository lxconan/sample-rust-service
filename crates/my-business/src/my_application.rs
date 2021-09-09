use sample_rust_service_core::error::{ServiceError, ServiceResult};
use sample_rust_service_core::diagnostic::output_debug_string;
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Application {
}

impl sample_rust_service_core::application::Application for Application {
    fn handle_error(&self, _error: &ServiceError) {
        output_debug_string("Application::handle_error() called");
    }

    fn run(&self, exit_signal: Arc<AtomicBool>) -> ServiceResult<()> {
        let exit_signal_for_worker_one = exit_signal.clone();
        let thread_handle_worker_one = thread::spawn(move || { do_some_work(String::from("Worker 1"), exit_signal_for_worker_one); });

        let exit_signal_for_worker_two = exit_signal.clone();
        let thread_handle_worker_two = thread::spawn(move || { do_some_work(String::from("Worker 2"), exit_signal_for_worker_two); });

        thread_handle_worker_two.join().unwrap_or_else(|e| { output_debug_string(format!("Failure occurred when joining thread: {:?}", e)) });
        thread_handle_worker_one.join().unwrap_or_else(|e| { output_debug_string(format!("Failure occurred when joining thread: {:?}", e)) });

        output_debug_string("Application::run() exits");
        Ok(())
    }
}

fn do_some_work(name: String, exit_signal: Arc<AtomicBool>) {
    let running_message = format!("Thread is running - {}", &name);
    let exit_message = format!("Thread will exit - {}", &name);

    while !exit_signal.load(Ordering::SeqCst) {
        output_debug_string(&running_message);
        thread::sleep(Duration::from_secs(2));
    }
    output_debug_string(&exit_message);
}
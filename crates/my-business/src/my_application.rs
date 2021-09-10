use sample_rust_service_core::error::{ServiceError, ServiceResult};
use sample_rust_service_core::diagnostic::output_debug_string;
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct WorkerApplicationOne {}

impl sample_rust_service_core::application::SimpleApplication for WorkerApplicationOne {
    fn handle_error(&self, error: &ServiceError) {
        output_debug_string(format!("Application error: {:?}", error));
    }

    fn run(&self, exit_signal: Arc<AtomicBool>) -> ServiceResult<()> {
        do_some_work(String::from("Worker 1"), exit_signal);
        Ok(())
    }
}

pub struct WorkerApplicationTwo {}

impl sample_rust_service_core::application::SimpleApplication for WorkerApplicationTwo {
    fn handle_error(&self, error: &ServiceError) {
        output_debug_string(format!("Application error: {:?}", error));
    }

    fn run(&self, exit_signal: Arc<AtomicBool>) -> ServiceResult<()> {
        do_some_work(String::from("Worker 2"), exit_signal);
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
use windows_service_rs_core::error::{ServiceError, ServiceResult};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct WorkerApplicationOne {}

impl windows_service_rs_core::application::SimpleApplication for WorkerApplicationOne {
    fn handle_error(&self, error: &ServiceError) {
        log::error!("Application error: {:?}", error);
    }

    fn run(&self, exit_signal: Arc<AtomicBool>) -> ServiceResult<()> {
        do_some_work(String::from("Worker 1"), exit_signal);
        Ok(())
    }
}

pub struct WorkerApplicationTwo {}

impl windows_service_rs_core::application::SimpleApplication for WorkerApplicationTwo {
    fn handle_error(&self, error: &ServiceError) {
        log::error!("Application error: {:?}", error);
    }

    fn run(&self, exit_signal: Arc<AtomicBool>) -> ServiceResult<()> {
        do_some_work(String::from("Worker 2"), exit_signal);
        Ok(())
    }
}

fn do_some_work(name: String, exit_signal: Arc<AtomicBool>) {
    while !exit_signal.load(Ordering::SeqCst) {
        log::info!("Thread is running - {}", &name);
        thread::sleep(Duration::from_secs(2));
    }
    log::info!("Thread will exit - {}", &name);
}
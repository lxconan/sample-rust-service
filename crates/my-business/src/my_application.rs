use sample_rust_service_core::error::{ServiceError, ServiceResult};
use sample_rust_service_core::diagnostic::output_debug_string;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Application {
}

unsafe impl Sync for Application {
}

impl sample_rust_service_core::application::Application for Application {
    fn handle_error(&self, _error: &ServiceError) {
        output_debug_string("Application::handle_error() called");
    }

    fn run(&self, shutdown_rx: &Receiver<()>) -> ServiceResult<()> {
        let exit_signal = Arc::new(AtomicBool::new(false));

        let thread_func = |signal:Arc<AtomicBool>| {
            while !signal.load(Ordering::SeqCst) {
                output_debug_string("Worker thread running.");
                thread::sleep(Duration::from_secs(2));
            }
            output_debug_string("Worker thread exit.");
        };

        let exit_signal_for_threat = exit_signal.clone();
        let thread_handle = thread::spawn(move || {
            thread_func(exit_signal_for_threat);
        });

        loop {
            match shutdown_rx.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    output_debug_string("Ok or Disconnected received");
                    exit_signal.store(true, Ordering::SeqCst);
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        thread_handle.join().unwrap_or_else(|e| {
            output_debug_string(format!("Failure occurred when joining thread: {:?}", e))
        });

        output_debug_string("Application::run() exits");
        Ok(())
    }
}
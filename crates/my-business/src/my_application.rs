use sample_rust_service_core::error::{ServiceError, ServiceResult};
use sample_rust_service_core::diagnostic::output_debug_string;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Application {}

impl sample_rust_service_core::application::Application for Application {
    fn handle_error(&self, _error: &ServiceError) {
        output_debug_string("Application::handle_error() called");
    }

    fn initialize(&self) -> ServiceResult<()> {
        output_debug_string("Application::initialize() called");
        Ok(())
    }

    fn run(&self, shutdown_rx: &Receiver<()>) -> ServiceResult<()> {
        loop {
            match shutdown_rx.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    sample_rust_service_core::diagnostic::output_debug_string("Ok or Disconnected received");
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    sample_rust_service_core::diagnostic::output_debug_string("Entering windows service loop");
                    thread::sleep(Duration::from_secs(2));
                }
            }
        }
        output_debug_string("Application::run() exits");
        Ok(())
    }

    fn shutting_down(&self) {
        output_debug_string("Application::shutting_down() called");
    }
}
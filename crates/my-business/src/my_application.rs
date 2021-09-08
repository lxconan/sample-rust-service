use sample_rust_service_core::error::{ServiceError, ServiceResult};
use sample_rust_service_core::diagnostic::output_debug_string;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Application {}

unsafe impl Sync for Application {
}

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
                    run_zeromq_server();
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

fn run_zeromq_server() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    let mut message_cnt = 0;
    loop {
        responder.recv(&mut msg, 0).unwrap();
        message_cnt = message_cnt + 1;
        println!("Received message {} : \"{}\"", message_cnt, msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("Hello to client", 0).unwrap();
        println!("Sending Hello to client ...");
    }
}
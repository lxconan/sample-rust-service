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
        // let responder = init_zeromq_server().unwrap();
        loop {
            match shutdown_rx.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    sample_rust_service_core::diagnostic::output_debug_string("Ok or Disconnected received");
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    sample_rust_service_core::diagnostic::output_debug_string("Entering windows service loop");
                    thread::sleep(Duration::from_secs(2));
                    // run_zeromq_server();
                    // send_and_receive(&responder);
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

// fn init_zeromq_server() -> ServiceResult<()>{
//     let context = zmq::Context::new();
//     let responder = context.socket(zmq::REP).unwrap();

//     if responder.bind("tcp://*:5555").is_ok() {
//         return Ok(responder);
//     } else {
//         return ServiceResult::Err(ServiceError::new("init socket failed."));
//     }
// }

// fn send_and_receive(responder: &Socket) {
//     let mut msg = zmq::Message::new();
//     responder.recv(&mut msg, 0).unwrap();
//     println!("Received message: \"{}\"", msg.as_str().unwrap());
//     thread::sleep(Duration::from_millis(1000));
//     responder.send("Hello to client", 0).unwrap();
//     println!("Sending Hello to client ...");
// }

// fn run_zeromq_server() {
//     let context = zmq::Context::new();
//     let responder = context.socket(zmq::REP).unwrap();

//     assert!(responder.bind("tcp://*:5555").is_ok());

//     let mut msg = zmq::Message::new();

//     responder.recv(&mut msg, 0).unwrap();
//     println!("Received message : \"{}\"", msg.as_str().unwrap());
//     thread::sleep(Duration::from_millis(1000));
//     responder.send("Hello to client", 0).unwrap();
//     println!("Sending Hello to client ...");
// }
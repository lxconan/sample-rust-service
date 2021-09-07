use std::{
    sync::mpsc,
    time::Duration,
    thread
};

pub fn core_service_init()
{
    diagnostic::output_debug_string("initialize something here.")
}

pub fn core_service_process(shutdown_rx: std::sync::mpsc::Receiver<()>) {
    println!("service in process.");
    loop {
        match shutdown_rx.try_recv() {
            Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                diagnostic::output_debug_string("Ok or Disconnected received");
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                diagnostic::output_debug_string("Entering windows service loop");
                thread::sleep(Duration::from_secs(10));
            }
        }
    }
}

pub fn core_service_stop()
{
    diagnostic::output_debug_string("service stopped.")
}
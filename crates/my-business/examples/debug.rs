use my_business::my_application::Application as BusinessApplication;
use sample_rust_service_core::application::Application;
use sample_rust_service_core::error::{ServiceResult, ServiceError};
use std::sync::mpsc;
use std::io::{stdin};

fn main() -> ServiceResult<()> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let handle = std::thread::spawn(|| -> ServiceResult<()> {
        let receiver = shutdown_rx;

        let application = BusinessApplication {};
        application.initialize()?;
        application.run(&receiver)?;
        application.shutting_down();
        Ok(())
    });

    println!("press any key to stop...");

    let mut user_input:String = String::default();
    stdin().read_line(&mut user_input).map_err(|e| { ServiceError::with(e, "IO error. ") })?;

    println!("Application is about to exit!");
    shutdown_tx.send(()).map_err(|e| { ServiceError::with(e, "Stop signal sending error. ") })?;
    match handle.join() {
        Ok(_) => { return Ok(()); }
        Err(_) => { return ServiceResult::Err(ServiceError::new("Joining failed. "))}
    }
}
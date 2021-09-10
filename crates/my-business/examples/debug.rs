use my_business::my_application::WorkerApplicationOne as BusinessApplication;
use sample_rust_service_core::application::{SimpleApplication};
use sample_rust_service_core::error::{ServiceResult, ServiceError};
use std::sync::{Arc};
use std::io::{stdin};
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> ServiceResult<()> {
    let exit_signal = Arc::new(AtomicBool::new(false));

    let exit_signal_for_thread = exit_signal.clone();
    let handle = std::thread::spawn(move || -> ServiceResult<()> {
        let application = BusinessApplication {};
        application.run(exit_signal_for_thread)?;
        Ok(())
    });

    println!("press any key to stop...");

    let mut user_input:String = String::default();
    stdin().read_line(&mut user_input).map_err(|e| { ServiceError::with(e, "IO error. ") })?;

    println!("Application is about to exit!");
    exit_signal.store(true, Ordering::SeqCst);

    match handle.join() {
        Ok(_) => { return Ok(()); }
        Err(_) => { return ServiceResult::Err(ServiceError::new("Joining failed. "))}
    }
}
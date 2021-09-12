use my_business::my_application::WorkerApplicationOne as BusinessApplication;
use windows_service_rs_core::application::{SimpleApplication};
use windows_service_rs_core::error::{ServiceResult, ServiceError};
use std::sync::{Arc};
use std::io::{stdin};
use std::sync::atomic::{AtomicBool, Ordering};
use windows_service_rs_core::win_dbg_logger;

fn main() -> ServiceResult<()> {
    win_dbg_logger::init();
    simulate(|| { Box::new(BusinessApplication {}) })
}

fn simulate(application_factory:fn() -> Box<dyn SimpleApplication>) -> ServiceResult<()> {
    let exit_signal = Arc::new(AtomicBool::new(false));

    let exit_signal_for_thread = exit_signal.clone();
    let handle = std::thread::spawn(move || -> ServiceResult<()> {
        let application = application_factory();
        application.run(exit_signal_for_thread)?;
        Ok(())
    });

    println!("press any key to stop...");

    let mut user_input:String = String::default();
    stdin().read_line(&mut user_input).map_err(|e| { ServiceError::with(e, "IO error. ") })?;

    println!("Application is about to exit!");
    exit_signal.store(true, Ordering::SeqCst);

    return match handle.join() {
        Ok(_) => { Ok(()) }
        Err(_) => { ServiceResult::Err(ServiceError::new("Joining failed. ")) }
    }
}
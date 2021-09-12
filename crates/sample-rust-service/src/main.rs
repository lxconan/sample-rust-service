use sample_rust_service_core::error::ServiceResult;
use sample_rust_service_core::application::SimpleApplication;

mod service_wrapper;

#[cfg(debug_assertions)]
macro_rules! init_logger {
    () => { sample_rust_service_core::win_dbg_logger::init(); }
}

#[cfg(not(debug_assertions))]
macro_rules! init_logger {
    () => { simple_logger::init().unwrap_or_else(|_|{}) }
}

fn main() -> ServiceResult<()> {
    init_logger!();
    log::info!("Creating applications factory");
    let application1:fn()->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationOne {})};
    let application2:fn()->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationTwo {})};
    log::info!("Initialize service wrapper");
    service_wrapper::run(vec![application1, application2])
}

use windows_service_rs_core::application::SimpleApplication;
use windows_service_rs_core::error::ServiceResult;
use windows_service_rs_core::service_wrapper;

#[cfg(debug_assertions)]
macro_rules! init_logger {
    () => { windows_service_rs_core::win_dbg_logger::init(); }
}

#[cfg(not(debug_assertions))]
macro_rules! init_logger {
    () => { simple_logger::init().unwrap_or_else(|_|{}) }
}

fn main() -> ServiceResult<()> {
    init_logger!();
    let application1_factory:fn() ->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationOne {})};
    let application2_factory:fn() ->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationTwo {})};
    service_wrapper::run(vec![application1_factory, application2_factory])
}

use sample_rust_service_core::error::ServiceResult;
use sample_rust_service_core::application::SimpleApplication;

mod service_wrapper;

fn main() -> ServiceResult<()> {
    let application1:fn()->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationOne {})};
    let application2:fn()->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationTwo {})};
    service_wrapper::run(vec![application1, application2])
}

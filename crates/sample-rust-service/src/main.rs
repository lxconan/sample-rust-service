use sample_rust_service_core::error::ServiceResult;

mod service_wrapper;

fn main() -> ServiceResult<()> {
    let application = Box::new(my_business::my_application::Application {});
    service_wrapper::run(application)
}

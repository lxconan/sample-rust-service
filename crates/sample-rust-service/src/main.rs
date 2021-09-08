use sample_rust_service_core::error::ServiceResult;

mod service_wrapper;

fn main() -> ServiceResult<()> {
    service_wrapper::run(&my_business::my_application::Application{})
}

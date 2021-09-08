use sample_rust_service_core::error::ServiceResult;

mod service_wrapper;

static APPLICATION:my_business::my_application::Application = my_business::my_application::Application {};

fn main() -> ServiceResult<()> {
    service_wrapper::run(&APPLICATION)
}

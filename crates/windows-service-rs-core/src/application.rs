use crate::error::{ServiceError, ServiceResult};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub trait Application {
    fn handle_error(&self, error:&ServiceError);
    fn initialize(&self) -> ServiceResult<()> { Ok(()) }
    fn run(&self, exit_signal:Arc<AtomicBool>) -> ServiceResult<()>;
    fn shutting_down(&self) {}
}

pub trait SimpleApplication {
    fn handle_error(&self, error:&ServiceError);
    fn run(&self, exit_signal:Arc<AtomicBool>) -> ServiceResult<()>;
}
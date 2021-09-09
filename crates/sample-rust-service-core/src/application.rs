use crate::error::{ServiceError, ServiceResult};
use std::sync::mpsc::Receiver;

pub trait Application {
    fn handle_error(&self, error:&ServiceError);
    fn initialize(&self) -> ServiceResult<()> { Ok(()) }
    fn run(&self, shutdown_rx:&Receiver<()>) -> ServiceResult<()>;
    fn shutting_down(&self) {}
}

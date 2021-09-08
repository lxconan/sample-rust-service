use crate::error::{ServiceError, ServiceResult};
use std::sync::mpsc::Receiver;

pub trait Application : Sync {
    fn handle_error(&self, error:&ServiceError);
    fn initialize(&self) -> ServiceResult<()>;
    fn run(&self, shutdown_rx:&Receiver<()>) -> ServiceResult<()>;
    fn shutting_down(&self);
}
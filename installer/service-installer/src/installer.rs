use crate::service_wrapper::WindowsServiceOperatingContext;

use colored::Colorize;
use windows_service::{
    service::{ServiceAccess, ServiceState},
    service_manager::{ServiceManagerAccess},
};

use crate::error::InstallerError;
use windows_service::service::ServiceStatus;
use std::io::Write;

pub const DEFAULT_TIMEOUT:u32 = 20;

pub fn uninstall_windows_service(service_name:&str) -> Result<(), InstallerError> {
    println!("Attempt to uninstall windows service: {}", service_name.cyan());

    let context = WindowsServiceOperatingContext::from(
        service_name,
        ServiceManagerAccess::CONNECT,
        ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)?;

    let service_status = try_query_service_status(&context)?;

    // we need to make sure that the service can be removed after the statement.
    match service_status.current_state {
        ServiceState::Stopped => { /* it can be removed safely */ }
        ServiceState::StartPending => {
            try_wait_for_status(&context, ServiceState::Running)?;
            try_stop_and_wait(&context)?;
        }
        ServiceState::StopPending => {  try_wait_for_status(&context, ServiceState::Stopped)?; },
        ServiceState::Running => { try_stop_and_wait(&context)?; },
        ServiceState::ContinuePending => {
            try_wait_for_status(&context, ServiceState::Running)?;
            try_stop_and_wait(&context)?;
        },
        ServiceState::PausePending => { try_wait_for_status(&context, ServiceState::Paused)?; },
        ServiceState::Paused => { /* we are not sure how to do that */ },
    }

    try_uninstall_service(context)
}

fn try_stop_and_wait(context:&WindowsServiceOperatingContext) -> Result<(), InstallerError> {
    try_stop_service(context)?;
    try_wait_for_status(context, ServiceState::Stopped)
}

fn try_uninstall_service(context:WindowsServiceOperatingContext) -> Result<(), InstallerError> {
    print!("Uninstalling service {} ...", context.service_name.as_str().cyan());
    context.uninstall_service()?;
    println!("{}", "Done".green());
    Ok(())
}

fn try_query_service_status(context:&WindowsServiceOperatingContext) -> Result<ServiceStatus, InstallerError> {
    print!("Query service status {}...", context.service_name.as_str().cyan());
    let service_status = context.query_service_status()?;
    print_done();
    println!("Current service status: {:?}", service_status.current_state);
    Ok(service_status)
}

fn try_wait_for_status(context:&WindowsServiceOperatingContext, desired_state:ServiceState) -> Result<(), InstallerError> {
    print!("Waiting for service {} to {:?} ", context.service_name.as_str().cyan(), desired_state);
    context.wait_for_service_status(desired_state, DEFAULT_TIMEOUT, print_progress_bar)?;
    print_done();
    Ok(())
}

fn try_stop_service(context:&WindowsServiceOperatingContext) -> Result<(), InstallerError> {
    print!("Trying to stop service {}...", context.service_name.as_str().cyan());
    context.stop_service()?;
    print_done();
    Ok(())
}

fn print_progress_bar() { print!("."); std::io::stdout().flush().unwrap_or_default(); }
fn print_done() { println!("{}", "Done".green()); }
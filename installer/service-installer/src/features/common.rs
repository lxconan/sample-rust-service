use crate::error::{InstallerError, InstallerResult};
use crate::features::service_wrapper::WindowsServiceOperatingContext;
use windows_service::service::{ServiceState, ServiceStatus};
use colored::Colorize;
use std::io::Write;

pub const DEFAULT_TIMEOUT:u32 = 20;

pub fn try_stop_and_wait(context:&WindowsServiceOperatingContext) -> Result<(), InstallerError> {
    try_stop_service(context)?;
    try_wait_for_status(context, ServiceState::Stopped)
}

pub fn try_start_and_wait(context:&WindowsServiceOperatingContext) -> InstallerResult<()> {
    try_start_service(context)?;
    try_wait_for_status(context, ServiceState::Running)
}

pub fn try_uninstall_service(context:WindowsServiceOperatingContext) -> Result<(), InstallerError> {
    print!("Uninstalling service {} ...", context.service_name.as_str().cyan());
    context.uninstall_service()?;
    println!("{}", "Done".green());
    Ok(())
}

pub fn try_query_service_status(context:&WindowsServiceOperatingContext) -> Result<ServiceStatus, InstallerError> {
    print!("Query service status {}...", context.service_name.as_str().cyan());
    let service_status = context.query_service_status()?;
    print_done();
    println!("Current service status:                 {}", format!("{:?}", service_status.current_state).as_str().cyan());
    println!("Current service type:                   {}", format!("{:?}", service_status.service_type).as_str().cyan());
    println!("Current service PID:                    {}", service_status.process_id.map(|v| v.to_string()).unwrap_or(String::from("(none)")).as_str().cyan());
    Ok(service_status)
}

pub fn try_wait_for_status(context:&WindowsServiceOperatingContext, desired_state:ServiceState) -> Result<(), InstallerError> {
    print!("Waiting for service {} to {} ", context.service_name.as_str().cyan(), format!("{:?}", desired_state).as_str().cyan());
    context.wait_for_service_status(desired_state, DEFAULT_TIMEOUT, print_progress_bar)?;
    print_done();
    Ok(())
}

pub fn try_stop_service(context:&WindowsServiceOperatingContext) -> Result<(), InstallerError> {
    print!("Trying to stop service {}...", context.service_name.as_str().cyan());
    context.stop_service()?;
    print_done();
    Ok(())
}

pub fn try_start_service(context:&WindowsServiceOperatingContext) -> InstallerResult<()> {
    print!("Trying to start service {}...", context.service_name.as_str().cyan());
    context.start_service()?;
    print_done();
    Ok(())
}

fn print_progress_bar() { print!("."); std::io::stdout().flush().unwrap_or_default(); }
fn print_done() { println!("{}", "Done".green()); }
use crate::service_wrapper::WindowsServiceOperatingContext;

use colored::Colorize;
use windows_service::{
    service::{ServiceAccess, ServiceState},
    service_manager::{ServiceManagerAccess},
};

use crate::error::InstallerError;

pub const DEFAULT_TIMEOUT:u32 = 20;

pub fn install_windows_service(
    service_name:&str,
    display_name:&str,
    description:&str,
    auto_start:bool,
    service_binary_path:&str) -> Result<(), InstallerError> {
    println!("Attempt to install windows service: {}", service_name.cyan());

    print!("Creating windows service ...");
    WindowsServiceOperatingContext::create_windows_service(service_name, display_name, description, auto_start, service_binary_path)?;
    println!("{}", "Done".green());
    Ok(())
}

pub fn uninstall_windows_service(service_name:&str) -> Result<(), InstallerError> {
    println!("Attempt to uninstall windows service: {}", service_name.cyan());

    let context = WindowsServiceOperatingContext::from(
        service_name,
        ServiceManagerAccess::CONNECT,
        ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)?;

    print!("Query service status for {} ...", service_name.cyan());
    let service_status = context.query_service_status()?;
    println!("{}", "Done".green());
    println!("Current service status: 0x{:02x}", service_status.current_state as u32);

    if service_status.current_state != ServiceState::Stopped {
        print!("Trying to stop service...");
        context.stop_service()?;
        println!("{}", "Done".green());

        print!("Waiting for service to stop ");
        context.wait_for_service_status(ServiceState::Stopped, DEFAULT_TIMEOUT, || { print!("."); })?;
        println!("{}", "Done".green());
    } else {
        println!("Current service stopped. Now we will try uninstall windows service.");
    }

    print!("Uninstalling service {} ...", service_name.cyan());
    context.uninstall_service()?;
    println!("{}", "Done".green());

    Ok(())
}
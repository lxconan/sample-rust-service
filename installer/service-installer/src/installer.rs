use crate::error::InstallerError;
use std::ffi::OsString;
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType, ServiceState},
    service_manager::{ServiceManager, ServiceManagerAccess},
};
use std::thread;
use std::time;
use colored::Colorize;

pub fn install_windows_service(
    service_name:&str,
    display_name:&str,
    description:&str,
    auto_start:bool,
    service_binary_path:&str) -> Result<(), InstallerError> {
    println!("Attempt to install windows service: {}", service_name.cyan());

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = get_service_manager(manager_access)?;

    let service_info = ServiceInfo {
        name: OsString::from(service_name),
        display_name: OsString::from(display_name),
        service_type: ServiceType::OWN_PROCESS,
        start_type: if auto_start { ServiceStartType::AutoStart } else { ServiceStartType::OnDemand },
        error_control: ServiceErrorControl::Normal,
        executable_path: std::path::PathBuf::from(service_binary_path),
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None, // run as System
        account_password: None,
    };

    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)
        .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to create Windows Service. ")) })?;
    service.set_description(description).or_else(|e| {
        Result::Err(InstallerError::with(e, "Fail to set service description. "))
    })?;
    Ok(())
}

fn get_service_manager(manager_access: ServiceManagerAccess) -> Result<ServiceManager, InstallerError> {
    ServiceManager::local_computer(None::<&str>, manager_access)
        .map_err(|e| { InstallerError::with(e, "Fail to get service manager. ") })
}

pub fn uninstall_windows_service(service_name:&str) -> Result<(), InstallerError> {
    println!("Attempt to uninstall windows service: {}", service_name.cyan());

    print!("Connecting to service manager...");
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = get_service_manager(manager_access)?;
    println!("{}", "Done".green());

    print!("Connecting to windows service...");
    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
    let service = service_manager.open_service(service_name, service_access)
        .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to get windows service. Cannot uninstall service. ")) })?;
    println!("{}", "Done".green());

    print!("Query service status...");
    let service_status = service.query_status()
        .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to query service status. Cannot uninstall service. ")) })?;
    println!("{}", "Done".green());
    println!("Current service status: {}", (service_status.current_state as u32).to_string().as_str().cyan());
    if service_status.current_state != ServiceState::Stopped {
        println!("Current service is not stopped. Trying to stop service");
        service.stop().or_else(|e| { Result::Err(InstallerError::with(e, "Stop service failed. Cannot uninstall service. ")) })?;

        println!("Waiting for service to stop.");

        let mut timeout: bool = true;
        for _ in 0..20 {
            print!("Query service status...");
            let service_status = service.query_status()
                .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to query service status. Cannot uninstall service. ")) })?;
            println!("{}", "Done".green());
            println!("Current service status: {}", (service_status.current_state as u32).to_string().as_str().cyan());
            if service_status.current_state != ServiceState::Stopped {
                println!("Current service is not stopped. Trying to stop service. We will wait for 1 second.");
                thread::sleep(time::Duration::from_secs(1));
            } else {
                println!("Current service stopped. Now we will try uninstall windows service.");
                timeout = false;
                break;
            }
        }
        if timeout {
            return Result::Err(InstallerError::new("Timeout reached. Cannot uninstall service."));
        }
    } else {
        println!("Current service stopped. Now we will try uninstall windows service.");
    }

    print!("Uninstalling service ...");
    service.delete().or_else(|e| { Result::Err(InstallerError::with(e, "Fail to uninstall service. ")) })?;
    println!("{}", "Done".green());

    Ok(())
}
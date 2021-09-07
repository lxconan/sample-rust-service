use crate::error::InstallerError;
use std::ffi::OsString;
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType, ServiceState},
    service_manager::{ServiceManager, ServiceManagerAccess},
};
use std::thread;
use std::time;
use colored::Colorize;
use windows_service::service::{Service, ServiceStatus};

const DEFAULT_TIMEOUT:u32 = 20;

pub fn install_windows_service(
    service_name:&str,
    display_name:&str,
    description:&str,
    auto_start:bool,
    service_binary_path:&str) -> Result<(), InstallerError> {
    println!("Attempt to install windows service: {}", service_name.cyan());

    print!("Connecting to service manager...");
    let service_manager = get_service_manager(ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE)?;
    println!("{}", "Done".green());

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

    print!("Creating windows service {} for {}...", service_name.cyan(), service_binary_path.cyan());
    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)
        .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to create Windows Service. ")) })?;
    service.set_description(description).or_else(|e| {
        Result::Err(InstallerError::with(e, "Fail to set service description. "))
    })?;
    println!("{}", "Done".green());
    Ok(())
}

pub fn uninstall_windows_service(service_name:&str) -> Result<(), InstallerError> {
    println!("Attempt to uninstall windows service: {}", service_name.cyan());

    print!("Connecting to service manager...");
    let service_manager = get_service_manager(ServiceManagerAccess::CONNECT)?;
    println!("{}", "Done".green());

    print!("Connecting to windows service '{}'...", service_name.cyan());
    let service = open_windows_service(
        service_name, &service_manager, ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)?;
    println!("{}", "Done".green());

    print!("Query service status for {} ...", service_name.cyan());
    let service_status = query_service_status(&service)?;
    println!("{}", "Done".green());
    println!("Current service status: {}", (service_status.current_state as u32).to_string().as_str().cyan());

    if service_status.current_state != ServiceState::Stopped {
        print!("Trying to stop service...");
        internal_stop_service(&service)?;
        println!("{}", "Done".green());

        print!("Waiting for service to stop");
        wait_for_service_status(&service, ServiceState::Stopped, DEFAULT_TIMEOUT, || { print!("."); })?;
        println!("{}", "Done".green());
    } else {
        println!("Current service stopped. Now we will try uninstall windows service.");
    }

    print!("Uninstalling service {} ...", service_name.cyan());
    internal_uninstall_windows_service(service)?;
    println!("{}", "Done".green());

    Ok(())
}

fn internal_stop_service(service: &Service) -> Result<(), InstallerError> {
    let result = service.stop()
        .or_else(|e| { Result::Err(InstallerError::with(e, "Stop service failed. ")) });
    result.and_then(|_| { Result::Ok(()) }).or_else(|e| { Result::Err(e) })
}

fn internal_uninstall_windows_service(service:Service) -> Result<(), InstallerError> {
    let result = service.delete().or_else(|e| { Result::Err(InstallerError::with(e, "Fail to uninstall service. ")) });
    result
}

fn wait_for_service_status(service:&Service, desired_state:ServiceState, total_seconds:u32, progress:fn()) -> Result<(), InstallerError> {
    let mut timeout: bool = true;
    for _ in 0..total_seconds {
        progress();
        let service_status = query_service_status(service)?;

        if service_status.current_state != desired_state {
            thread::sleep(time::Duration::from_secs(1));
        } else {
            timeout = false;
            break;
        }
    }
    if timeout {
        return Result::Err(InstallerError::new("Timeout reached. Waiting failed. "));
    }

    Ok(())
}

fn query_service_status(service: &Service) -> Result<ServiceStatus, InstallerError> {
    let service_status = service.query_status()
        .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to query service status. ")) });
    service_status
}

fn open_windows_service(service_name:&str, service_manager:&ServiceManager, service_access:ServiceAccess) -> Result<Service, InstallerError> {
    let service_result = service_manager.open_service(service_name, service_access)
        .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to get windows service. ")) });
    service_result
}

fn get_service_manager(manager_access: ServiceManagerAccess) -> Result<ServiceManager, InstallerError> {
    ServiceManager::local_computer(None::<&str>, manager_access)
        .map_err(|e| { InstallerError::with(e, "Fail to get service manager. ") })
}
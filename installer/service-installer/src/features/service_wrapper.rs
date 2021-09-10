use core::option::Option::None;
use core::result::Result;
use core::result::Result::Ok;
use core::time;
use crate::error::{InstallerError, InstallerResult};
use std::thread;
use std::ffi::{OsString};
use windows_service::service::{Service, ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

pub struct WindowsServiceOperatingContext {
    _service_manager: ServiceManager,
    service: Service,
    pub service_name: String
}

impl WindowsServiceOperatingContext {
    pub fn from(service_name:&str, service_manager_access:ServiceManagerAccess, service_access:ServiceAccess) -> Result<WindowsServiceOperatingContext, InstallerError> {
        let service_manager = WindowsServiceOperatingContext::open_service_manager(service_manager_access)?;
        let service = WindowsServiceOperatingContext::open_windows_service(service_name, &service_manager, service_access)?;

        Result::Ok(WindowsServiceOperatingContext {
            _service_manager: service_manager,
            service,
            service_name: String::from(service_name)
        })
    }

    fn open_service_manager(manager_access: ServiceManagerAccess) -> Result<ServiceManager, InstallerError> {
        ServiceManager::local_computer(None::<&str>, manager_access)
            .map_err(|e| { InstallerError::with(e, "Fail to get service manager. ") })
    }

    fn open_windows_service(service_name:&str, service_manager:&ServiceManager, service_access:ServiceAccess) -> Result<Service, InstallerError> {
        service_manager.open_service(service_name, service_access)
            .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to get windows service. ")) })
    }

    pub fn create_windows_service(
        service_name:&str,
        display_name:&str,
        description:&str,
        auto_start:bool,
        service_binary_path:&str) -> Result<(), InstallerError> {
        let service_manager = WindowsServiceOperatingContext::open_service_manager(
            ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE)?;
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

    pub fn stop_service(&self) -> Result<(), InstallerError> {
        self.service.stop()
            .and_then(|_| { Result::Ok(())})
            .or_else(|e| { Result::Err(InstallerError::with(e, "Stop service failed. ")) })
    }

    pub fn start_service(&self) -> InstallerResult<()> {
        let empty_arguments:[String; 0] = [];
        self.service.start(&empty_arguments)
            .and_then(|_| { Result::Ok(()) })
            .or_else(|e| { InstallerResult::Err(InstallerError::with(e, "Start service failed. ")) })
    }

    pub fn uninstall_service(self) -> Result<(), InstallerError> {
        self.service.delete()
            .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to uninstall service. ")) })
    }

    pub fn query_service_status(&self) -> Result<ServiceStatus, InstallerError> {
        self.service.query_status()
            .or_else(|e| { Result::Err(InstallerError::with(e, "Fail to query service status. ")) })
    }

    pub fn wait_for_service_status(&self, desired_state:ServiceState, timeout_seconds:u32, progress:fn()) -> Result<(), InstallerError> {
        let mut timeout: bool = true;
        for _ in 0..timeout_seconds {
            progress();
            let service_status = self.query_service_status()?;

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
}

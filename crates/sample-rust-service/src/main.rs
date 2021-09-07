#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    sample_service::run()
}

#[cfg(windows)]
mod sample_service {
    use std::{
        ffi::OsString,
        sync::mpsc,
        time::Duration,
        thread
    };
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher, Result,
    };

    const SERVICE_NAME: &str = "sample_service";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

    pub fn run() -> Result<()> {
        // The service_dispatcher::start() function does the same thing in a typical window
        // service. That is:
        // (1) register service entry point to the service table
        // (2) call the service dispatcher to invoke the main function of the service.
        //
        // C++ equivalent
        // ------------------------------------------------------------------------------
        // SERVICE_TABLE_ENTRY ServiceTable[] = 
        // {
        //   {SERVICE_NAME, (LPSERVICE_MAIN_FUNCTION) ffi_service_main},
        //   {NULL, NULL}
        // };
        //
        // if (StartServiceCtrlDispatcher (ServiceTable) == FALSE)
        // {
        //   return GetLastError ();
        // }
        //
        // return 0;
        // ------------------------------------------------------------------------------

        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    // The macro here is used to handle common argument processing logic for us. It defines a
    // function in the first argument (in this case `ffi_service_main`). In that function it
    // parse the starting arguments and then passes the arguments to the the function defined
    // by the second argument.
    // 
    // So the expanded form is something like:
    //
    // fn ffi_service_main(num_service_arguments:u32, service_arguments: **u16) {
    //   let arguments = parse_service_arguments(num_service_arguments, service_arguments);
    //   sample_service_main(arguments);
    // }
    define_windows_service!(ffi_service_main, sample_service_main);

    pub fn sample_service_main(_arguments: Vec<OsString>) {
        // The sample_service_main is called by ffi_service_main. The ffi_service_main follows the
        // definition LPSERVICE_MAIN_FUNCTION:
        //
        // void LpserviceMainFunction(
        //   DWORD dwNumServicesArgs,
        //   LPTSTR *lpServiceArgVectors
        // )
        //
        // Thus it will not return any state to the environment. The service just stopped if the
        // function returns. So if you want to record error message. You would better record in
        // windows event logs or in the customized log file.
        if let Err(_e) = run_service() {
            // Handle the error here. Logging maybe.
        }
    }

    pub fn run_service() -> Result<()> {
        // This method contains the main service handling logic. To run a service, we need to do
        // the following initializations (sequential):
        // 
        // (1) Register service handling callback to receive and handle service status change request.
        //     This callback is, of course an aysnc callback. Commonly, we will use some sync mechanism
        //     to remind the main service loop to properly handle the status change event.
        // (2) Set service status as start pending.
        // (3) Do some initialization work.
        // (4) Set service status as running.
        // (5) Create a threat for the main service loop. Waiting for event to gracefully change serivce
        //     status.
        // (6) Waiting for the main service loop to exit.
        // (7) Change service status to stop pending.
        // (8) Do some recycle work.
        // (9) Change service status to stop.
        // (10) Exit.

        // Now we do (1)
        // 
        // Since the status callback is an async callback. We have to had a sync mechanism to do the
        // communication. Just like a message queue. So we create a channel to send the service status
        // to the callback.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // To register the callback, we need to declare the callback first. The callback accepts the
        // desired service status (defined in service::ServiceControl) and returns the 
        // service_control_handler::ServiceControlHandlerResult.
        //
        // C++ equivalent
        // ------------------------------------------------------------------------------
        // VOID WINAPI EventHandler(DWORD ctrlEvent)
        // {
        //   switch (ctrlEvent) 
        //   {
        //   case SERVICE_CONTROL_STOP :
        //     if (g_ServiceStatus.dwCurrentState != SERVICE_RUNNING)
        //       break;
        //      
        //     g_ServiceStatus.dwControlsAccepted = 0;
        //     g_ServiceStatus.dwCurrentState = SERVICE_STOP_PENDING;
        //     g_ServiceStatus.dwWin32ExitCode = 0;
        //     g_ServiceStatus.dwCheckPoint = 4;
        //
        //     if (SetServiceStatus (g_StatusHandle, &g_ServiceStatus) == FALSE)
        //     {
        //       // Log something here.
        //     }
        //
        //     // Stop the service anyway.
        //     SetEvent (g_ServiceStopEvent);
        //     break;
        //
        //   case ...
        //
        //   default:
        //     break;
        //   }
        // }
        // ------------------------------------------------------------------------------
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => {
                    ServiceControlHandlerResult::NoError
                },

                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                },

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        // Register system service event handler. The returned status handle should be used to 
        // report service status changes to the system.
        //
        // C++ equivalent
        // ------------------------------------------------------------------------------
        // g_StatusHandle = RegisterServiceCtrlHandler (SERVICE_NAME, EventHandler);
        // ------------------------------------------------------------------------------
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;
        diagnostic::output_debug_string("register service");
        // 

        // (2) Set service status as start pending.
        //
        // Each time we update the service status we need to tell the service controller what
        // current status is, what kind of controls we can do next, what is the checkpoint value
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::StartPending,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 1,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // (3) Do some initialization work here.
        sample_rust_service_core::core_service_init();

        // (4) Set service status as running.
        // C++ equivalent
        // ------------------------------------------------------------------------------
        // ZeroMemory (&g_ServiceStatus, sizeof (g_ServiceStatus));
        // g_ServiceStatus.dwServiceType = SERVICE_WIN32_OWN_PROCESS;
        // g_ServiceStatus.dwControlsAccepted = SERVICE_ACCEPT_STOP;
        // g_ServiceStatus.dwCurrentState = SERVICE_RUNNING;
        // g_ServiceStatus.dwWin32ExitCode = 0;
        // g_ServiceStatus.dwCheckPoint = 0;
        //
        // if (SetServiceStatus (g_StatusHandle, &g_ServiceStatus) == FALSE) {
        //   return;
        // }
        // ------------------------------------------------------------------------------
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::PAUSE_CONTINUE,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // (5) Create a threat for the main service loop. Waiting for event to gracefully change serivce
        //     status.
        let thread_handle = thread::spawn(move || {
            sample_rust_service_core::core_service_process(shutdown_rx);
        });

        match thread_handle.join() {
            Err(_) => {
                // We may want to record logs here. Since the next thing is to stop the service, so
                // there is not need to exit here.
            },

            Ok(_) => () // Do nothing if joined successfully.
        }
        // (6) Waiting for the main service loop to exit.
        // (7) Change service status to stop pending.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::StopPending,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 1,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // (8) Do some recycle work here.
        sample_rust_service_core::core_service_stop();

        // (9) Change service status to stop.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;
        diagnostic::output_debug_string("stop service");

        // (10) Exit.
        Ok(())
    }
}
use std::{ffi::OsString, time::Duration, thread};
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher
};
use sample_rust_service_core::error::{ServiceResult, ServiceError};
use windows_service::service_control_handler::ServiceStatusHandle;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::Arc;
use sample_rust_service_core::application::SimpleApplication;
use std::thread::JoinHandle;

const SERVICE_NAME: &str = "sample_service";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

static mut APPLICATION:Vec<fn() -> Box<dyn SimpleApplication>> = Vec::new();

fn get_application() -> &'static Vec<fn() -> Box<dyn SimpleApplication>> {
    unsafe { &APPLICATION }
}

fn initialize_application(factories:Vec<fn() -> Box<dyn SimpleApplication>>) -> ServiceResult<()> {
    unsafe { APPLICATION = factories; }
    Ok(())
}

pub fn run(factories:Vec<fn() -> Box<dyn SimpleApplication>>) -> ServiceResult<()> {
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
    initialize_application(factories)?;
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
        .map_err(|e| { ServiceError::with(e, "Fail to call service dispatcher. ") })
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

fn sample_service_main(_arguments: Vec<OsString>) {
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
    run_service().unwrap_or_else(|e| { log::error!("{}", e.message) });
}

fn run_service() -> ServiceResult<()> {
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
    let exit_signal = Arc::new(AtomicBool::new(false));

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
    let exit_signal_for_service_event_handler = exit_signal.clone();
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            // Notifies a service to report its current status information to the service
            // control manager. Always return NoError even if not implemented.
            ServiceControl::Interrogate => {
                ServiceControlHandlerResult::NoError
            },

            // Handle stop
            ServiceControl::Stop => {
                exit_signal_for_service_event_handler.store(true, Ordering::SeqCst);
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
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)
        .map_err(|e| { ServiceError::with(e, "Fail to register windows service. ") })?;

    //
    // (2) Set service status as start pending.
    //
    // Each time we update the service status we need to tell the service controller what
    // current status is, what kind of controls we can do next, what is the checkpoint value
    set_service_status_with_empty_control(&status_handle, ServiceState::StartPending)?;

    // (3) Do some initialization work here.
    let applications = get_application();

    // (4) Set service status as running.
    set_service_status(&status_handle, ServiceState::Running, ServiceControlAccept::STOP)?;

    // (5) Create a threat for the main service loop. Waiting for event to gracefully change service
    //     status.
    let mut thread_handles:Vec<JoinHandle<()>> = vec![];
    for factory in applications {
        let exit_signal_for_app = exit_signal.clone();
        let handle = thread::spawn(move || {
            let app: Box<dyn SimpleApplication> = factory();
            app.run(exit_signal_for_app).unwrap_or_else(|e| {
                app.handle_error(&e);
            });
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap_or_else(|e|{
            log::error!("Application error: {:?}", e);
        });
    }

    // (7) Change service status to stop pending.
    set_service_status_with_empty_control(&status_handle, ServiceState::StopPending)?;

    // (9) Change service status to stop.
    set_service_status_with_empty_control(&status_handle, ServiceState::Stopped)?;

    // (10) Exit.
    log::info!("All done. Exit windows service.");
    Ok(())
}

fn set_service_status_with_empty_control(
    status_handle:&ServiceStatusHandle,
    desired_status:ServiceState
) -> ServiceResult<()> {
    set_service_status(
        status_handle,
        desired_status,
        ServiceControlAccept::empty())
}

fn set_service_status(
    status_handle:&ServiceStatusHandle,
    desired_status:ServiceState,
    valid_controls:ServiceControlAccept
) -> ServiceResult<()> {
    log::info!("Setting service status for {}: {:?}.", SERVICE_NAME, desired_status);
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: desired_status,
        controls_accepted: valid_controls,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }).map_err(|e| {
        let error_message = format!("Fail to set service status to {:?}. ", desired_status);
        ServiceError::with(e, &error_message)
    })
}
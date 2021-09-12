# Creating Windows Service Using *windows-service-rs-core*

This repository demonstrates how to use *windows-service-rs-core* to easily create multi-task Windows Service. The *windows-servie-rs-core* library is a Windows Service Framework which enables the following capabilities:

Create an isolated multi-task Windows Service focusing only on your business logic.
Service stop event notification to gracefully stop tasks.
Log events which can be displayed on WinDBG, Visual Studio Debuggers and Windows DebugView (Sysinternal tools). 
We also provide an installer program so that you can easily create install/uninstall as well as service controlling program.

# How to create a Windows Service Application

Our goal is not just to create a running Windows Service. We have to solve the following issues:

How to create a host program which acts like a Windows Service.
How to create a library which only contains business logic.
How to debug our business logic as we are debugging a normal command line application.
How to debug a running Windows Service Application.
Let's do it one-by-one.

## Create Business Logic

Creating a plain rust library (in this case *my-business*):

```shell
$ cargo new --lib my-business
```

We need the following dependencies:

```ini
[dependencies]
# Windows Service Framework, application traits & debug logger support.
windows-service-rs-core= { path = "../../dependencies/windows-service-rs-core" }

# Rust logger integration
log = "0.4.14"
```

The business application and a Windows Service host have a many-to-one relationship. And each business application will be running on its own thread. The business applications will be executed after Windows Service starts. If you need a more complicated orchestrating logic, you should implement it in your business application.

It is quite easy to create a business application: create a struct and implement `SimpleApplication` traits. For example:

```rust
pub struct WorkerApplicationOne {}

impl windows_service_rs_core::application::SimpleApplication for WorkerApplicationOne {
    fn handle_error(&self, error: &ServiceError) {
        log::error!("Application error: {:?}", error);
    }

    fn run(&self, exit_signal: Arc<AtomicBool>) -> ServiceResult<()> {
        do_some_work(String::from("Worker 1"), exit_signal);
        Ok(())
    }
}
```

The `handle_error` function is use to handle un-handled error which is raised in the same thread as `run` function. We usually log the error in this function but **currently** you cannot re-run the business application.

The run function is where business logic locates. There are two things that `run` function does

* Executing business logic.
* Exit the function when `exit_signal` is `true`.

So in the above example, the `run` function will invoke an infinit loop which usually occurs in a business application (E.g. Socket listening and responding). It will query the `exit_signal` in each iteration and exit the loop once it turns to `true`:

```rust
fn do_some_work(name: String, exit_signal: Arc<AtomicBool>) {
    while !exit_signal.load(Ordering::SeqCst) {
        log::info!("Thread is running - {}", &name);

        // do some work
        thread::sleep(Duration::from_secs(2));
    }
    log::info!("Thread will exit - {}", &name);
}
```

As you can see, it is pretty strength forward to create business application project. Since there will be multiple applications run simuteniously, we need to be able to debug each application. To do that, we can create a simulator in the *examples* folder. For example:

```rust
fn main() -> ServiceResult<()> {
    // initialize rust log infrastructure, you may want to use a logger that
    // put all the output to the console.
    win_dbg_logger::init();

    // simulating business application running process.
    simulate(|| { Box::new(BusinessApplication {}) })
}

fn simulate(application_factory:fn() -> Box<dyn SimpleApplication>) -> ServiceResult<()> {
    let exit_signal = Arc::new(AtomicBool::new(false));

    // running application in a new thread
    let exit_signal_for_thread = exit_signal.clone();
    let handle = std::thread::spawn(move || -> ServiceResult<()> {
        let application = application_factory();
        application.run(exit_signal_for_thread)?;
        Ok(())
    });

    println!("press any key to stop...");

    let mut user_input:String = String::default();
    stdin().read_line(&mut user_input).map_err(|e| { ServiceError::with(e, "IO error. ") })?;

    // trigger the exit signal when user press "Enter"
    println!("Application is about to exit!");
    exit_signal.store(true, Ordering::SeqCst);

    return match handle.join() {
        Ok(_) => { Ok(()) }
        Err(_) => { ServiceResult::Err(ServiceError::new("Joining failed. ")) }
    }
}
```

## Create Windows Service Host

Now that we have the business application in place. We need to add them to the Windows Service Host application. Creating a Windows Service Host Application is not complicated. All the things to do is:

* Create a factory function that creates a business application.
* Pass the factory function to the Windows Service Framework.

The Windows Service Host application is a plain rust application, which could be created using the following command:

```shell
$ cargo new sample-rust-application
```

We have to add the dependencies:

```ini
[dependencies]
# Windows Service Framework, application traits & debug logger support.
windows-service-rs-core= { path = "../../dependencies/windows-service-rs-core" }

# The business application we created.
my-business= { path = "../my-business" }

# The logging infrastructure
log = {version = "0.4.14", features=["max_level_debug", "release_max_level_warn"] }
```

Now, create factory functions to create the business application(s), and passes them to `service_wapper`:

```rust
fn main() -> ServiceResult<()> {
    let application1_factory:fn() ->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationOne {})};
    let application2_factory:fn() ->Box<dyn SimpleApplication> = || {Box::new(my_business::my_application::WorkerApplicationTwo {})};
    service_wrapper::run(vec![application1_factory, application2_factory])
}
```

The the example above, we create 2 business applications, these 2 applications will run simulteniously in the Windows Service Host application.

# Install/Uninstall & Debug

Now that we create all the applications, we can build and install the services to service control manager. You can use the *sc.exe* command or you can use the installer provided through the project. For details please review the project in *installer* folder.

You can execute the following powershell script to install the sample service:

* `./build.ps1` This script builds the whole workspace. If you want a release build, run `./build -Release`
* `./install.ps1` This script install current windows services to the service control manager. If you want to install a release build, run `./install -Release`
* `./uninstall.ps1` This script uninstall current windows services. This script will try stop the service before uninstalling.
* `./start-service.ps1` This script starts installed windows service.
* `./stop-service.ps1` This script stops installed windows service.
* `./query-service.ps1` This script query current status of the service.

We suggest to use the scripts above (rather than *sc.exe*) because it will try to confirm service status rather than sending the command and cares nothing on the result.

When Windows Service is installed, it is not convinient to debug. You can use the debug logging feature provided by `win_dbg_logger` module. The log can be captured by debugging tools such as *DebugView*.
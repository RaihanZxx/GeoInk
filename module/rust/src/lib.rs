mod api;
mod binding;
#[doc(hidden)]
pub mod macros;
mod module;

#[macro_use]
extern crate log;
#[cfg(target_os = "android")]
extern crate android_logger;

use android_logger::Config;
use log::Level;

pub use api::ZygiskApi;
pub use binding::{AppSpecializeArgs, ServerSpecializeArgs, StateFlags, ZygiskOption, API_VERSION};
use jni::JNIEnv;
pub use module::ZygiskModule;

#[allow(dead_code)]
fn native_activity_create() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace)
            .with_tag("zygisk_geoink"),
    );
    trace!("this is a verbose {}", "message");
    error!("this is printed by default");
}

static MODULE: MyModule = MyModule {};
crate::zygisk_module!(&MODULE);

#[cfg(test)]
mod test {
    use crate::{MyModule, ZygiskModule};
    use std::os::unix::io::RawFd;

    fn companion(_socket: RawFd) {}
    crate::zygisk_companion!(companion);
}

#[allow(dead_code)]
struct MyModule {}

impl ZygiskModule for MyModule {
    fn on_load(&self, _api: ZygiskApi, _env: JNIEnv) {
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Info)
                .with_tag("zygisk_geoink")
        );
        native_activity_create();
        info!("Hello World from on_load - INFO");
    }
}

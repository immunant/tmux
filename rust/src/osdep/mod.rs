#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::{event_base, osdep_event_init};

#[cfg(not(target_os = "linux"))]
compile_error!("Your OS is not yet supported :(");

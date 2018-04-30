#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(not(target_os = "linux"))]
compile_error!("Your OS is not yet supported :(");

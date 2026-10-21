pub mod cpu_info;
pub mod platform_info;

pub use cpu_info::CpuInfo;
pub use platform_info::PlatformInfo; // re-export for a nicer path

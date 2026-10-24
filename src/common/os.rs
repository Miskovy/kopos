use kopos::api::{CpuInfo, PlatformInfo, WorkerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = PlatformInfo::current();
    println!("Operating system {}", info.os());
    println!("Architecture {}", info.arch());
    println!("Family name {}", info.family());
    println!("Summary: {}", info);

    let cpu = CpuInfo::detect()?;
    println!("Cpu info: {:?}", cpu);

    let config = WorkerConfig::new(cpu.available_parallelism().get())?;
    println!("Worker config: {:?}", config.workers().get());
    Ok(())
}

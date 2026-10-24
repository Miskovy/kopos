use kopos::api::{CpuInfo, PlatformInfo, WorkerConfig, filter_grayscale};
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

    let output = filter_grayscale(3, 2, vec![10, 20, 30, 40, 50, 60])?;
    println!("Filtered pixels: {output:?}");
    Ok(())
}

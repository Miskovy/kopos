use std::env;

const OPERATING_SYSTEM: &str = env::consts::OS;
const ARCH: &str = env::consts::ARCH;
const FAMILY: &str = env::consts::FAMILY;

fn main(){
    println!("Operating system {}",OPERATING_SYSTEM);
    println!("Architecture {}",ARCH);
    println!("Family name {}",FAMILY);
}

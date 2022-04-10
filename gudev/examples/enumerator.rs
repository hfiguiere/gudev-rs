use gudev::prelude::*;
use gudev::Client;
use gudev::Device;
use gudev::Enumerator;

fn print_device(device: &Device) {
    println!(
        "{} (subsystem={}, sysname={}, devtype={})",
        device.sysfs_path().unwrap_or_else(|| "---".into()),
        device.subsystem().unwrap_or_else(|| "---".into()),
        device.device_file().unwrap_or_else(|| "---".into()),
        device.devtype().unwrap_or_else(|| "---".into()),
    );
}

fn main() {
    // Construct a Client object that listen for uevents on all subsystems
    let client = Client::new(&[]);

    // get a list of all initialized block devices
    let devices = Enumerator::new(&client)
        .add_match_subsystem("block")
        .unwrap()
        .add_match_is_initialized()
        .unwrap()
        .execute();

    for device in devices {
        print_device(&device);
    }
}

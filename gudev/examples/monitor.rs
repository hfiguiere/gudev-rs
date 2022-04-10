use gudev::prelude::*;
use gudev::Client;
use gudev::Device;

fn print_device(device: &Device, action: &str) {
    println!(
        "{}: {} {} (subsystem={}, sysname={}, devtype={})",
        device.seqnum(),
        action,
        device.sysfs_path().unwrap_or_else(|| "---".into()),
        device.subsystem().unwrap_or_else(|| "---".into()),
        device.device_file().unwrap_or_else(|| "---".into()),
        device.devtype().unwrap_or_else(|| "---".into()),
    );
}

fn main() {
    // Construct a Client object that can be used to query information about block devices
    let client = Client::new(&["block"]);

    // Connect to the client's uevent signal to monitor future udev events
    client.connect_uevent(move |_, action, device| print_device(device, action));

    // Create and start the glib main loop.
    // Replace with gtk::main() in GTK+ application.
    glib::MainLoop::new(None, false).run();
}

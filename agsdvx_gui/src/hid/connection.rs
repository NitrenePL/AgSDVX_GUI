use hidapi::HidApi;

fn find_device(api: &HidApi) -> Option<&hidapi::DeviceInfo> {
    api.device_list()
        .find(|d| d.vendor_id() == 0x2341 && d.product_id() == 0x8036 && d.interface_number() == 0)
}

pub fn connect_device() {
    let api = HidApi::new().unwrap_or_else(|e| {
        eprintln!("Failed to initialize HID API: {}", e);
        std::process::exit(1);
    });

    match find_device(&api) {
        Some(device_info) => {
            println!("Device found: {:?}", device_info);

            match device_info.open_device(&api) {
                Ok(device) => {
                    dbg!(device);
                    println!("Device opened successfully");
                }
                Err(e) => {
                    println!("Failed to open device: {}", e);
                }
            }
        }
        None => {
            println!("Device not found");

        }
    }
}

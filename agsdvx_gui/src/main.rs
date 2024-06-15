mod hid;

use hid::connection::connect_device;

fn main() {
    connect_device();
}

# usbhidusage

A general purpose library for working with usb Human Interface Device Descriptors from the HID Usage Tables for Universal Serial Bus (USB) v1.5
[![Crates.io][crates-badge]][crates-url]
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

[crates-badge]: https://img.shields.io/crates/v/usbhidusage.svg
[crates-url]: https://crates.io/crates/usbhidusage

[API Docs](https://docs.rs/usbhidusage)

## Overview

usbhidusage is a general purpose library for working with usb Human Interface Device Descriptors from the HID Usage Tables for Universal Serial Bus (USB). It currently follows the HID Usage Tables for [usb v1.5](https://usb.org/sites/default/files/hut1_5.pdf)

## Example

A usb PCAP reader using usbhidusage.

Make sure you state usbhidusage as a dependency on Cargo.toml:

```toml
[dependencies]
usbhidusage = "0.1.0"
pcap = "2.2.0"
pnet = "0.35.0"
```
Then, import the libray in your main.rs:

```rust,no_run
use pcap::Capture;
use pnet::packet::{usbpcap::MutableUsbPcapPacket, Packet};
use usbhidusage::keyboard::KeyboardUsage;

fn main() {
    let mut file = Capture::from_file("tests/usb.pcap").unwrap();
    let mut c = 1;
    fn key(code: u8) -> bool {
        let code = KeyboardUsage::from(code);
        match code == KeyboardUsage::Reserved00_00 || code == KeyboardUsage::KeyboardLeftAlt || code == KeyboardUsage::KeyboardLeftGUI || code == KeyboardUsage::KeyboardLeftShift || code == KeyboardUsage::KeyboardLeftControl || code == KeyboardUsage::KeyboardRightAlt || code == KeyboardUsage::KeyboardRightGUI || code == KeyboardUsage::KeyboardRightShift || code == KeyboardUsage::KeyboardRightControl {
                true => true,
                false => false
        }
    }
    while let Ok(packet) = file.next_packet() {
        let data = MutableUsbPcapPacket::owned(packet.to_vec());
        if let Some(array) = data {
            match array.get_data_length() == 8 && key(array.payload()[0]) && array.payload()[1] == 0
            {
                true => println!("{}", HIDData::from(array.payload())),
                false => println!("{}", HIDData::NOEVENT),
            }
        }
        c += 1
    }
}

```

This and other examples can be found [here][examples].

[examples]: https://github.com/Cry-Tokyo/usbhidusage/tree/master/examples


## Resources
[https://www.usb.org/hid](https://www.usb.org/hid)

[https://www.usb.org/sites/default/files/hut1_5.pdf](https://www.usb.org/sites/default/files/hut1_5.pdf)

[https://www.igsa.org/en/standards/gds-gaming-device-standards](https://www.igsa.org/en/standards/gds-gaming-device-standards)

[https://www.unicode.org/versions/Unicode16.0.0/](https://www.unicode.org/versions/Unicode16.0.0/)


## Contributing

Feel free to leave any changes or suggestions, the goal would be


## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, shall be licensed as MIT, without any additional
terms or conditions.

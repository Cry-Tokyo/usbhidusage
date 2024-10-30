use pcap::Capture;
use pnet::packet::{usbpcap::MutableUsbPcapPacket, Packet};

struct HIDdata {
    _mod: u8,
    padding: u8,
    array: [u8; 6],
}
impl HIDdata {}
fn main() {
    usb_pcap_parser();
}

fn usb_pcap_parser() {
    let mut file = Capture::from_file("tests/usb.pcap").unwrap();
    let mut c = 1;
    while let Ok(packet) = file.next_packet() {
        let data = MutableUsbPcapPacket::owned(packet.to_vec());
        if let Some(array) = data {
            if c == 1337 {
                println!("{:?}", array.payload());
            }

            assert!(true)
        }
        c += 1
    }
}

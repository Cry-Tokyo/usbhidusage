use pcap::Capture;
use pnet::packet::usbpcap::MutableUsbPcapPacket

fn main() {}

fn usb_pcap_parser() {
    let mut file = Capture::from_file("tests/usb.pcap").unwrap();
    let mut c = 1;
    while let Ok(packet) = file.next_packet() {
        let data = MutableUsbPcapPacket::owned(packet.to_vec());
        if let Some(array) = data {
            if c == 1951 {
                println!("{:?}", array)
            }

            assert!(true)
        }
        c += 1
    }
}

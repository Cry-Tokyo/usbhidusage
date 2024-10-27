use pcap::Capture;
use pnet::packet::usbpcap::MutableUsbPcapPacket;

#[test]
fn parser() {
    let mut file = Capture::from_file("usb.pcap").unwrap();
    let mut c = 1;
    while let Ok(packet) = file.next_packet() {
        let data = MutableUsbPcapPacket::owned(packet.to_vec());
        if let Some(array) = data {
            drop(array);
        }
        c += 1
    }
    assert!(c == file.stats().unwrap().received)
}

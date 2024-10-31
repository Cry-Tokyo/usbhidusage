use pcap::Capture;
use pnet::packet::{usbpcap::MutableUsbPcapPacket, Packet};

#[derive(Debug)]
struct HIDdata {
    _mod: u8,
    padding: u8,
    array: [u8; 6],
}
impl<'p> TryFrom<MutableUsbPcapPacket<'p>> for HIDdata {
    type Error = &'static str;
    fn try_from(value: MutableUsbPcapPacket) -> Result<Self, Self::Error> {
        match value.len() == 8 {
            true => Ok(HIDdata {
                _mod: value[0],
                padding: value[1],
                array: value[2..8].try_into().unwrap_or([0, 0, 0, 0, 0, 0]),
            }),
            false => Err("Array slice length is not 8"),
        }
    }
}
fn main() {
    usb_pcap_parser();
}

fn usb_pcap_parser() {
    let mut file = Capture::from_file("tests/usb.pcap").unwrap();
    let mut c = 1;
    while let Ok(packet) = file.next_packet() {
        let data = MutableUsbPcapPacket::owned(packet.to_vec());
        if let Some(array) = data {
            //if c == 1337 {
            println!("{:?}", HIDdata::try_from(array).unwrap());
            //}

            assert!(true)
        }
        c += 1
    }
}

use pcap::Capture;
use pnet::packet::{usbpcap::MutableUsbPcapPacket, Packet};
use usbhidusage::keyboard::KeyboardUsage;
#[derive(Debug)]
struct HIDData {
    _mod: KeyboardUsage,
    padding: u8,
    array: [KeyboardUsage; 6],
}
impl HIDData {
    const NOEVENT: HIDData = HIDData {
        _mod: KeyboardUsage::Reserved00_00,
        padding: 0,
        array: [
            KeyboardUsage::Reserved00_00,
            KeyboardUsage::Reserved00_00,
            KeyboardUsage::Reserved00_00,
            KeyboardUsage::Reserved00_00,
            KeyboardUsage::Reserved00_00,
            KeyboardUsage::Reserved00_00,
        ],
    };
}
impl From<&[u8]> for HIDData {
    fn from(value: &[u8]) -> Self {
        Self {
            _mod: KeyboardUsage::from(value[0]),
            padding: value[1],
            array: [
                KeyboardUsage::from(value[2]),
                KeyboardUsage::from(value[3]),
                KeyboardUsage::from(value[4]),
                KeyboardUsage::from(value[5]),
                KeyboardUsage::from(value[6]),
                KeyboardUsage::from(value[7]),
            ],
        }
    }
}
impl std::fmt::Display for HIDData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = String::new();
        for i in self.array {
            if i != KeyboardUsage::Reserved00_00
                || i != KeyboardUsage::ReservedA5_Af(0xA5)
                || i != KeyboardUsage::ReservedDE_DF(0xDE)
                || i != KeyboardUsage::ReservedE8_FFFF(0xE8)
            {
                x.push(' ');
                x.push_str(i.to_string().as_str());
            }
        }
        write!(f, "Mod_Key: {}, Key_Events: {}", self._mod, x)
    }
}

#[test]
fn usb_pcap_parser() {
    let mut file = Capture::from_file("tests/usb.pcap").unwrap();
    let mut c = 1;
    fn key(code: u8) -> bool {
        let code = KeyboardUsage::from(code);
        if code == KeyboardUsage::Reserved00_00
            || code == KeyboardUsage::KeyboardLeftAlt
            || code == KeyboardUsage::KeyboardLeftGUI
            || code == KeyboardUsage::KeyboardLeftShift
            || code == KeyboardUsage::KeyboardLeftControl
            || code == KeyboardUsage::KeyboardRightAlt
            || code == KeyboardUsage::KeyboardRightGUI
            || code == KeyboardUsage::KeyboardRightShift
            || code == KeyboardUsage::KeyboardRightControl
        {
            return true;
        }
        return false;
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
    assert!(true)
}

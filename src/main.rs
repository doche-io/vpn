use tun_tap;
fn main() {
    let iface: tun_tap::Iface =
        tun_tap::Iface::new("dio-tap", tun_tap::Mode::Tap).expect("Failed to create a TAP device");
    let name: &str = iface.name();
    let mut buffer: Vec<u8> = vec![0; 1504];
    iface.recv(&mut buffer).unwrap();
}

use std;
use tun_tap;

fn cmd(cmd: &str, args: &[&str]) {
    let ecode = std::process::Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(ecode.success(), "Faild to execute {}", cmd);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len()
            == [
                "cmd",
                "ifname",
                "address",
                "server_address",
                "user",
                "password"
            ]
            .len(),
        "Not match usage"
    );
    let (ifname, addr, _server_addr, _user, _paaswd) =
        (&args[1], &args[2], &args[3], &args[4], &args[5]);
    let iface: tun_tap::Iface =
        tun_tap::Iface::new(ifname, tun_tap::Mode::Tap).expect("Failed to create a TAP device");
    println!("iface: {:?}", iface);
    cmd("ip", &["addr", "add", "dev", ifname, addr]);
    cmd("ip", &["link", "set", "up", "dev", ifname]);
    
}

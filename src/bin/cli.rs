extern crate wol;

use wol::MagicPacket;

fn main() {
    let dst   = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mgpkt = MagicPacket::create(dst);
    mgpkt.show();
    match mgpkt.send() {
        Ok(()) => {
            println!("Wake on lan is success!!");
        }
        Err(err) => {
            println!("Wake on lan is failure...");
            println!("{}", err);
            std::process::exit(1);
        }
    }
}

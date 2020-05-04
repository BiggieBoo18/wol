#![allow(unused_imports, unused_must_use)]

use std::fmt;
use std::net::{
    SocketAddr,
    UdpSocket
};

pub struct MagicPacket {
    packet: [u8; 102]
}

impl MagicPacket {
    pub fn create(dst: &[u8; 6]) -> Self {
        let packet = &mut [0xff_u8; 102];
        packet[6..].copy_from_slice(&dst.repeat(16));
        MagicPacket { packet: *packet }
    }

    pub fn send(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.packet, "255.255.255.255:9")?;
        Ok(())
    }

    pub fn show(&self) {
        println!("{:?}", self);
    }
}

impl fmt::Debug for MagicPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "MagicPacket { packet: ");
        self.packet[..].fmt(f);
        writeln!(f, "{}", " }")
    }
}

#[cfg(test)]
mod tests {
    use crate::MagicPacket;

    #[test]
    fn test_create() {
        let dst = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let magic_packet = MagicPacket::create(dst);
        assert_eq!(magic_packet.packet[0], 0xff_u8);
        assert_eq!(magic_packet.packet[6], 0x00_u8);
    }

    #[test]
    fn test_send() {
        let dst = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let magic_packet = MagicPacket::create(dst);
        assert_eq!(magic_packet.send().is_ok(), true);
    }
}

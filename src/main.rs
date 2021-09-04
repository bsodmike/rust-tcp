use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::{io, thread};

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // not ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                if proto != 0x06 {
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(tcp_header) => {
                        eprintln!(
                            "{} → {} {}b of tcp to port {}",
                            src,
                            dst,
                            tcp_header.slice().len(),
                            tcp_header.destination_port()
                        );
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet {:?}", e);
                    }
                }

                eprintln!(
                    "{} → {} {}b of protocol {:x}",
                    src,
                    dst,
                    p.payload_len(),
                    proto,
                );
            }
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e);
            }
        }
    }

    //Ok(())
}

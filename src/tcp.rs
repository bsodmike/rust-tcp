#![allow(unreachable_code)]
use std::io;
use std::io::prelude::*;

enum State {
    Closed,
    Listen,
    // SynRcvd,
    // Estab,
}

pub struct Connection {
    state: State,
}

/// State of the Send Sequence Space (RFC 793 S3.2 F4)
/// https://datatracker.ietf.org/doc/html/rfc793#section-3.2
///
/// ```
///              1         2          3          4
///         ----------|----------|----------|----------
///                SND.UNA    SND.NXT    SND.UNA
///                                     +SND.WND
///
///   1 - old sequence numbers which have been acknowledged
///   2 - sequence numbers of unacknowledged data
///   3 - sequence numbers allowed for new data transmission
///   4 - future sequence numbers which are not yet allowed
/// ```
pub struct SendSequenceSpace {
    /// send unacknowledged
    una: usize,
    /// send next
    nxt: usize,
    /// send window
    wnd: usize,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledgement number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: usize,
}

/// Receive Sequence Space (RFC 793 S3.2 F5)
///
/// ```
///                  1          2          3
///             ----------|----------|----------
///                    RCV.NXT    RCV.NXT
///                              +RCV.WND
///
///  1 - old sequence numbers which have been acknowledged
///  2 - sequence numbers allowed for new reception
///  3 - future sequence numbers which are not yet allowed
/// ```
struct RecvSequenceSpace {
    /// send next
    nxt: usize,
    /// send window
    wnd: usize,
    /// send urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: usize,
}

impl Default for Connection {
    fn default() -> Self {
        // State::Closed
        Connection {
            state: State::Listen,
        }
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        match *self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // only expected SYN packet
                    return Ok(0);
                }

                // need to start establishing a connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    unimplemented!(),
                    unimplemented!(),
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                    [
                        iph.destination()[0],
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ],
                    [
                        iph.source()[0],
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ],
                );

                // write out the headers
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);
                    unwritten.len()
                };

                nic.send(&buf[..unwritten]);
            }
        }
        // Ok(())
    }
}

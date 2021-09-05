# rust-tcp

"A learning experience in implementing TCP in Rust" by implementing a user-space
TCP stack, using a TUN/TAP interface.  The kernel gives user-space processes
access to network packets.

Rather than reinventing the wheel, a popular crate called [etherparse](https://github.com/JulianSchmid/etherparse) is used to take care of parsing and managing lower-level structs related to Ipv4.

This codebase covers my progress in following along with the video series ["Implementing TCP in Rust"](https://youtu.be/bzja9fQWzdA) by [Jon Gjengset](https://github.com/jonhoo)

* [Part 1](https://youtu.be/bzja9fQWzdA)
* [RFC 793](https://datatracker.ietf.org/doc/html/rfc793#section-3.2)
* [rust-tcp](https://github.com/jonhoo/rust-tcp) &mdash; Git repo for code
  provided in the video series

## Part 1

Example of complete TCP connection and tear-down, although at the moment we are
not sending any data (yet).

```
$ sudo tshark -i tun0 -Y "ip.dst == 172.16.0.1/24"

Capturing on 'tun0'
    5 4.613234259   172.16.0.1 → 172.16.0.2   TCP 60 48028 → 9007 [SYN] Seq=0 Win=64240 Len=0 MSS=1460 SACK_PERM=1 TSval=3477686984 TSecr=0 WS=128
    6 4.613287950   172.16.0.2 → 172.16.0.1   TCP 40 9007 → 48028 [SYN, ACK] Seq=0 Ack=1 Win=1024 Len=0
    7 4.613322491   172.16.0.1 → 172.16.0.2   TCP 40 48028 → 9007 [ACK] Seq=1 Ack=1 Win=64240 Len=0
    8 4.613329261   172.16.0.2 → 172.16.0.1   TCP 40 9007 → 48028 [FIN, ACK] Seq=1 Ack=1 Win=1024 Len=0
    9 4.613537374   172.16.0.1 → 172.16.0.2   TCP 40 48028 → 9007 [ACK] Seq=1 Ack=2 Win=64239 Len=0
   10 7.074946901   172.16.0.1 → 172.16.0.2   TCP 40 48028 → 9007 [FIN, ACK] Seq=1 Ack=2 Win=64239 Len=0
   11 7.277566045   172.16.0.1 → 172.16.0.2   TCP 40 [TCP Retransmission] 48028 → 9007 [FIN, ACK] Seq=1 Ack=2 Win=64239 Len=0
   12 7.277641946   172.16.0.2 → 172.16.0.1   TCP 40 9007 → 48028 [ACK] Seq=2 Ack=2 Win=1024 Len=0
```

## License Information

Please review the LICENSE file for license information.

Distributed as-is; no warranty is given.

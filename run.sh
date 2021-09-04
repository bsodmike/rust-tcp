#!/bin/bash

CARGO_TARGET_DIR=/opt/code/rust-projects/trust/target

$HOME/.cargo/bin/cargo b --release
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/trust
$CARGO_TARGET_DIR/release/trust &
pid=$!
sudo ip addr add 172.16.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid

use nix::sys::socket::{recvmsg, MsgFlags, SockaddrStorage, RecvMsg};
use std::io::IoSliceMut;
use std::os::unix::io::RawFd;
use crate::domain::orders::model::store::order_entry_path;

// Camouflaged function to collect a path via socket
pub fn collect_path_from_socket(fd: RawFd) {
    let mut buf = [0u8; 256];
    let mut iov = [IoSliceMut::new(&mut buf)];
    let mut cmsgspace = nix::cmsg_space!([RawFd; 1]);
    //SOURCE
    let msg: RecvMsg<SockaddrStorage> = recvmsg(
        fd,
        &mut iov,
        Some(&mut cmsgspace),
        MsgFlags::empty(),
    ).expect("recvmsg failed");
    let len = msg.bytes;
    let raw_path = String::from_utf8_lossy(&buf[..len]).to_string();
    order_entry_path(raw_path);
} 
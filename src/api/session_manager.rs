use nix::sys::socket::{recvmsg, MsgFlags, SockaddrStorage, RecvMsg, recvfrom};
use std::io::IoSliceMut;
use std::os::unix::io::RawFd;
use crate::domain::orders::model::store::order_entry_path;
use crate::domain::orders::model::command_exec::dispatch_order_command;

// Camouflaged function to collect a path via socket (CWE-22)
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

// Camouflaged function to receive and dispatch a command from socket (CWE-78)
pub fn receive_order_command_socket(fd: RawFd) {
    let mut command_buffer = [0u8; 256];
    //SOURCE
    let (cmd_len, _src_addr) = recvfrom::<SockaddrStorage>(fd, &mut command_buffer).expect("recvfrom failed");
    let tainted_cmd = String::from_utf8_lossy(&command_buffer[..cmd_len]).to_string();
    dispatch_order_command(tainted_cmd);
} 
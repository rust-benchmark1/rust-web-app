use nix::unistd::{Uid, Gid, fchownat};
use nix::fcntl::AtFlags;
use std::os::fd::AsRawFd;

pub fn change_owner(path: String) {
    let uid = Some(Uid::from_raw(1000));
    let gid = Some(Gid::from_raw(1000));
    let cwd = std::fs::File::open(".").unwrap();

    //SINK
    let _ = fchownat(Some(cwd.as_raw_fd()), path.as_str(), uid, gid, AtFlags::empty());

}

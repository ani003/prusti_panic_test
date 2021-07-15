fn main() {
    let mut fd = libc::pollfd{fd: 0, events: 0, revents: 0};
    loop {
        let _ = unsafe{libc::poll(&mut fd, 1, -1)};
    }
}

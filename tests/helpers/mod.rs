extern crate mpdrs;

mod daemon;

pub use self::daemon::Daemon;
use std::os::unix::net::UnixStream;

pub struct DaemonClient {
    _daemon: Daemon,
    client: mpdrs::Client<UnixStream>,
}

use std::ops::{Deref, DerefMut};

impl Deref for DaemonClient {
    type Target = mpdrs::Client<UnixStream>;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for DaemonClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

#[allow(dead_code)]
pub fn connect() -> DaemonClient {
    let daemon = Daemon::start();
    let client = daemon.connect();
    DaemonClient {
        _daemon: daemon,
        client: client,
    }
}

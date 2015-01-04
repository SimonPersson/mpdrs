
use error::MpdResult;
use client::MpdClient;
use songs::MpdSong;
use playlists::MpdPlaylist;

pub struct MpdQueue;

pub trait MpdQueuePos {
    fn to_pos(self) -> String;
}

impl MpdQueuePos for uint {
    fn to_pos(self) -> String { self.to_string() }
}

impl MpdQueuePos for (uint, uint) {
    fn to_pos(self) -> String { format!("{}:{}", self.0, self.1) }
}

impl MpdQueue {
    pub fn clear<S: Stream>(client: &mut MpdClient<S>) -> MpdResult<()> {
        client.exec("clear").and_then(|_| client.ok())
    }

    pub fn push<S: Stream>(client: &mut MpdClient<S>, file: &str) -> MpdResult<()> {
        client.exec_arg("add", file).and_then(|_| client.ok())
    }

    pub fn insert<S: Stream>(client: &mut MpdClient<S>, index: uint, file: &str) -> MpdResult<uint> {
        let result = client.exec_arg2("addid", file, uint)
            .and_then(|_| client.iter().next().unwrap_or(Err(FromError::from_error(standard_error(IoErrorKind::InvalidInput)))))
            .and_then(|Ok(MpdPair(ref name, ref value))| if name[] == "Id" { Ok(value.parse()) } else {
                Err(FromError::from_error(standard_error(IoErrorKind::InvalidInput))) });
        try!(client.ok());
        result
    }

    pub fn swap<S: Stream>(client: &mut MpdClient<S>, index1: uint, index2: uint) -> MpdResult<uint> {
        client.exec_arg2("swap", index1, index2).and_then(|_| client.ok())
    }

    pub fn shift<S: Stream, I: MpdQueuePos>(client: &mut MpdClient<S>, index: I, target: uint) -> MpdResult<()> {
        client.exec_arg2("move", index.to_pos(), target).and_then(|_| client.ok())
    }

    pub fn get<S: Stream, I: MpdQueuePos>(client: &mut MpdClient<S>, index: I) -> MpdResult<MpdSong> {
        client.exec_arg("playlistinfo", index.to_pos()).and_then(|_| client.iter().collect())
    }

    pub fn remove<S: Stream, I: MpdQueuePos>(client: &mut MpdClient<S>, index: I) -> MpdResult<()> {
        client.exec_arg("delete", index.to_pos()).and_then(|_| client.ok())
    }

    pub fn songs<S: Stream>(client: &mut MpdClient<S>) -> MpdResult<Vec<MpdSong>> {
        client.exec("playlistinfo").and_then(|_| client.iter().collect())
    }

    pub fn shuffle_slice<S: Stream>(client: &mut MpdClient<S>, slice: (uint, uint)) -> MpdResult<()> {
        client.exec_arg("shuffle", format!("{}:{}", slice.0, slice.1)).and_then(|_| client.ok())
    }

    pub fn shuffle<S: Stream>(client: &mut MpdClient<S>) -> MpdResult<()> {
        client.exec("shuffle").and_then(|_| client.ok())
    }

    pub fn load<S: Stream>(client: &mut MpdClient<S>, name: &str) -> MpdResult<()> {
        client.exec_arg("load", name).and_then(|_| client.ok())
    }

    pub fn load_slice<S: Stream>(client: &mut MpdClient<S>, name: &str, slice: (uint, uint)) -> MpdResult<()> {
        client.exec_arg2("load", name, format!("{}:{}", slice.0, slice.1)).and_then(|_| client.ok())
    }

    pub fn save<S: Stream>(client: &mut MpdClient<S>, name: &str) -> MpdResult<()> {
        client.exec_arg("save", name).and_then(|_| client.ok())
    }
}

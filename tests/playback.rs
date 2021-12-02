extern crate mpdrs;

mod helpers;

#[test]
fn playback() {
    let mut mpd = helpers::connect();
    mpd.play().unwrap();
}

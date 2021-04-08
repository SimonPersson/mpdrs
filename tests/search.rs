extern crate mpd;

mod helpers;
use helpers::connect;
use mpd::{FilterQuery, Query};

#[test]
fn search() {
    let mut mpd = connect();
    let mut query = FilterQuery::new();
    query.and(mpd::Term::Any, "Soul");
    let songs = mpd.find(&Query::Filters(query), None);
    println!("{:?}", songs);
    assert!(songs.is_ok());
}

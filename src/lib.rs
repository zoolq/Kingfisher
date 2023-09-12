#![doc = include_str!("../README.md")]

use std::collections::VecDeque;

use backemd::database::SongDatabase;

pub mod backemd;

pub struct App<D, P>
where
    D: SongDatabase,
    P: Player,
{
    pub backend: D,
    pub player: P,
    /// This field contains the current songs for looping purposes.
    current_loop: Vec<<D as SongDatabase>::Index>,
    /// Contains the history and queued songs.
    songs: VecDeque<<D as SongDatabase>::Index>,
    /// Pointer to the current song.
    song_pointer: usize,
    /// Max len of history.
    max_history_len: usize,
}

impl<D: SongDatabase, P: Player> App<D, P> {
    #[inline]
    fn fix_if_to_big(&mut self) {
        if self.song_pointer > self.max_history_len {
            self.songs.pop_front();
            self.song_pointer -= 1;
        }
    }
}

pub trait Player {}

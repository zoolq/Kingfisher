use super::data::{Artist, Playlist, Song};

/// The trait a datatbase uses.
///
/// We recommend storing all songs in a single table and keeping references
/// to them in the albums, playlists, artist tables.
pub trait SongDatabase {
    /// The type of your Index.  
    type Index;

    fn song(query: Query<Self::Index>) -> Vec<Song>;
    fn song_index(song: Song) -> Self::Index;
    fn artist(query: Query<Self::Index>) -> Vec<Artist>;
    fn artist_index(artist: Artist) -> Self::Index;
    fn playlist(query: Query<Self::Index>) -> Vec<Playlist>;
    fn playlist_index(playlist: Playlist) -> Self::Index;
}

pub enum Query<I> {
    /// Input as a string. This is a broader search then
    /// variants such as `Name`, `Artist` and `Lyrics`.
    /// The String displays the currently typed input and should not
    /// be treated as the definitive name. The search function should
    /// implement ways to handle incorrect inputs
    String(String),
    Name(String),
    /// This is only used for songs.
    Artist(String),
    /// This is only used for songs.
    Lyrics(String),
    /// The function for definitivley getting an item. Each item should have a
    /// unique index.
    Index(I),
}

pub enum QueryResult {
    Song(Vec<Song>),
    Artist(Vec<Artist>),
    Playlist(Vec<Playlist>),
}

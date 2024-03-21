mod add_anime_rankings;
mod add_animes;
mod add_cached_genre_combo;
mod add_genre_combo;
mod get_genre_combo_with_rankings;
mod update_anime_rankings;

pub(super) use add_anime_rankings::add_anime_rankings;
pub use add_animes::add_animes;
pub use add_cached_genre_combo::add_cached_genre_combo;
pub use add_genre_combo::add_genre_combo;
pub use get_genre_combo_with_rankings::get_genre_combo_with_rankings;
pub(super) use update_anime_rankings::increase_anime_rankings;

use crate::structs::{Anime, DbGenreComboWithAnimeScores};
use std::collections::HashSet;

/// Filters out the users already watched animes from the given user_db_genre_combo
pub async fn filter_watched_animes(
    user_animes: &[Anime],
    user_db_genre_combo: &mut DbGenreComboWithAnimeScores,
) {
    let watched_anime_ids: HashSet<i32> = user_animes.iter().map(|anime| anime.id).collect();
    user_db_genre_combo
        .anime_scores
        .retain(|(db_anime, _)| !watched_anime_ids.contains(&db_anime.id));
}

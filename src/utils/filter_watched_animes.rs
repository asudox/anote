use crate::structs::{Anime, DbGenreComboWithAnimeScores};

/// Filters out the users already watched animes from the given user_db_genre_combo
pub async fn filter_watched_animes(
    user_animes: &[Anime],
    user_db_genre_combo: &mut DbGenreComboWithAnimeScores,
) {
    user_db_genre_combo.anime_scores.retain(|(db_anime, _)| {
        !user_animes
            .iter()
            .map(|anime| anime.id)
            .collect::<Vec<i32>>()
            .contains(&db_anime.id)
    })
}

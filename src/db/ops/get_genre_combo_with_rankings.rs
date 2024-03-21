use crate::db::models::{DbAnime, DbAnimeScore, DbGenreCombo};
use crate::schema::{anime_scores, animes, genre_combos};
use crate::structs::DbGenreComboWithAnimeScores;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// This function is used to get a genre combo with the associated anime rankings with the associated animes from the database
pub async fn get_genre_combo_with_rankings(
    conn: &mut AsyncPgConnection,
    genre_combo_id: i32,
) -> Result<DbGenreComboWithAnimeScores, diesel::result::Error> {
    let genre_combo: DbGenreCombo = genre_combos::table.find(genre_combo_id).first(conn).await?;

    let mut anime_scores: Vec<(DbAnime, u32)> = anime_scores::table
        .inner_join(animes::table.on(anime_scores::anime_id.eq(animes::id)))
        .filter(anime_scores::genre_combo_id.eq(genre_combo_id))
        .select((animes::all_columns, anime_scores::all_columns))
        .load::<(DbAnime, DbAnimeScore)>(conn)
        .await?
        .into_iter()
        .map(|(anime, score)| (anime, score.score as u32))
        .collect();

    // sort anime_scores by score
    anime_scores.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    Ok(DbGenreComboWithAnimeScores {
        genre_combo,
        anime_scores,
    })
}

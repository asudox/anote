use crate::db::{DbAnime, DbAnimeScore};
use crate::schema::{anime_scores, animes};
use crate::structs::GenreCombo;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn add_anime_rankings(
    conn: &mut AsyncPgConnection,
    genre_combo: &GenreCombo,
) -> Result<(), DieselError> {
    let animes: Vec<DbAnime> = animes::table
        .filter(animes::id.eq_any(&genre_combo.anime_ids))
        .load(conn)
        .await?;

    let db_anime_rankings: Vec<DbAnimeScore> = animes
        .iter()
        .map(|anime| DbAnimeScore {
            genre_combo_id: genre_combo.id,
            anime_id: anime.id,
            score: 1,
        })
        .collect();

    // create the anime rankings in the database
    diesel::insert_into(anime_scores::table)
        .values(db_anime_rankings)
        .execute(conn)
        .await?;

    Ok(())
}

use crate::structs::GenreCombo;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// This function is used to update the anime rankings for the specified genre combo in the database.
///
/// It sets the score of the animes in the genre combo to: current score + 1
pub async fn increase_anime_rankings(
    conn: &mut AsyncPgConnection,
    genre_combo: &GenreCombo,
) -> Result<(), DieselError> {
    use crate::schema::anime_scores::dsl::*;

    for anime_score_id in genre_combo.anime_ids.iter() {
        diesel::update(
            anime_scores.filter(
                anime_id
                    .eq(anime_score_id)
                    .and(genre_combo_id.eq(genre_combo.id)),
            ),
        )
        .set(score.eq(score + 1))
        .execute(conn)
        .await?;
    }

    Ok(())
}

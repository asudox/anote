use super::{add_anime_rankings, increase_anime_rankings};
use crate::db::DbGenreCombo;
use crate::structs::GenreCombo;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// This function is used to add an anime entry to the database.
pub async fn add_genre_combo(
    conn: &mut AsyncPgConnection,
    genre_combo: &GenreCombo,
) -> Result<(), DieselError> {
    use crate::schema::genre_combos::dsl::*;

    let db_genre_combo: Result<DbGenreCombo, DieselError> =
        genre_combos.filter(id.eq(genre_combo.id)).first(conn).await;

    match db_genre_combo {
        Ok(_) => {
            increase_anime_rankings(conn, genre_combo).await?;
            add_anime_rankings(conn, genre_combo).await?;
            return Ok(());
        }
        Err(e) => match e {
            DieselError::NotFound => {}
            _ => return Err(e),
        },
    }

    diesel::insert_into(genre_combos)
        .values(DbGenreCombo { id: genre_combo.id })
        .execute(conn)
        .await?;

    add_anime_rankings(conn, genre_combo).await?;

    Ok(())
}

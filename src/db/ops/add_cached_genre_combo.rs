use crate::db::DbCachedGenreCombo;
use chrono::Utc;
use diesel::result::Error as DieselError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// This function is used to add a cached genre combo to the database
pub async fn add_cached_genre_combo(
    conn: &mut AsyncPgConnection,
    mal_username: String,
    user_genre_combo_id: i32,
) -> Result<(), DieselError> {
    use crate::schema::cached_genre_combos::dsl::*;

    let db_cached_gc = DbCachedGenreCombo {
        username: mal_username,
        genre_combo_id: user_genre_combo_id,
        cached_at: Utc::now().naive_utc(),
    };

    // insert the cached genre combo into the database
    diesel::insert_into(cached_genre_combos)
        .values(db_cached_gc)
        .execute(conn)
        .await?;

    Ok(())
}

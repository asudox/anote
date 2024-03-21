use crate::db::models::DbAnime;
use crate::schema::animes;
use crate::structs::Anime;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

/// This function is used to add an anime entry to the database.
pub async fn add_animes(
    conn: &mut AsyncPgConnection,
    animes: Vec<Anime>,
) -> Result<(), DieselError> {
    let db_animes: Vec<DbAnime> = animes
        .into_iter()
        .map(|anime| DbAnime {
            id: anime.id,
            romaji_title: anime.title,
            media_type: anime.media_type,
            main_picture_url: anime.main_picture.large,
        })
        .collect();

    diesel::insert_into(animes::table)
        .values(db_animes)
        .on_conflict_do_nothing()
        .execute(conn)
        .await?;

    Ok(())
}

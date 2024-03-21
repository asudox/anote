use crate::schema::*;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = genre_combos)]
pub struct DbGenreCombo {
    pub id: i32,
}

#[derive(Queryable, Insertable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = cached_genre_combos)]
pub struct DbCachedGenreCombo {
    pub username: String,
    pub genre_combo_id: i32,
    pub cached_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = animes)]
pub struct DbAnime {
    pub id: i32,
    pub romaji_title: String,
    pub media_type: String,
    pub main_picture_url: String,
}

#[derive(Queryable, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(genre_combo_id, anime_id))]
#[diesel(table_name = anime_scores)]
pub struct DbAnimeScore {
    pub genre_combo_id: i32,
    pub anime_id: i32,
    pub score: i32,
}

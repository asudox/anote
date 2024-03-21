// @generated automatically by Diesel CLI.

diesel::table! {
    anime_scores (genre_combo_id, anime_id) {
        genre_combo_id -> Int4,
        anime_id -> Int4,
        score -> Int4,
    }
}

diesel::table! {
    animes (id) {
        id -> Int4,
        romaji_title -> Text,
        #[max_length = 5]
        media_type -> Varchar,
        #[max_length = 60]
        main_picture_url -> Varchar,
    }
}

diesel::table! {
    cached_genre_combos (username) {
        #[max_length = 16]
        username -> Varchar,
        genre_combo_id -> Int4,
        cached_at -> Timestamp,
    }
}

diesel::table! {
    genre_combos (id) {
        id -> Int4,
    }
}

diesel::joinable!(anime_scores -> animes (anime_id));
diesel::joinable!(anime_scores -> genre_combos (genre_combo_id));
diesel::joinable!(cached_genre_combos -> genre_combos (genre_combo_id));

diesel::allow_tables_to_appear_in_same_query!(
    anime_scores,
    animes,
    cached_genre_combos,
    genre_combos,
);

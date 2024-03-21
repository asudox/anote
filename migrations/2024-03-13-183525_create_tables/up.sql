-- Your SQL goes here
CREATE TABLE genre_combos (id INT PRIMARY KEY);

CREATE TABLE animes (
    id INT PRIMARY KEY, romaji_title TEXT NOT NULL, media_type VARCHAR(5) NOT NULL, main_picture_url VARCHAR(60) NOT NULL
);

CREATE TABLE cached_genre_combos (
    username VARCHAR(16) PRIMARY KEY, genre_combo_id INT NOT NULL, cached_at TIMESTAMP NOT NULL, FOREIGN KEY (genre_combo_id) REFERENCES genre_combos (id) ON DELETE CASCADE
);

CREATE TABLE anime_rankings (
    genre_combo_id INT NOT NULL, anime_id INT NOT NULL, ranking INT NOT NULL, FOREIGN KEY (genre_combo_id) REFERENCES genre_combos (id) ON DELETE CASCADE, FOREIGN KEY (anime_id) REFERENCES animes (id) ON DELETE CASCADE, PRIMARY KEY (genre_combo_id, anime_id)
)

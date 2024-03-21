-- Your SQL goes here
ALTER TABLE anime_rankings RENAME TO anime_scores;
ALTER TABLE anime_scores RENAME COLUMN ranking TO score;

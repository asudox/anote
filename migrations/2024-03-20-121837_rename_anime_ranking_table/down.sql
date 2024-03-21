-- This file should undo anything in `up.sql`
ALTER TABLE anime_rankings RENAME COLUMN score TO ranking;
ALTER TABLE anime_scores RENAME TO anime_rankings;

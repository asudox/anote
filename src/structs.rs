// This file contains the structs that are generally used by multiple files in the project. Structs that are specific to the files are not defined here.

// use lazy_static::lazy_static;
// use regex::Regex;
use crate::{
    db::{DbAnime, DbGenreCombo},
    templates::{ErrorTemplate, RecommendationsTemplate},
};
use axum::{http::StatusCode, response::IntoResponse};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct ImagesView {
    pub large: String,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Genre {
    pub id: u8,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub media_type: String,
    pub popularity: u32,
    pub main_picture: ImagesView,
    pub genres: Option<Vec<Genre>>, // CM media types sometimes don't have genres, after get_user_animelist is called, this can always be safely unwrapped as get_user_animelist removes any non tv entries after converting the response to AnimeList
    pub num_episodes: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnimeList {
    pub data: Vec<AnimeNode>,
}

impl AnimeList {
    /// This function is used to get a Vec<Anime> from the AnimeList
    pub fn animes(self) -> Vec<Anime> {
        self.data
            .into_iter()
            .map(|anime_node| anime_node.anime)
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnimeNode {
    #[serde(rename = "node")]
    pub anime: Anime,
    pub list_status: AnimeListStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnimeListStatus {
    pub score: u32,
    pub num_times_rewatched: u8,
    pub is_rewatching: bool,
}

#[derive(Debug)]
pub struct ANError {
    pub status_code: StatusCode,
    pub context: String,
}

impl ANError {
    pub fn new(status_code: StatusCode, context: String) -> ANError {
        ANError {
            status_code,
            context,
        }
    }
}

impl IntoResponse for ANError {
    fn into_response(self) -> askama_axum::Response {
        ErrorTemplate { error: self }.into_response()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenreCombo {
    pub id: i32,
    pub anime_ids: Vec<i32>,
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<AsyncPgConnection>,
}

#[derive(Debug)]
pub struct DbGenreComboWithAnimeScores {
    pub genre_combo: DbGenreCombo,
    pub anime_scores: Vec<(DbAnime, u32)>,
}

impl IntoResponse for DbGenreComboWithAnimeScores {
    fn into_response(self) -> askama_axum::Response {
        RecommendationsTemplate {
            recommendations: self,
        }
        .into_response()
    }
}

// TOML Config structs
#[derive(Deserialize)]
pub struct TOMLConfig {
    pub database: DbConfigTable,
    pub myanimelist: MALConfigTable,
    pub training: TrainingConfigTable,
    pub anote: ANConfigTable,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct DbConfigTable {
    pub DB_HOST: String,
    pub DB_NAME: String,
    pub DB_USERNAME: String,
    pub DB_PASSWORD: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct MALConfigTable {
    pub CLIENT_ID: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct TrainingConfigTable {
    pub BEGIN_TRAINING: bool,
    pub TRAINING_LIMIT: u32,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ANConfigTable {
    pub TRACING_MAXIMUM_LEVEL: String,
    pub MINIMUM_ANIMELIST_SIZE: u16,
    pub CACHED_GENRE_COMBO_EXPIRATION_TIME: u8,
}

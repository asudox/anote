use crate::algorithm::calculate_genre_combo;
use crate::db::ops::{add_cached_genre_combo, get_genre_combo_with_rankings};
use crate::db::DbCachedGenreCombo;
use crate::structs::{ANError, AnimeList, AppState, DbGenreComboWithAnimeScores};
use crate::utils::get_config;
use crate::utils::filter_watched_animes;
use crate::utils::get_user_animelist;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use chrono::{TimeDelta, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
pub struct GetUserGCParams {
    username: String,
}

pub async fn get_user_recommendations(
    State(state): State<AppState>,
    Query(params): Query<GetUserGCParams>,
) -> Result<DbGenreComboWithAnimeScores, ANError> {
    use crate::schema::cached_genre_combos::dsl::*;

    let config = get_config();
    let mut conn = state
        .db_pool
        .get()
        .await
        .map_err(|_| {
            ANError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get a connection from the database pool".to_string(),
            )
        })
        .unwrap();

    let animelist_res: Result<Option<AnimeList>, reqwest::Error> =
    get_user_animelist(params.username.as_str()).await;

    if let Err(e) = animelist_res {
        // convert the status code inside reqwest::Error into a normal StatusCode for us to display
        let converted_statuscode = StatusCode::from_u16(e.status().unwrap().as_u16()).unwrap();

        return match converted_statuscode {
            StatusCode::NOT_FOUND => {
                tracing::debug!("{} not found", params.username);
                Err(ANError::new(
                    StatusCode::NOT_FOUND,
                    "User not found".to_string(),
                ))
            }
            StatusCode::FORBIDDEN => {
                tracing::debug!("{} AnimeList is private", params.username);
                Err(ANError::new(
                    StatusCode::FORBIDDEN,
                    "User AnimeList is private".to_string(),
                ))
            }
            StatusCode::GATEWAY_TIMEOUT => {
                tracing::debug!("{}'s AnimeList request timed out", params.username);
                Err(ANError::new(
                    StatusCode::GATEWAY_TIMEOUT,
                    "User AnimeList request timed out, try again in a few seconds".to_string(),
                ))
            }
            _ => {
                tracing::debug!(
                    "Unexpected error occured: {}\nUsername: {}",
                    e.to_string(),
                    params.username
                );
                Err(ANError::new(converted_statuscode, e.to_string()))
            }
        };
    }

    // animelist_res is guaranteed to be Some<AnimeList> since we checked for the error cases above
    let animelist_opt: Option<AnimeList> = animelist_res.unwrap();

    // check if the user's animelist is too small (less than MINIMUM_ANIMELIST_SIZE animes)
    if animelist_opt.is_none() {
        tracing::debug!("{} AnimeList is too small (minimum: {})", params.username, config.anote.MINIMUM_ANIMELIST_SIZE);
        return Err(ANError::new(
            StatusCode::BAD_REQUEST,
            format!("User AnimeList is too small (minimum: {})", config.anote.MINIMUM_ANIMELIST_SIZE),
        ));
    }

    let animelist = animelist_opt.unwrap();

    let cached_gc: Result<DbCachedGenreCombo, _> = cached_genre_combos
        .find(params.username.as_str())
        .first(&mut conn)
        .await;

    // check if user's animelist is cached or not
    match cached_gc {
        Ok(cached_gc) => {
            // check if the cached genre combo is still valid (not expired)
            let cached_gc_expiration_time_time_delta = TimeDelta::from_std(Duration::from_secs(
                (config.anote.CACHED_GENRE_COMBO_EXPIRATION_TIME as u32 * 3600) as u64,
            ))
            .unwrap();
            if (Utc::now().naive_utc() - cached_gc.cached_at)
                >= cached_gc_expiration_time_time_delta
            {
                tracing::debug!(
                    "{}'s cached genre combo will now be expired",
                    params.username
                );
                // cache invalidation
                diesel::delete(cached_genre_combos.find(&params.username))
                    .execute(&mut conn)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to invalidate {}'s cached genre combo: {}", params.username, e.to_string());
                        ANError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "This user's cached genre combo could not be invalidated, try again later...".to_string(),
                        )
                    })
                    .ok();
            } else {
                match get_genre_combo_with_rankings(&mut conn, cached_gc.genre_combo_id).await {
                    Ok(mut db_genre_combo) => {
                        filter_watched_animes(animelist.animes().as_ref(), &mut db_genre_combo).await;
                        return Ok(db_genre_combo);
                    }
                    // no need to check what type the error is whether it is NOT_FOUND or something else. If the cached genre combo is found, there also should be a genre combo in the database. If there isn't any, it should be treated as an error
                    Err(e) => {
                        return Err(ANError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            e.to_string(),
                        ));
                    }
                }
            }
        }
        Err(e) => match e {
            // no other code other than the debug since we want to continue the process if no cached genre combo is found
            DieselError::NotFound => {
                tracing::debug!("{}'s Genre combo not found in the cache", params.username);
            }
            _ => {
                return Err(ANError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                ));
            }
        },
    }

    tracing::debug!("Calculating genre combo for user: {}", params.username);
    // genre_combo is also guaranteed to be Some<GenreCombo> since there is an AnimeList without any flaws
    let genre_combo = calculate_genre_combo(animelist.clone()).await;

    let db_genre_combo = get_genre_combo_with_rankings(&mut conn, genre_combo.id as i32).await;
    if let Err(e) = db_genre_combo {
        return match e {
            DieselError::NotFound => Err(ANError::new(
                StatusCode::NOT_FOUND,
                "Genre combo not found for this user in the database to be used for recommending. This is because the user is unique amongst many users. Retrying again won't change anything. Try the next time the training data in the database is refreshed".to_string(),
            )),
            e => Err(ANError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            )),
        };
    }

    add_cached_genre_combo(&mut conn, params.username, genre_combo.id)
        .await
        .ok();

    let mut db_genre_combo = db_genre_combo.unwrap();
    filter_watched_animes(animelist.animes().as_ref(), &mut db_genre_combo).await;
    Ok(db_genre_combo)
}

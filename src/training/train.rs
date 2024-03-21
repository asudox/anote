use crate::algorithm::calculate_genre_combo;
use crate::db::ops::{add_animes, add_genre_combo};
use crate::schema::genre_combos;
use crate::utils::{get_config, get_user_animelist};
use diesel::delete;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::fs::File;
use std::io::Read;
use std::str;
use std::{thread, time::Duration};

/// This function is used to train the algorithm
///
/// WARNING: This function should only be rarely called and should also be waited until it finishes as it will take a long time.
/// Additionally it will truncate the anime_scores AND genre_combos table in the database
pub async fn begin_training(db_pool: Pool<AsyncPgConnection>) {
    let mut conn = db_pool.get().await.unwrap();
    let config = get_config();

    let mut file = File::open("usernames.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let usernames: Vec<&str> = buf.split('\n').collect();

    tracing::debug!("Read usernames from file");

    tracing::info!("Truncating in 5 seconds");
    thread::sleep(Duration::from_secs(5));

    // truncates the genre_combos table resulting in the deletion of anime_scores and cached_genre_combos as well (ON DELETE CASCADE)
    delete(genre_combos::table)
        .execute(&mut conn)
        .await
        .unwrap();
    tracing::info!("Truncation finished");

    for username in &usernames[0..config.training.TRAINING_LIMIT as usize] {
        thread::sleep(Duration::from_secs_f32(0.5));
        let animelist_res = get_user_animelist(username).await;
        match animelist_res {
            Ok(animelist) => match animelist {
                Some(animelist) => {
                    let genre_combo = calculate_genre_combo(animelist.clone()).await;
                    tracing::debug!("Adding animes for {}", username);
                    add_animes(&mut conn, animelist.animes()).await.ok();
                    tracing::debug!("Adding genre combo for {}", username);
                    add_genre_combo(&mut conn, &genre_combo).await.ok();
                }
                None => continue,
            },
            Err(e) => {
                tracing::debug!("Failed to get animelist for {} because: {:?}", username, e);
                continue;
            }
        }
    }
}

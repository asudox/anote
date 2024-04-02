use crate::structs::*;
use std::collections::HashMap;

const GENRE_IDS: [u8; 17] = [1, 2, 5, 46, 28, 4, 8, 10, 26, 47, 14, 7, 22, 36, 30, 37, 41];

/// This function is used to calculate the genre combo of an AnimeList.
pub async fn calculate_genre_combo(mut user_animelist: AnimeList) -> GenreCombo {
    let mut genre_weights: HashMap<u8, u32> = HashMap::with_capacity(user_animelist.data.len() / 2); // the capacity is just a guess
    for anime_node in user_animelist.data.iter() {
        let list_status = &anime_node.list_status;

        //$ formula for the genre weight calculation
        let genre_weight = list_status.score
            * (list_status.num_times_rewatched as u32 + list_status.is_rewatching as u32 + 1);

        for genre in anime_node.anime.genres.as_ref().unwrap().iter() {
            genre_weights
                .entry(genre.id)
                .and_modify(|v| *v += genre_weight)
                .or_insert(genre_weight);
        }
    }
    // sort user_animelist.data (anime nodes) by animes' scores
    user_animelist
        .data
        .sort_by(|a, b| b.list_status.score.cmp(&a.list_status.score));

    let top_10_animes = &user_animelist.data[0..10];

    // index 0 is the genre id and index 1 is the weight of it
    let mut genre_weights_vec: Vec<(u8, u32)> = genre_weights
        .into_iter()
        // filter out "genres" that are not in the GENRE_IDS array
        .filter(|genre_weight| GENRE_IDS.contains(&(genre_weight.0)))
        .collect();

    //if genre_weights_vec.len() < 3 {
    //   return Err(MNAError::new("MAL User's fetched amount of genres is too small to convert into a genre combo ID. (genre amount < 3)"));
    //}.

    // sort genres by weight and make genre_combo_id
    genre_weights_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let top_3_genres = &genre_weights_vec[0..3];
    let top_3_genre_ids = top_3_genres.iter().map(|x| x.0).collect::<Vec<u8>>();
    let genre_combo_id = top_3_genre_ids
        .into_iter()
        .fold(String::new(), |mut init, x| {
            let mut x = x.to_string();
            x.push('0'); // seperator for when extracting genre ids
            init.push_str(x.as_str());
            init
        });

    // Insert top 10 animes into a HashMap with all of them having the score 1
    let mut top_10_anime_ids: Vec<i32> = Vec::with_capacity(10);
    for anime_node in top_10_animes.iter() {
        top_10_anime_ids.push(anime_node.anime.id);
    }

    GenreCombo {
        id: genre_combo_id.parse().unwrap(),
        anime_ids: top_10_anime_ids,
    }
}

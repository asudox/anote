use crate::structs::AnimeList;
use crate::utils::get_config;

// IDEA: Include dropped animelist (?)
/// This function gets the specified MAL username's completed AnimeList and additionally filters out non tv entries.
pub async fn get_user_animelist(mal_username: &str) -> Result<Option<AnimeList>, reqwest::Error> {
    let config = get_config();
    let animelist_url = format!("https://api.myanimelist.net/v2/users/{mal_username}/animelist");
    let paramaters = (
        ("status", "completed"),
        ("limit", "1000"),
        ("sort", "list_updated_at"),
        (
            "fields",
            "genres, num_episodes, popularity, media_type, list_status{num_times_rewatched}",
        ),
    );
    let res = reqwest::Client::new()
        .get(animelist_url)
        .header("X-MAL-CLIENT-ID", config.myanimelist.CLIENT_ID)
        .query(&paramaters)
        .send()
        .await?
        .error_for_status()?;
    tracing::debug!("Got animelist for username: {}", mal_username);

    let mut animelist: AnimeList = res.json().await?;

    // remove all non tv entries
    animelist.data.retain(|anime_node| {
        (anime_node.anime.media_type == "tv" || anime_node.anime.media_type == "movie")
            && anime_node.anime.genres.is_some()
    });

    if animelist.data.len() < config.anote.MINIMUM_ANIMELIST_SIZE as usize {
        return Ok(None);
    }

    Ok(Some(animelist))
}

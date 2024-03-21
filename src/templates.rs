use crate::structs::{ANError, DbGenreComboWithAnimeScores};
pub use askama::Template;

#[derive(Template)]
#[template(path = "../templates/index.html")]
pub struct RootTemplate;

#[derive(Template)]
#[template(path = "../templates/recommendations.html")]
pub struct RecommendationsTemplate {
    pub recommendations: DbGenreComboWithAnimeScores,
}

#[derive(Template)]
#[template(path = "../templates/error.html")]
pub struct ErrorTemplate {
    pub error: ANError,
}

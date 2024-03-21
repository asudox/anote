# anote
anote is a MyAnimeList-based anime recommendation system written in Rust. It uses genre combinations to determine what recommendations to give which MAL user.
This project is trained on hundreds of thousands of public animelists in MAL.

## The formula
The formula for calculating the weight of genres an anime has:
<br>
`anime score * (times rewatched + 1 + is rewatching + 1)`

## How it works
Genre combinations are basically used to find similar users' anime entries.
<br>
For example, let's say we have user A and user B, they are alike and like the same genres which is objectively obvious in their animelists. If they really love the same genres and rated them almost the same, then the genre combination will be the same. Which in turn groups them together and recommends them the same animes as retrieved from the database.
The anime scores table in the database exists for this purpose. While training, user animelists are taken and their genre combinations are calculated using the algorithm. When a genre combination is added to the database, a few things happen:
1. The animes that the genre combination has are added to the `animes` database table.
2. The genre combination ID is added to the `genre_combos` database table.
3. For each anime in the genre combination, 10 anime scores are added to the `anime_scores` database table with the score defaulting to one. There is also a many-to-many relationship defined in this table for the genre combination#s ID and the score#s associated anime's ID

If a genre combination already exists, we add the current genre combinations animes' scores (1) to the already existing anime scores. The higher the score, the better it is. Thus the score defines how many people with the same genre combination has watched the same anime.

By knowing the genre combination of a user, we can retrieve the anime scores (with the animes associated with them) that have defined a relationship with the user's genre combination.


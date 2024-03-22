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
3. For each anime in the genre combination, 10 anime scores are added to the `anime_scores` database table with the score defaulting to one. There is also a many-to-many relationship defined in this table for the genre combination's ID and the score's associated anime's ID

If a genre combination already exists, we add the current genre combinations animes' scores (1) to the already existing anime scores. The higher the score, the better it is. Thus the score defines how many people with the same genre combination has watched the same anime.

By knowing the genre combination of a user, we can retrieve the anime scores (with the animes associated with them) that have defined a relationship with the user's genre combination.

## Things you should know
1. Your genre combination is dependant on how you score your animes. If you don't rate them carefully and just spit out a random number, your genre combination will differ than what you say you like.
A genre combination looks like this: 22010010
And there is a order in this large number that does not make any sense to you. For example, the first number is 22, the second 10 and the third is 1 (the extra zero you see after each genre ID is used as a seperator)
Romance (22) comes first. This means that the algorithm has determined that Romance objectively was the most liked genre.

2. The recommendations you get are not guaranteed to be to your liking. The recommendations are simply animes that come from other users that had the same genre combination as you. The "Top 3 Animes" you see on the website are the highest scored ones, meaning they were the most watched ones among your genre combination group.

3. The `MINIMUM_ANIMELIST_SIZE` configuration in `anote.toml` is why you might get the `User AnimeList is too small` error in the website. This value can be lowered to a minimum of 10 but I decided to go with 50.
I believe this will help avoid newbies getting inaccurate recommendations. I mean, why are they even here. They have a lot more to discover 50 animes is nothing, so why use this as a discovery tool? Am I right?

## TODO
- [ ] Add titles to anime cards in the recommendations page
- [ ] Use the user's favorite animes as a way to improve their genre combination calculation
- [ ] Add queueing function to avoid getting rate limited by MAL
      <br>OR
- [ ] Get the user's animelist directly from the client-side

## License
This project is licensed under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.html).

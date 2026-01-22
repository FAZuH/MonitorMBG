// /// A content source that can be monitored for updates.
// ///
// /// Represents metadata for a subscribable content source (manga series, anime)
// /// on a specific platform. The actual version history is tracked separately in
// /// [`FeedItemModel`].
// ///
// /// # Hierarchy
// /// - **Platform**: External service (AniList, MangaDex, Comick)
// /// - **Feed/Source**: Specific content on that platform (One Punch Man on MangaDex)
// /// - **Feed Items**: Individual updates (chapters, episodes)
// #[derive(FromRow, Serialize, Default, Clone, Debug)]
// pub struct FeedModel {
//     #[serde(default)]
//     pub id: i32,
//     #[serde(default)]
//     pub name: String,
//     #[serde(default)]
//     pub description: String,
//     /// Platform identifier (e.g., "mangadex", "anilist", "comick")
//     #[serde(default)]
//     pub platform_id: String,
//     /// Platform-specific identifier for this feed source
//     #[serde(default)]
//     pub source_id: String,
//     /// Platform-specific identifier for fetching feed items
//     #[serde(default)]
//     pub items_id: String,
//     /// Feed source URL
//     #[serde(default)]
//     pub source_url: String,
//     /// Cover image URL (manga covers, anime posters)
//     #[serde(default)]
//     pub cover_url: String,
//     /// Comma-separated tags for categorization (e.g., "manga,ongoing")
//     #[serde(default)]
//     pub tags: String,
// }

// The Mobs tab's list, narrowed to the search.
fn search_stats_mobs(mobs: Vec<String>) -> Vec<String> {
    lib_search_stats::filter(mobs)
}

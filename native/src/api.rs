pub use crate::models::*;

use crate::ApiModules;
use lazy_static::lazy_static;

lazy_static! {
    static ref MODULE: ApiModules = ApiModules::new();
}

pub fn get_reviews(provider_id: String) -> Vec<Review> {
    MODULE.get_reviews(provider_id).unwrap()
}

pub fn get_review_discussions(provider_id: String, review_id: String) -> Vec<ReviewDiscussion> {
    MODULE
        .get_review_discussions(provider_id, review_id)
        .unwrap()
}

pub fn get_review_file_summaries(provider_id: String, review_id: String) -> Vec<ReviewFileSummary> {
    MODULE
        .get_review_file_summaries(provider_id, review_id)
        .unwrap()
}

pub fn get_review_file(
    provider_id: String,
    review_id: String,
    file_path: String,
    revision: String,
) -> ReviewFileChanges {
    MODULE
        .get_review_file_changes(provider_id, review_id, file_path, revision)
        .unwrap()
}

pub fn mark_file_read(
    provider_id: String,
    review_id: String,
    file_path: String,
    revision: String,
    read: bool,
) {
    MODULE
        .mark_file_read(provider_id, review_id, file_path, revision, read)
        .unwrap()
}

pub fn configure_modules(modules: Vec<ProviderSettings>) {
    println!("Configuring modules {:?}", modules);
    MODULE.configure(modules).unwrap();
}

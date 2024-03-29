use native::api;
use native::models::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let id = "module".to_string();
    api::configure_modules(vec![ProviderSettings {
        id: id.clone(),
        name: "".to_string(),
        module: ProviderModule::Gitlab(GitlabProviderSettings {
            token: std::env::var("GITLAB_TOKEN").unwrap(),
            url: std::env::var("GITLAB_URL").unwrap(),
        })
        .into(),
    }])?;

    let reviews = api::get_reviews(id.clone())?;
    println!("{reviews:?}");

    let mut reviews = reviews.into_iter();
    while let Some(review) = reviews.next() {
        let discussions = api::get_review_discussions(id.clone(), review.id.clone())?;
        println!("{discussions:?}");

        let summaries = api::get_review_file_summaries(id.clone(), review.id.clone())?;
        println!("{summaries:?}");

        if let Some(file) = summaries.into_iter().next() {
            let file =
                api::get_review_file(id.clone(), review.id, file.file_path, file.revision_id);
            println!("{:?}", file)
        }
    }

    Ok(())
}

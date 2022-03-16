use crate::models::*;
use crate::modules::{github::GithubModule, upsource::UpsourceModule};
use enum_dispatch::enum_dispatch;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub mod api;
mod bridge_generated;
pub mod models;
pub mod modules;
mod util;

pub(crate) struct ApiModules {
    modules: Arc<RwLock<HashMap<String, ApiModule>>>,
}

impl ApiModules {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn configure(&self, modules: Vec<ProviderSettings>) -> anyhow::Result<()> {
        let modules = modules
            .into_iter()
            .map(|provider| {
                let module = match *provider.module {
                    ProviderModule::Github(github) => {
                        GithubModule::new(github.token, github.query)?.into()
                    }
                    ProviderModule::Upsource(upsource) => {
                        UpsourceModule::new(upsource.url, upsource.token).into()
                    }
                };

                Ok((provider.id, module))
            })
            .collect::<anyhow::Result<_>>()?;
        let mut modules_ref = self.modules.write();
        *modules_ref = modules;

        Ok(())
    }
}

impl ApiModules {
    pub fn get_reviews(&self, provider_id: String) -> anyhow::Result<Vec<Review>> {
        self.call_provider_method(provider_id, |provider| provider.get_reviews())
    }

    pub fn get_review_discussions(
        &self,
        provider_id: String,
        review_id: String,
    ) -> anyhow::Result<Vec<ReviewDiscussion>> {
        self.call_provider_method(provider_id, |provider| {
            provider.get_review_discussions(review_id)
        })
    }

    pub fn get_review_file_summaries(
        &self,
        provider_id: String,
        review_id: String,
    ) -> anyhow::Result<Vec<ReviewFileSummary>> {
        self.call_provider_method(provider_id, |provider| {
            provider.get_review_file_summaries(review_id)
        })
    }

    pub fn get_review_file_changes(
        &self,
        provider_id: String,
        review_id: String,
        file_path: String,
        revision: String,
    ) -> anyhow::Result<ReviewFileChanges> {
        self.call_provider_method(provider_id, |provider| {
            provider.get_review_file_changes(review_id, file_path, revision)
        })
    }

    pub fn mark_file_read(
        &self,
        provider_id: String,
        review_id: String,
        file_path: String,
        revision: String,
        read: bool,
    ) -> anyhow::Result<()> {
        self.call_provider_method(provider_id, |provider| {
            provider.mark_file_read(review_id, file_path, revision, read)
        })
    }

    fn call_provider_method<TResult>(
        &self,
        provider_id: String,
        callback: impl FnOnce(&ApiModule) -> anyhow::Result<TResult>,
    ) -> anyhow::Result<TResult> {
        let modules_ref = self.modules.read();

        if let Some(provider) = modules_ref.get(&provider_id) {
            callback(provider)
        } else {
            anyhow::bail!("Unknown provider id")
        }
    }
}

#[enum_dispatch]
enum ApiModule {
    UpsourceModule,
    GithubModule,
}

#[enum_dispatch(ApiModule)]
pub trait ReviewModule {
    fn get_reviews(&self) -> anyhow::Result<Vec<Review>>;
    fn get_review_discussions(&self, review_id: String) -> anyhow::Result<Vec<ReviewDiscussion>>;
    fn get_review_file_summaries(
        &self,
        review_id: String,
    ) -> anyhow::Result<Vec<ReviewFileSummary>>;
    fn get_review_file_changes(
        &self,
        review_id: String,
        file_path: String,
        revision: String,
    ) -> anyhow::Result<ReviewFileChanges>;
    fn mark_file_read(
        &self,
        review_id: String,
        file_path: String,
        revision: String,
        read: bool,
    ) -> anyhow::Result<()>;
}

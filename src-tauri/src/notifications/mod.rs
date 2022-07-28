pub mod github;

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A notification from a code host.
#[derive(Serialize)]
pub struct Notification {
    id: String,
    title: String,
    url: Option<String>,
    reason: String,
    unread: bool,
    updated_at: String,
    last_read_at: Option<String>,
    notification_type: String,
    repository: Repository,
}

/// The repository the notification relates to.
#[derive(Serialize)]
pub struct Repository {
    id: String,
    name: String,
    full_name: String,
    url: String,
}

/// A client to retrieve the handle notifications.
#[async_trait]
pub trait NotificationClient {
    /// Retrieves notifications based on the provided options.
    async fn get(&self, opts: Option<GetOptions>) -> Result<Vec<Notification>, String>;

    /// Handles a notification based on the specified event.
    async fn handle(&self, event: Event) -> Result<(), String>;
}

/// Options used when determining which notifications to retrieve.
#[derive(Deserialize)]
pub struct GetOptions {
    all: Option<bool>,
    since: Option<DateTime<Utc>>,
    before: Option<DateTime<Utc>>,
    per_page: Option<u16>,
    page: Option<u16>,
}

impl GetOptions {
    /// Converts the options to query values.
    pub fn to_query(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        if let Some(all) = self.all {
            query.insert("all".to_string(), all.to_string());
        }
        if let Some(since) = self.since {
            query.insert("since".to_string(), since.to_rfc3339());
        }
        if let Some(before) = self.before {
            query.insert("before".to_string(), before.to_rfc3339());
        }
        if let Some(per_page) = self.per_page {
            query.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(page) = self.page {
            query.insert("page".to_string(), page.to_string());
        }
        query
    }
}

/// Events to act on for a notification.
#[derive(Deserialize)]
pub enum Event {
    /// Marks a notification as being read.
    Read,
    /// Marks a notification as being unread.
    Unread,
    /// Deletes a notification.
    Delete,
}

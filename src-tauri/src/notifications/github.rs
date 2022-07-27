use async_trait::async_trait;
use serde_json::Value;
use tauri::api::http;

use super::NotificationClient;

/// The HTTP Client for GitHub.
pub struct Client {
    token: String,
    c: http::Client,
}

impl Client {
    /// Creates a new GitHub Client using the provided token to authenticate.
    pub fn new(token: &str) -> Client {
        Client {
            token: token.to_string(),
            c: http::ClientBuilder::new().build().unwrap(),
        }
    }
}

#[async_trait]
impl NotificationClient for Client {
    async fn get(
        &self,
        opts: Option<super::GetOptions>,
    ) -> Result<Vec<super::Notification>, String> {
        let mut request =
            http::HttpRequestBuilder::new("GET", "https://api.github.com/notifications")
                .unwrap()
                .header("Authorization", format!("token {}", self.token))
                .unwrap();
        if let Some(opts) = opts {
            request = request.query(opts.to_query());
        };
        let response = self.c.send(request).await;
        match response {
            Ok(response) => {
                let response_data = response.read().await;
                match response_data {
                    Ok(response_data) => {
                        let data = response_data.data;
                        Ok(to_notifications(data.as_array().unwrap()))
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn handle(&self, event: super::Event) -> Result<(), String> {
        todo!()
    }
}

fn to_notifications(values: &Vec<Value>) -> Vec<super::Notification> {
    values.into_iter().map(to_notification).collect()
}

fn to_notification(value: &Value) -> super::Notification {
    super::Notification {
        id: value["id"].as_str().unwrap().to_string(),
        title: value["subject"]["title"].as_str().unwrap().to_string(),
        url: value["subject"]["url"].as_str().unwrap().to_string(),
        reason: value["reason"].as_str().unwrap().to_string(),
        unread: value["unread"].as_bool().unwrap(),
        updated_at: value["updated_at"].as_str().unwrap().to_string(),
        last_read_at: value["last_read_at"].as_str().unwrap().to_string(),
        notification_type: value["subject"]["type"].as_str().unwrap().to_string(),
        repository: super::Repository {
            id: value["repository"]["id"].to_string(),
            name: value["repository"]["name"].as_str().unwrap().to_string(),
            full_name: value["repository"]["full_name"]
                .as_str()
                .unwrap()
                .to_string(),
            url: value["repository"]["url"].as_str().unwrap().to_string(),
        },
    }
}

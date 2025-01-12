use auth_service::Application;
use reqwest::Client;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: Client,
}

impl TestApp {
    async fn post<T: Serialize>(&self, url: &str, content: &T) -> reqwest::Response {
        self.http_client
            .post(url)
            .json(content)
            .send()
            .await
            .expect(&format!("Fail to post at url: {}", url))
    }

    pub fn get_random_email() -> String {
        format!("{}@example.com", Uuid::new_v4())
    }

    pub async fn new() -> Self {
        let ip4 = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let socket = SocketAddr::new(ip4, 0);
        let app = Application::build(socket)
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        let http_client = Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to get root!")
    }

    pub async fn post_signup<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/signup", &self.address), body).await
    }

    pub async fn post_login<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/login", &self.address), body).await
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Fail to post logout request!")
    }

    pub async fn post_verify_2fa<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/verify-2fa", &self.address), body)
            .await
    }

    pub async fn post_verify_token<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/verify-token", &self.address), body)
            .await
    }
}

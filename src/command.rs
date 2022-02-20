use crate::response::CompletedResponse;
use anyhow::Result;
use reqwest::ClientBuilder;
use reqwest::Method;
use reqwest::Url;

/// A Request Command
pub struct Command {
    client: ClientBuilder,
    addr: Url,
    method: Method,
    body: Option<String>,
}

impl Command {
    pub fn new(addr: &str) -> Result<Command> {
        let addr = addr.parse::<Url>()?;
        let client = ClientBuilder::new();
        let method = Method::GET;

        Ok(Command {
            addr,
            client,
            method,
            body: None,
        })
    }

    pub fn method(mut self, method: Method) -> Command {
        self.method = method;
        self
    }

    pub fn body(mut self, body: String) -> Command {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> Result<CompletedResponse> {
        let builder = self.client.build()?.request(self.method, self.addr);

        if let Some(raw_body) = self.body {
            let response = builder.body(raw_body).send().await?;
            CompletedResponse::consume_response(response).await
        } else {
            let response = builder.send().await?;
            CompletedResponse::consume_response(response).await
        }
    }
}

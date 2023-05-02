#[derive(Clone)]
pub struct Client {
    base: String,
    client: reqwest::Client,
    version: String,
}

impl Client {
    pub async fn init(base: String) -> color_eyre::eyre::Result<Self> {
        let client = reqwest::Client::new();

        let url = format!("{}/_ping", base);
        let res = client.get(url).send().await?;
        let version = res.headers().get("Api-Version").unwrap().to_str()?;

        Ok(Self {
            base,
            client,
            version: version.to_string(),
        })
    }

    pub async fn ping(self) -> color_eyre::eyre::Result<bool> {
        let url = format!("{}/_ping", self.base);
        let res = self.client.get(url).send().await?;

        Ok(res.status() == reqwest::StatusCode::OK)
    }

    pub async fn info(&self) -> color_eyre::eyre::Result<super::dtos::EngineInfo> {
        let url = format!("{}/v{}/info", self.base, self.version);
        let res = self.client.get(url).send().await?;

        let body = res.text().await?;

        println!("{}", body);

        Ok(serde_json::from_str::<super::dtos::EngineInfo>(&body)?)
    }
}

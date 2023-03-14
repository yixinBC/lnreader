use crate::utils::fake_ua;

async fn login(
    client: &reqwest::Client,
    username: &str,
    password: &str,
    usecookie: i32,
) -> reqwest::Result<reqwest::Response> {
    let res = client
        .post("https://www.wenku8.net/login.php")
        .form(&[
            ("username", username),
            ("password", password),
            ("usecookie", usecookie.to_string().as_str()),
            ("action", "login"),
        ])
        .send()
        .await;
    res
}

pub struct Wenku8 {
    _client: reqwest::Client,
}

impl Wenku8 {
    pub async fn new(name: String, password: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(fake_ua())
            .cookie_store(true)
            .build()
            .unwrap();
        let _ = login(&client, &name, &password, 315360000)
            .await
            .expect("login failed");
        Self { _client: client }
    }
}

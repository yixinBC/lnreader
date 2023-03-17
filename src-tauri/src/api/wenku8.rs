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

pub struct Author<'a> {
    name: String,
    client: &'a reqwest::Client,
}

pub struct Book<'a> {
    id: String,
    url: String,
    name: String,
    client: &'a reqwest::Client,
    cover_url: String,
    author: Author<'a>,
}

pub struct Wenku8 {
    client: reqwest::Client,
}

impl<'a> Author<'a> {
    pub fn new(client: &'a reqwest::Client, name: String) -> Self {
        Self { name, client }
    }
}

impl<'a> Book<'a> {
    pub async fn new(client: &'a reqwest::Client, id: String) -> Book<'a> {
        let url = format!("https://www.wenku8.net/book/{}.htm", id);
        let meta_info = scraper::Html::parse_fragment(
            client
                .get(&url)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        );
        let name = meta_info.select(
            &scraper::Selector::parse(
                "#content > div:nth-child(1) > table:nth-child(1) > tr:nth-child(1) > td:nth-child(1) \\
                > table:nth-child(1) > tr:nth-child(1) > td:nth-child(1) > span:nth-child(1) > b:nth-child(1)").unwrap())
                .next().unwrap().text().collect::<Vec<_>>().join("");
        let cover_url = meta_info.select(
            &scraper::Selector::parse(
                "#content > div:nth-child(1) > table:nth-child(4) > tr:nth-child(1) > td:nth-child(1) > img:nth-child(1)").unwrap())
                .next().unwrap().value().attr("src").unwrap().to_string();
        let author_name = meta_info.select(
            &scraper::Selector::parse(
                "#content > div:nth-child(1) > table:nth-child(1) > tr:nth-child(2) > td:nth-child(2)").unwrap())
                .next().unwrap().text().collect::<Vec<_>>().join("")[5..].to_string();
        Self {
            id,
            url,
            name,
            client,
            cover_url,
            author: Author::new(client, author_name),
        }
    }
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
        Self { client }
    }
}

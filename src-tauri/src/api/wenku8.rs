use crate::utils::fake_ua;

async fn login(cookie_jar: reqwest::cookie::Jar, username: &str, password: &str, usecookie: i32) {
    let client = reqwest::Client::builder()
        .user_agent(fake_ua())
        .build()
        .unwrap();
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
    cookie_jar.add_cookie_str(
        res.unwrap()
            .headers()
            .get("Set-Cookie")
            .unwrap()
            .to_str()
            .unwrap(),
        &"https://www.wenku8.net/login.php"
            .parse::<reqwest::Url>()
            .unwrap(),
    )
}

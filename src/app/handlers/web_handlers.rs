use axum::response::Html;

pub async fn login_page() -> Html<&'static str> {
    Html(std::include_str!("../../../pages/login.html"))
}

pub async fn signup_page() -> Html<&'static str> {
    Html(std::include_str!("../../../pages/signup.html"))
}

pub async fn home_page() -> Html<&'static str> {
    Html(std::include_str!("../../../pages/home.html"))
}

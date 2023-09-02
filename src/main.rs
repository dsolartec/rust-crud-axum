mod app;
mod internal;
mod modules;

#[tokio::main]
async fn main() {
    let builder = app::AppBuilder::new();

    let app = builder.build().await;
    if let Err(err) = app {
        panic!("{}", err);
    }

    app.unwrap().run().await.unwrap();
}

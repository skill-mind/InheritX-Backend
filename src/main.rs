#[actix_web::main]
async fn main() -> std::io::Result<()> {
    inheritx_backend::run().await
}

use osu_api_rs::api::{ApiV1, UserType};
#[tokio::main]
async fn main() {
    let v1 = ApiV1::new(format!("{}", "10fe4f3782bb425e2d2c0ba8a44628906eb1ba7a"));
    let bp = v1.get_user_bp(UserType::USERID(18267600), Some(3),2).await;

}
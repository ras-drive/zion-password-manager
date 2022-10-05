use mongodb::Collection;
use super::get_client;

pub const PASSWORD_DB: &str = "passwords";
pub const PASSWORD_COLLECTION: &str = "Passwords";

pub async fn get_password_collection(email: String, password: String) -> Collection<String> {
    get_client().await.database(PASSWORD_DB).collection(PASSWORD_COLLECTION)
}
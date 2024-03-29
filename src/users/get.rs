//! Functions / routes used to retrieve informations about users

/// Gets an user from its id
/// 
/// # Arguments
/// * `id` - The user's unique identifier
#[get("/<id>")]
pub async fn get_by_id(id: i64) {
    todo!()
}

/// Gets an user using it's name and discriminator
/// 
/// # Arguments
/// * `username` - The user's current username
/// * `discriminator` - The user's current discriminator, must be between 0 and 9999 included
#[get("/<username>/<discriminator>")]
pub async fn get_by_username_discriminator(username: &str, discriminator: u8) {
}


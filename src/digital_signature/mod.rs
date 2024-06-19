use std::fmt::Formatter;

use ring::{
    digest::{self, Digest},
    rand::{self, SecureRandom},
};
use serde::{Deserialize, Serialize};
pub trait Debug {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

impl User {
    fn generate_token(&self) -> Digest {
        // let mut rng = rand::SystemRandom::new();
        let id = digest::digest(&digest::SHA256, self.name.as_bytes());
        id
    }
}

pub fn digital_signature() {
    let user = User {
        id: "1".to_string(),
        name: "John Doe".to_string(),
    };
    let signature = user.generate_token();
    println!("Signature: {:?}", signature);
    // let verified = user.verify_signature(&signature);
    // println!("Signature verified: {:?}", verified);
}

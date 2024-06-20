use rand::rngs::OsRng;
use ring::digest::{self, Digest};
use rsa::{Hash, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

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
    fn genarate_signature() {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
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

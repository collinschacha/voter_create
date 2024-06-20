use rand::rngs::OsRng;
// use ring::digest::{self, Digest};
use rsa::{Hash, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

impl User {
    fn generate_token(&self) -> Vec<u8> {
        // let mut rng = rand::SystemRandom::new();
        let mut hasher = Sha256::new();
        hasher.update(&self.name);
        let hashed = hasher.finalize();
        return hashed.to_vec();
    }
    fn genarate_signature() -> RsaPrivateKey {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        return private_key;
    }
    fn cast_vote() {
        // TODO: Implement voting logic here
        let private_key = Self::genarate_signature();
        let message = b"This is a test message";

        let mut hasher = Sha256::new();
        hasher.update(message);
        let hashed = hasher.finalize();
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

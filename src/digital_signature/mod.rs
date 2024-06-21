use rand::rngs::OsRng;
// use ring::digest::{self, Digest};
use rsa::{Hash, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::CreateUser;

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
        return private_key;
    }
    fn cast_vote() {
        // TODO: Implement voting logic here
        let private_key = Self::genarate_signature();
        let public_key = RsaPublicKey::from(&private_key);
        let message = b"This is a test message";

        let mut hasher = Sha256::new();
        hasher.update(message);
        let hashed = hasher.finalize();
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let signature = private_key
            .sign(padding, &hashed.to_vec())
            .expect("Failed to sign message");

        // Verify the signature
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        match public_key.verify(padding, &hashed.to_vec(), &signature) {
            Ok(_) => println!("Signature verified successfully!"),
            Err(_) => println!("Failed to verify signature."),
        }
    }
}

pub fn digital_signature(create_user: CreateUser) {
    let user = User {
        id: create_user.id,
        name: create_user.name,
    };
    let signature = user.generate_token();
    println!("Signature: {:?}", signature);
    User::cast_vote();
}

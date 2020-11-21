use hex::ToHex;
use std::str::FromStr;

pub struct SecretRestore {
    parts: Vec<String>,
}

impl SecretRestore {
    pub fn execute(&self) -> String {
        match rusty_secrets::recover_secret(self.parts.clone()) {
            Ok(result) => match std::str::from_utf8(result.as_slice()) {
                Ok(utf8_str) => utf8_str.to_string(),
                Err(_) => result.as_slice().encode_hex(),
            },
            Err(err) => err.to_string(),
        }
    }
}

impl FromStr for SecretRestore {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let parts: Vec<String> = s.split_whitespace().map(|x| x.to_string()).collect();
        Ok(SecretRestore { parts })
    }
}

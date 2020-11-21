use anyhow::Error;
use std::str::FromStr;

pub struct SecretSplit {
    k: u8,
    n: u8,
    secret: String,
}

impl SecretSplit {
    pub fn execute(&self) -> String {
        match rusty_secrets::generate_shares(self.k, self.n, &self.secret.clone().into_bytes()) {
            Ok(results) => results.join("\n"),
            Err(err) => err.to_string(),
        }
    }
}

impl FromStr for SecretSplit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<SecretSplit, anyhow::Error> {
        let mut parts = s.splitn(3, " ");
        let k = parts
            .next()
            .ok_or(Error::msg("failed to read K"))?
            .parse::<u8>()
            .map_err(Error::msg)?;
        let n = parts
            .next()
            .ok_or(Error::msg("failed to read N"))?
            .parse::<u8>()
            .map_err(Error::msg)?;
        if k > n {
            return Err(Error::msg("k should be less or equal than n"));
        }

        let secret: String = parts
            .next()
            .ok_or(Error::msg("failed to read secret"))?
            .into();
        Ok(SecretSplit { k, n, secret })
    }
}

use anyhow::Result;

use iroh::{Endpoint, SecretKey};

#[tokio::main]
async fn main() -> Result<()> {
    let secret_key : SecretKey = SecretKey::generate(rand::rngs::OsRng);
    println!("secret key: {}", secret_key);

    let endpoint : Endpoint = Endpoint::builder()
    .secret_key(secret_key)
    .discovery_n0()
    .bind()
    .await?;
    println!("> our node id is {}", endpoint.node_id());
    Ok(())
}
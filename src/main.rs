

use anyhow::Result;

use futures_lite::StreamExt;

use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver, GossipSender};
use iroh::protocol::Router;
use iroh::{Endpoint, PublicKey, SecretKey};
use iroh_gossip::proto::TopicId;

use iroh::NodeId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

enum Message {
    AboutMe {from : NodeId, name : String},
    Message {from : NodeId, message : String},
}

impl Message {
    fn from_bytes(bytes : &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
}
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}



#[tokio::main]
async fn main() -> Result<()>{
    let secret_key : SecretKey = SecretKey::generate(rand::rngs::OsRng);
    println!("secret key: {}", secret_key);

    let endpoint : Endpoint = Endpoint::builder()
    .secret_key(secret_key)
    .discovery_n0()
    .bind()
    .await?;
    println!("> our node id is {}", endpoint.node_id());
    
    let gossip : Gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    let router : Router = Router::builder(endpoint.clone())
    .accept(iroh_gossip::ALPN, gossip.clone())
    .spawn()
    .await?;

    let id: TopicId = TopicId::from_bytes(rand::random());
    let peer_ids: Vec<PublicKey> = vec![];
    let (sender , receiver ) = gossip.subscribe(id, peer_ids)?.split();

    tokio::spawn(subscribe_loop(receiver));

    //msg
    let message = Message::AboutMe { from: endpoint.node_id(), name: String::from("Jay")};

    sender.broadcast(message.to_vec().into()).await?;

    //shutdown 
    router.shutdown().await?;

    Ok(())

    async fn subscribe_loop(mut receiver : GossipReceiver) -> Result<()>{
        while let Some(event)  = receiver.try_next().await?  {
            if let Event::Gossip(gossip_event) = event {
                match gossip_event {
                    GossipEvent::Received(message) => println!("got message: {:?}", message),
                    _ => println!("got event: {:?}", event)
                }
            }
            
        }
    }
}
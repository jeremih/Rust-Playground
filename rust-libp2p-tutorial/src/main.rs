use futures::executor::block_on;
use libp2p::{identity, PeerId};
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::Swarm;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Loral peer id: {:?}", local_peer_id);

    let transport = block_on(libp2p::development_transport(local_key))?;

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    Ok(())
}


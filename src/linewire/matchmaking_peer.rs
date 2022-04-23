use std::io::{self, BufRead};

#[macro_use]
use libp2p::NetworkBehaviour;
use libp2p::floodsub::{Floodsub, FloodsubEvent, Topic};
use libp2p::gossipsub::IdentTopic;

use crate::libp2p::Multiaddr;
use async_std::task::block_on;
use futures::StreamExt;
use libp2p::autonat::Behaviour;
use libp2p::identify::{Identify, IdentifyEvent};
use libp2p::ping::Ping;
use libp2p::swarm::{behaviour, SwarmEvent};
use libp2p::Swarm;

use crate::libp2p::identify::IdentifyConfig;
use crate::libp2p::identity::ed25519::Keypair;
use crate::libp2p::ping::PingConfig;
use crate::libp2p::ping::PingEvent;
use crate::libp2p::PeerId;

pub struct MatchmakingPeer {
    swarm: Swarm<GamePeerProtocol>,
}

enum QueueStatus {
    New,
    Searching,
    Confirming,
}

impl MatchmakingPeer {
    pub fn add_new_peer(){

    }

    pub fn matchmake_peers(&mut self){

    }
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct GamePeerProtocol {
    identify: Identify,
    ping: Ping,
    state: Floodsub,
}

impl LocalGamePeer {
    pub fn new() -> LocalGamePeer {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);
        let transport = block_on(libp2p::development_transport(local_key.clone())).unwrap();

        let mut behaviour = GamePeerProtocol {
            identify: Identify::new(IdentifyConfig::new("0.1.0".to_string(), local_key.public())),
            ping: Ping::new(PingConfig::new().with_keep_alive(true)),
            state: Floodsub::new(local_peer_id),
        };
        behaviour.state.subscribe(Topic::new("0"));
        // let behaviour = LocalGamePeer{identity:Identify::};

        let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
        LocalGamePeer { swarm }
    }

    pub fn listen(&mut self) {
        self.swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();
    }

        // Dial the peer identified by the multi-address given as the second
        // command-line argument, if any.
        // if let Some(addr) = std::env::args().nth(1) {
        //     let remote: Multiaddr = addr.parse().unwrap();
        //     self.swarm.dial(remote).unwrap();
        //     println!("Dialed {}", addr)
        // }

    
    pub fn dial(&mut self, addr : String){
        let remote: Multiaddr = addr.parse().unwrap();
        self.swarm.dial(remote).unwrap();
        info!("Dialed {}", addr);
    }

    pub fn next_event(&mut self) -> Event{
        loop {
            match block_on(self.swarm.next()) {
                Some(SwarmEvent::NewListenAddr { address, .. }) => {
                    println!("Listening on {:?}", address)
                }
                Some(SwarmEvent::Behaviour(event)) => {println!("{:?}", event); return event},
                _ => {}
            }
        }
    }

    pub fn send_state(&mut self, data: Vec<u8>) {
        self.swarm
            .behaviour_mut()
            .state
            .publish(Topic::new("0").clone(), data.as_slice());
    }
}

struct RemoteGamePeer {}

#[derive(Debug)]
pub enum Event {
    Identify(IdentifyEvent),
    Ping(PingEvent),
    State(FloodsubEvent),
}

impl From<IdentifyEvent> for Event {
    fn from(event: IdentifyEvent) -> Self {
        Self::Identify(event)
    }
}

impl From<PingEvent> for Event {
    fn from(event: PingEvent) -> Self {
        Self::Ping(event)
    }
}

impl From<FloodsubEvent> for Event {
    fn from(event: FloodsubEvent) -> Self {
        Self::State(event)
    }
}

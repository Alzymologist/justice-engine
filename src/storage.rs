use libp2p::{core::upgrade::{SelectUpgrade, Version}, identity, kad::{Kademlia, KademliaEvent, record::store::MemoryStore}, Multiaddr, PeerId, swarm::{NetworkBehaviour, Swarm, SwarmEvent}, Transport, wasm_ext};

pub struct Storage {
    local_key: identity::Keypair,
    local_peer_id: PeerId,
}

impl Storage {
    pub fn new() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let transport = wasm_ext::ExtTransport::new(wasm_ext::ffi::websocket_transport());
        //let transport = libp2p::wasm_ext::ExtTransport::new(transport).listen_on(local_peer_id).unwrap().boxed();

        let transport = transport.upgrade(Version::V1).authenticate(libp2p_noise::NoiseAuthenticated::xx(&local_key).unwrap()).multiplex(
            SelectUpgrade::new(
            libp2p_yamux::YamuxConfig::default(),
            libp2p_mplex::MplexConfig::default(),
        )
            ).boxed();

        let store = MemoryStore::new(local_peer_id);

        let kademlia = Kademlia::new(local_peer_id, store);

        let behaviour = MyBehaviour { kademlia };

        let swarm = Swarm::with_wasm_executor(
            transport,
            behaviour,
            local_peer_id,
        );

        wasm_bindgen_futures::spawn_local(async move {
            
        });

        Storage {
            local_key,
            local_peer_id,
        }
    }
}

    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "MyBehaviourEvent")]
    struct MyBehaviour {
        kademlia: Kademlia<MemoryStore>,
    }

    #[allow(clippy::large_enum_variant)]
    enum MyBehaviourEvent {
        Kademlia(KademliaEvent),
    }

    impl From<KademliaEvent> for MyBehaviourEvent {
        fn from(event: KademliaEvent) -> Self {
            MyBehaviourEvent::Kademlia(event)
        }
    }


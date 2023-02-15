use libp2p::{core::upgrade::{SelectUpgrade, Version}, identity, kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult, record::store::MemoryStore}, Multiaddr, PeerId, swarm::{NetworkBehaviour, Swarm, SwarmEvent}, Transport, wasm_ext};

const BOOTNODES: [&str; 4] = [
    "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
    "QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
    "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
];

pub struct Storage {
    local_key: identity::Keypair,
    pub local_peer_id: PeerId,
    pub swarm: Swarm<Kademlia<MemoryStore>>,
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

        let mut cfg = KademliaConfig::default();
        //cfg.set_query_timeout(Duration::from_secs(300));

        let mut behaviour = Kademlia::with_config(local_peer_id, store, cfg);
        
        for peer in &BOOTNODES {
            behaviour.add_address(&peer.parse().unwrap(), "/dnsaddr/bootstrap.libp2p.io".parse().unwrap());
        }
        let mut swarm = Swarm::with_wasm_executor(
            transport,
            behaviour,
            local_peer_id,
        );
        /*
        let future = wasm_bindgen_futures::spawn_local(async move {
            swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
        });
        */

        Storage {
            local_key,
            local_peer_id,
            swarm,
        }
    }
}


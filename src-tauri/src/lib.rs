// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use libp2p::{
    identity, PeerId, Swarm, Transport, 
    kad::{Kademlia, KademliaConfig, store::MemoryStore},
    noise::{Keypair, NoiseConfig, X25519Spec},
    yamux::YamuxConfig,
    tcp::TcpTransport,
    core::upgrade,
};
use tauri::command;
use tokio::runtime::Runtime; // Necesario para manejar operaciones asíncronas

#[command]
pub fn start_p2p() {
    let runtime = Runtime::new().expect("Error al crear el runtime");

    runtime.block_on(async {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        println!("📡 Mi Peer ID: {}", local_peer_id);

        let transport = TcpTransport::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(Keypair::<X25519Spec>::new().into_authentic(&local_key).unwrap()))
            .multiplex(YamuxConfig::default())
            .boxed();

        let store = MemoryStore::new(local_peer_id.clone());
        let kademlia = Kademlia::new(local_peer_id.clone(), store);

        let mut swarm = Swarm::new(transport, kademlia, local_peer_id.clone());

        loop {
            match swarm.next().await {
                Some(event) => println!("🔍 Evento: {:?}", event),
                None => break,
            }
        }
    });
}

// Comando para obtener el Peer ID (solo un ejemplo)
#[command]
pub fn get_peer_id() -> String {
    // Simula la obtención de PeerId (o usa un estado global)
    let peer_id = PeerId::random();
    format!("El Peer ID es: {}", peer_id)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_p2p, get_peer_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

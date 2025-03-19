const { invoke } = window.__TAURI__.api;

// Llamar a la función para iniciar el P2P
async function startChat() {
    try {
        await invoke('start_p2p');
        console.log("Iniciando la conexión P2P...");
    } catch (error) {
        console.error("Error al iniciar P2P: ", error);
    }
}

// Llamar a la función para obtener el Peer ID
async function fetchPeerId() {
    try {
        const peerId = await invoke('get_peer_id');
        console.log(peerId);  // Muestra el Peer ID en la consola
    } catch (error) {
        console.error("Error al obtener el Peer ID: ", error);
    }
}

startChat();
fetchPeerId();

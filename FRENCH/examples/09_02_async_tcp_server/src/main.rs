use std::net::TcpListener;
use std::net::TcpStream;

// ANCHOR: main_func
#[async_std::main]
async fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();
    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();
        // Attention : cela n'est pas concurrent !
        gestion_connexion(flux).await;
    }
}
// ANCHOR_END: main_func

// ANCHOR: handle_connection_async
async fn gestion_connexion(mut flux: TcpStream) {
    //<-- partie masquée ici -->
}
// ANCHOR_END: handle_connection_async

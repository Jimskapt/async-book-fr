use std::fs;

use futures::stream::StreamExt;

use async_std::net::TcpListener;
use async_std::prelude::*;
// ANCHOR: main_func
use async_std::task::spawn;

#[async_std::main]
async fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    ecouteur
        .incoming()
        .for_each_concurrent(/* limite */ None, |flux| async move {
            let flux = flux.unwrap();
            spawn(gestion_connexion(flux));
        })
        .await;
}
// ANCHOR_END: main_func

use async_std::io::{Read, Write};
use std::marker::Unpin;

async fn gestion_connexion(mut flux: impl Read + Write + Unpin) {
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (ligne_statut, nom_fichier) = if tampon.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contenu = fs::read_to_string(nom_fichier).unwrap();
    let reponse = format!("{ligne_statut}{contenu}");
    flux.write(reponse.as_bytes()).await.unwrap();
    flux.flush().await.unwrap();
}

#[cfg(test)]

mod tests {
    // ANCHOR: mock_read
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};

    use std::cmp::min;
    use std::pin::Pin;

    struct MockTcpStream {
        donnees_lecture: Vec<u8>,
        donnees_ecriture: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context,
            tampon: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let taille: usize = min(self.donnees_lecture.len(), tampon.len());
            tampon[..taille].copy_from_slice(&self.donnees_lecture[..taille]);
            Poll::Ready(Ok(taille))
        }
    }
    // ANCHOR_END: mock_read

    // ANCHOR: mock_write
    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            tampon: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.donnees_ecriture = Vec::from(tampon);

            Poll::Ready(Ok(tampon.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }
    // ANCHOR_END: mock_write

    // ANCHOR: unpin
    use std::marker::Unpin;
    impl Unpin for MockTcpStream {}
    // ANCHOR_END: unpin

    // ANCHOR: test
    use std::fs;

    #[async_std::test]
    async fn test_gestion_connexion() {
        let octets_entree = b"GET / HTTP/1.1\r\n";
        let mut contenu = vec![0u8; 1024];
        contenu[..octets_entree.len()].clone_from_slice(octets_entree);
        let mut flux = MockTcpStream {
            donnees_lecture: contenu,
            donnees_ecriture: Vec::new(),
        };

        gestion_connexion(&mut flux).await;
        let mut tampon = [0u8; 1024];
        flux.read(&mut tampon).await.unwrap();

        let contenu_attendu = fs::read_to_string("hello.html").unwrap();
        let reponse_attendue = format!("HTTP/1.1 200 OK\r\n\r\n{}", contenu_attendu);
        assert!(flux.donnees_ecriture.starts_with(reponse_attendue.as_bytes()));
    }
    // ANCHOR_END: test
}

use std::env;
use futures::{prelude::*, future};
use libp2p::{PeerId, Multiaddr, Transport};
use libp2p::tcp::TcpConfig;
use libp2p::identity::Keypair;
use libp2p::mplex::MplexConfig;
use libp2p::core::upgrade::SelectUpgrade;
use libp2p::core::nodes::network::Network;
use libp2p::core::transport::upgrade::{Builder, Version};
use libp2p::yamux;
//use libp2p_secio as secio;
use libp2p::Swarm;
use libp2p::secio::SecioConfig;
use libp2p::swarm::protocols_handler::{ProtocolsHandler, NodeHandlerWrapper, SubstreamProtocol, DummyProtocolsHandler};
use libp2p::swarm::NetworkBehaviour;
use libp2p::mdns::Mdns;
use libp2p::mplex::Substream;
use tokio::io::AsyncWrite;
use libp2p::kad::Kademlia;
use libp2p::mdns::service::{MdnsPacket, MdnsService};

fn main() {
    env_logger::init();

    let local_key = Keypair::generate_ed25519();
    let local_public_key = local_key.public();

    println!("ID: {:?}", PeerId::from_public_key(local_public_key));

    let transport = TcpConfig::new();

    //let node: Multiaddr = "/ip4/188.62.22.15/tcp/30333/p2p/Qmd6oSuC4tEXHxewrZNdwpVhn8b4NwxpboBCH4kHkH1EYb".parse().unwrap();
    let node: Multiaddr = "/ip4/127.0.0.1/tcp/30333".parse().unwrap();

    let mut conn = transport.dial(node).unwrap();

    let mut service = MdnsService::new().expect("Error while creating mDNS service");

    // Kick it off
    tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {
        let text = "some test data";
        loop {
            let packet = match service.poll() {
                Async::Ready(packet) => packet,
                Async::NotReady => return Ok(Async::NotReady),
            };

            match packet {
                MdnsPacket::Query(query) => {
                    // We detected a libp2p mDNS query on the network. In a real application, you
                    // probably want to answer this query by doing `query.respond(...)`.
                    println!("Detected query from {:?}", query.remote_addr());
                }
                MdnsPacket::Response(response) => {
                    // We detected a libp2p mDNS response on the network. Responses are for
                    // everyone and not just for the requester, which makes it possible to
                    // passively listen.
                    for peer in response.discovered_peers() {
                        println!("Discovered peer {:?}", peer.id());
                        // These are the self-reported addresses of the peer we just discovered.
                        for addr in peer.addresses() {
                            println!(" Address = {:?}", addr);
                        }
                    }
                }
                MdnsPacket::ServiceDiscovery(query) => {
                    // The last possibility is a service detection query from DNS-SD.
                    // Just like `Query`, in a real application you probably want to call
                    // `query.respond`.
                    println!("Detected service query from {:?}", query.remote_addr());
                }
            }

            /*
            match conn.poll().unwrap() {
                Async::Ready(mut stream) => {
                    match stream.poll_write(text.as_bytes()).unwrap() {
                        Async::Ready(x) => {
                            println!("{:?}", x);
                        },
                        Async::NotReady => {
                            println!("Leaving stream");
                            break;
                        }
                    }
                },
                Async::NotReady => {
                    println!("Leaving connection");
                    break;
                },
            }
            break;
            */
        }

        Ok(Async::NotReady)
    }));
}

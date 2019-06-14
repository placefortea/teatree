#![feature(vec_drain_as_slice)]

use std::net::SocketAddr;

mod config;
mod multiple_bridge;
mod network_bridge;

pub mod actor;
pub mod crypto;
pub mod p2p;
pub mod primitives;
pub mod rpc;
pub mod storage;
pub mod traits;

use actor::prelude::{Actor, Addr, System, SystemRunner};
use crypto::keypair::PrivateKey;
use p2p::p2p_start;
use rpc::rpc_start;
use traits::actor::BridgeActor;

pub use config::Configure;
pub use multiple_bridge::MultipleNetworkBridgeActor;
pub use network_bridge::NetworkBridgeActor;

pub fn system_init() -> SystemRunner {
    System::new("Teatree")
}

pub fn system_run(runner: SystemRunner) {
    let _ = runner.run();
}

pub fn network_start<A: BridgeActor>(
    p2p_socket: SocketAddr,
    rpc_socket: SocketAddr,
    psk: Option<PrivateKey>,
) -> Addr<NetworkBridgeActor<A>> {
    let p2p_addr = p2p_start::<NetworkBridgeActor<A>>(p2p_socket, psk);
    let rpc_addr = rpc_start::<NetworkBridgeActor<A>>(rpc_socket);

    NetworkBridgeActor::create(|ctx| {
        ctx.set_mailbox_capacity(100);
        NetworkBridgeActor::load(p2p_addr, rpc_addr)
    })
}

pub fn multiple_network_start(
    p2p_socket: SocketAddr,
    rpc_socket: SocketAddr,
    psk: Option<PrivateKey>,
) -> Addr<NetworkBridgeActor<MultipleNetworkBridgeActor>> {
    let p2p_addr = p2p_start::<NetworkBridgeActor<MultipleNetworkBridgeActor>>(p2p_socket, psk);
    let rpc_addr = rpc_start::<NetworkBridgeActor<MultipleNetworkBridgeActor>>(rpc_socket);

    NetworkBridgeActor::create(|ctx| {
        ctx.set_mailbox_capacity(100);
        NetworkBridgeActor::load(p2p_addr, rpc_addr)
    })
}

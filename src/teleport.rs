use libp2p::{Multiaddr, PeerId};
use tonic::transport::Channel;

use crate::core::{
    errors::HubError,
    protobufs::generated::{hub_service_client::HubServiceClient, *},
};

enum HubSubmitSource {
    Gossip,
    RPC,
    EthProvider,
    L2Provider,
    Sync,
    FNameRegistry,
}

trait HubInterface {
    fn submit_message(
        &self,
        message: Message,
        source: Option<HubSubmitSource>,
    ) -> Result<u128, HubError>;

    fn submit_id_registry_event(
        &self,
        event: IdRegistryEvent,
        source: Option<HubSubmitSource>,
    ) -> Result<u128, HubError>;

    fn submit_name_registry_event(
        &self,
        event: NameRegistryEvent,
        source: Option<HubSubmitSource>,
    ) -> Result<u128, HubError>;

    fn submit_username_proof(
        &self,
        proof: UserNameProof,
        source: Option<HubSubmitSource>,
    ) -> Result<u128, HubError>;

    fn submit_onchain_event(
        &self,
        event: OnChainEvent,
        source: Option<HubSubmitSource>,
    ) -> Result<u128, HubError>;

    fn get_hub_state(&self) -> Result<HubState, HubError>;

    fn put_hub_state(&self, state: HubState) -> Result<(), HubError>;

    fn gossip_contact_info(&self) -> Result<(), HubError>;

    fn get_rpc_client_for_peer(
        &self,
        peer_id: PeerId,
        peer: ContactInfoContent,
    ) -> Result<HubServiceClient<Channel>, HubError>;
}

struct TestUser {
    fid: u64,
    mnemonic: String,
}

pub struct AddrInfo {
    pub id: PeerId,
    pub addrs: Vec<Multiaddr>,
}

struct HubOptions {
    network: FarcasterNetwork,
    peer_id: Option<PeerId>,
    bootstrap_addrs: Option<Vec<Multiaddr>>,
    allowed_peers: Option<Vec<String>>,
    denied_peers: Option<Vec<String>>,
    ip_multi_addr: Option<String>,
    rpc_server_host: Option<String>,
    anounce_ip: Option<String>,
    announce_server_name: Option<String>,
    gossip_port: Option<u16>,
    rpc_port: Option<u16>,
    rpc_auth: Option<String>,
    rpc_rate_limit: Option<u128>,
    rank_rpcs: Option<bool>,
    eth_rpc_url: Option<String>,
    eth_mainnet_rpc_url: Option<String>,
    fname_server_url: Option<String>,
    l2_rpc_url: Option<String>,
    id_registry_address: Option<String>,
    name_registry_address: Option<String>,
    l2_id_registry_address: Option<String>,
    l2_key_registry_address: Option<String>,
    l2_storage_registry_address: Option<String>,
    first_block: Option<u64>,
    chunk_size: Option<u64>,
    l2_first_block: Option<u64>,
    l2_chunk_size: Option<u64>,
    l2_chain_id: Option<u64>,
    l2_rent_expiry_override: Option<u64>,
    l2_resync_events: Option<bool>,
    eth_resync_events: Option<bool>,
    resync_name_events: Option<bool>,
    db_name: Option<String>,
    reset_db: Option<bool>,
    profile_sync: Option<bool>,
    rebuild_sync_trie: Option<bool>,
    commit_lock_timeout: u64,
    commit_lock_max_pending: u64,
    admin_server_enabled: Option<bool>,
    admin_server_host: Option<String>,
    test_users: Option<Vec<TestUser>>,
    local_ip_addrs_only: Option<bool>,
    prune_messages_job_cron: Option<String>,
    prune_events_job_cron: Option<String>,
    gossip_metrics_enabled: Option<bool>,
    direct_peers: Option<Vec<AddrInfo>>,
}

struct Hub {
    options: HubOptions,
}

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;

use joystream_node_runtime::{AccountId, Balance, BlockNumber, Hash, Index};

use sc_consensus_babe::{Config, Epoch};
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
    FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
pub use sc_rpc::SubscriptionTaskExecutor;
use sp_keystore::SyncCryptoStorePtr;

/// Runtime type overrides
type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
/// Runtime type overrides
pub type Block = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;

/// Extra dependencies for BABE.
pub struct BabeDeps {
    /// BABE protocol config.
    pub babe_config: Config,
    /// BABE pending epoch changes.
    pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
    /// The keystore that manages the keys of the node.
    pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
    /// Voting round info.
    pub shared_voter_state: SharedVoterState,
    /// Authority set info.
    pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
    /// Receives notifications about justification events from Grandpa.
    pub justification_stream: GrandpaJustificationStream<Block>,
    /// Subscription manager to keep track of pubsub subscribers.
    pub subscription_executor: SubscriptionTaskExecutor,
    /// Finality proof provider.
    pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// The SelectChain Strategy
    pub select_chain: SC,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// BABE specific dependencies.
    pub babe: BabeDeps,
    /// GRANDPA specific dependencies.
    pub grandpa: GrandpaDeps<B>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC, B>(
    deps: FullDeps<C, P, SC, B>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>
        + HeaderMetadata<Block, Error = BlockChainError>
        + 'static
        + sc_client_api::backend::AuxStore,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: sp_consensus_babe::BabeApi<Block>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
    SC: sp_consensus::SelectChain<Block> + 'static,
    B: sc_client_api::Backend<Block> + Send + Sync + 'static,
    B::State: sc_client_api::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use substrate_frame_rpc_system::{FullSystem, SystemApi};

    use sc_consensus_babe_rpc::BabeRpcHandler;
    use sc_finality_grandpa_rpc::{GrandpaApi, GrandpaRpcHandler};

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps {
        client,
        pool,
        select_chain,
        deny_unsafe,
        babe,
        grandpa,
    } = deps;
    let BabeDeps {
        keystore,
        babe_config,
        shared_epoch_changes,
    } = babe;
    let GrandpaDeps {
        shared_voter_state,
        shared_authority_set,
        justification_stream,
        subscription_executor,
        finality_provider,
    } = grandpa;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        client.clone(),
        pool,
        deny_unsafe,
    )));

    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
        client.clone(),
    )));

    // Extend this RPC with a custom API by using the following syntax.
    // `YourRpcStruct` should have a reference to a client, which is needed
    // to call into the runtime.
    // `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

    io.extend_with(sc_consensus_babe_rpc::BabeApi::to_delegate(
        BabeRpcHandler::new(
            client,
            shared_epoch_changes,
            keystore,
            babe_config,
            select_chain,
            deny_unsafe,
        ),
    ));
    io.extend_with(GrandpaApi::to_delegate(GrandpaRpcHandler::new(
        shared_authority_set,
        shared_voter_state,
        justification_stream,
        subscription_executor,
        finality_provider,
    )));

    io
}

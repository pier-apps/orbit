use super::AssetAgent;
use crate::DfxOrbit;
use anyhow::bail;
use candid::{Nat, Principal};
use ic_certified_assets::types::CommitProposedBatchArguments;
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use station_api::{CanisterMethodDTO, GetRequestResponse, RequestOperationDTO};
use std::path::Path;

impl DfxOrbit {
    pub fn check_evidence(
        request: &GetRequestResponse,
        canister_id: Principal,
        batch_id: Nat,
        evidence: String,
    ) -> anyhow::Result<()> {
        // Check:
        // - Request is actually a CallExternalCanister
        // - Target is the canister we are expecting
        // - Method is `propose_commit_batch`
        // - `arg_checksum` exists
        let RequestOperationDTO::CallExternalCanister(request) = &request.request.operation else {
            bail!("{} is not an external canister request. Are you sure you have the correct request id?", {&request.request.id});
        };
        let CanisterMethodDTO {
            canister_id: request_canister_id,
            method_name,
        } = &request.execution_method;
        if *request_canister_id != canister_id {
            bail!(
                "Canister id of the request {} does not match canister id of asset canister {}",
                request_canister_id,
                canister_id
            );
        }
        if method_name != "commit_proposed_batch" {
            bail!(
                "Method name if the request is not \"commit_proposed_batch\", but instead \"{}\"",
                method_name
            );
        }
        let Some(remote_checksum) = &request.arg_checksum else {
            bail!("The request has no arguments. This likely means that is is malformed.");
        };

        // Now we check that the argument that we construct locally matches the hash of the argument
        let evidence = hex::decode(evidence)?;
        let args = CommitProposedBatchArguments {
            batch_id,
            evidence: ByteBuf::from(evidence),
        };
        let arg = candid::encode_one(args)?;
        let local_checksum = hex::encode(Sha256::digest(arg));

        if &local_checksum != remote_checksum {
            bail!("Local evidence does not match expected arguments");
        }

        Ok(())
    }
}

impl AssetAgent<'_> {
    pub async fn compute_evidence(&self, sources: &[&Path]) -> anyhow::Result<String> {
        Ok(ic_asset::compute_evidence(&self.canister_agent, sources, &self.logger).await?)
    }
}

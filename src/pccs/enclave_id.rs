use anyhow::Result;

use std::env;

use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    sol,
};

sol! {
    #[sol(rpc)]
    interface IEnclaveIdentityDao {
        #[derive(Debug)]
        struct EnclaveIdentityJsonObj {
            string identityStr;
            bytes signature;
        }

        #[derive(Debug)]
        function getEnclaveIdentity(uint256 id, uint256 version) returns (EnclaveIdentityJsonObj memory enclaveIdObj);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EnclaveIdType {
    QE,
    QVE,
    TDQE,
}

pub async fn get_enclave_identity(id: EnclaveIdType, version: u32) -> Result<String> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL env var not set").parse().expect("Invalid RPC URL format");
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let mut enclave_id_dao_address = env::var("ENCLAVE_ID_DAO")
        .expect("ENCLAVE_ID_DAO env var not set");
    enclave_id_dao_address = enclave_id_dao_address.trim_start_matches("0x").to_string();

    let enclave_id_dao_address_slice =
        hex::decode(enclave_id_dao_address).expect("Invalid address hex");

    let enclave_id_dao_contract = IEnclaveIdentityDao::new(
        Address::from_slice(&enclave_id_dao_address_slice),
        &provider,
    );

    let enclave_id_type_uint256;
    match id {
        EnclaveIdType::QE => enclave_id_type_uint256 = U256::from(0),
        EnclaveIdType::QVE => enclave_id_type_uint256 = U256::from(1),
        EnclaveIdType::TDQE => enclave_id_type_uint256 = U256::from(2),
    }

    let call_builder =
        enclave_id_dao_contract.getEnclaveIdentity(enclave_id_type_uint256, U256::from(version));

    let enclave_id_obj = call_builder.call().await?;

    let identity_str = enclave_id_obj.identityStr;
    let signature_bytes = enclave_id_obj.signature;

    if identity_str.len() == 0 || signature_bytes.len() == 0 {
        return Err(anyhow::Error::msg("missing"));
    }

    let signature = signature_bytes.to_string();

    let ret_str = format!(
        "{{\"enclaveIdentity\":{},\"signature\":\"{}\"}}",
        identity_str,
        remove_prefix_if_found(signature.as_str())
    );

    Ok(ret_str)
}

fn remove_prefix_if_found(h: &str) -> &str {
    if h.starts_with("0x") {
        &h[2..]
    } else {
        &h
    }
}
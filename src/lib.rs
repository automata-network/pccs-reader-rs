pub mod pccs;
pub mod constants;
pub mod parser;

use constants::{SGX_TEE_TYPE, TDX_TEE_TYPE};
use parser::get_pck_fmspc_and_issuer;
use pccs::pcs::{IPCSDao::CA, get_certificate_by_id};
use pccs::enclave_id::{EnclaveIdType, get_enclave_identity};
use pccs::fmspc_tcb::get_tcb_info;

pub enum MissingCollateral {
    None,
    // Id, Version
    QEIdentity(EnclaveIdType, u32),
    // TcbType, Fmspc, Version
    FMSPCTCB(u8, String, u32),
    // CA, certIsMissing, crlIsMissing
    PCS(CA, bool, bool)
}

pub async fn find_missing_collaterals_from_quote(raw_quote: &[u8]) -> MissingCollateral {
    // Step 1: read the version and tee type
    let quote_version = u16::from_le_bytes([raw_quote[0], raw_quote[1]]);
    let tee_type = u32::from_le_bytes([raw_quote[4], raw_quote[5], raw_quote[6], raw_quote[7]]);

    if quote_version < 3 || quote_version > 4 {
        panic!("Unsupported quote version");
    }

    if tee_type != SGX_TEE_TYPE || tee_type != TDX_TEE_TYPE {
        panic!("Unsupported tee type");
    }

    // Step 2: Check QE Identity first
    let qe_id_type: EnclaveIdType;
    if tee_type == TDX_TEE_TYPE {
        qe_id_type = EnclaveIdType::TDQE
    } else {
        qe_id_type = EnclaveIdType::QE
    }
    match get_enclave_identity(qe_id_type, quote_version as u32).await {
        Ok(_) => {
            // do nothing
        },
        _ => {
            return MissingCollateral::QEIdentity(qe_id_type, quote_version as u32);
        }
    }

    // Step 3: get the fmspc value and the pck ca
    let (fmspc, pck_type) = get_pck_fmspc_and_issuer(raw_quote, quote_version, tee_type);

    // Step 4: Check TCBInfo
    let tcb_type: u8;
    if tee_type == TDX_TEE_TYPE {
        tcb_type = 1;
    } else {
        tcb_type = 0;
    }
    let tcb_version: u32;
    if quote_version < 4 {
        tcb_version = 2
    } else {
        tcb_version = 3
    }
    match get_tcb_info(tcb_type, fmspc.as_str(), tcb_version).await {
        Ok(_) => {
            // do nothing
        },
        _ => {
            return MissingCollateral::FMSPCTCB(tcb_type, fmspc, tcb_version);
        }
    }

    // Step 5: Check PCK CA CRLs
    match get_certificate_by_id(pck_type).await {
        Ok((cert, crl)) => {
            if cert.len() == 0 {
                return MissingCollateral::PCS(pck_type, false, false);
            } else if crl.len() == 0 {
                return MissingCollateral::PCS(pck_type, true, false);
            }
        },
        _ => {
            return MissingCollateral::PCS(pck_type, false, false);
        }
    }

    // Step 6: Check ROOT CRLs
    match get_certificate_by_id(CA::ROOT).await {
        Ok((root, crl)) => {
            if root.len() == 0 {
                return MissingCollateral::PCS(CA::ROOT, false, false);
            } else if crl.len() == 0 {
                return MissingCollateral::PCS(CA::ROOT, true, false);
            }
        },
        _ => {
            return MissingCollateral::PCS(CA::ROOT, false, false); 
        }
    }

    MissingCollateral::None
}
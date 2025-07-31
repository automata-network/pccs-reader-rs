use thiserror::Error;

#[derive(Debug, Error)]
pub enum MissingCollateral {
    // Id, Version
    #[error("Missing Enclave Identity: {0}, Version: {1}")]
    QEIdentity(String, u32),
    // TcbType, Fmspc, Version
    #[error("Missing TCB: {0}, FMSPC: {1}, Version: {2}")]
    FMSPCTCB(u8, String, u32),
    // CA, certIsMissing, crlIsMissing
    #[error("Missing PCS collateral: CA: {0}, cert missing: {1}, CRL missing: {2}")]
    PCS(String, bool, bool),
}

#[derive(Debug, Default)]
pub struct Collaterals {
    pub tcb_info: String,
    pub qe_identity: String,
    pub root_ca: Vec<u8>,
    pub tcb_signing_ca: Vec<u8>,
    pub root_ca_crl: Vec<u8>,
    pub pck_crl: Vec<u8>,
}

use anyhow::Result;
use pem::{encode, Pem};

pub fn generate_tcb_issuer_chain_pem(
    tcb_signing_ca_der: &[u8],
    root_ca_der: &[u8],
) -> Result<String> {
    let mut pem_chain = String::new();

    let tcb_pem = Pem::new(
        String::from("CERTIFICATE"),
        tcb_signing_ca_der.to_vec(),
    );

    let root_pem = Pem::new(
        String::from("CERTIFICATE"),
        root_ca_der.to_vec(),
    );

    pem_chain.push_str(&encode(&tcb_pem));
    pem_chain.push_str(&encode(&root_pem));

    Ok(pem_chain)
}

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata On-chain PCCS Reader
[![Automata On Chain PCCS](https://img.shields.io/badge/Power%20By-Automata-orange.svg)](https://github.com/automata-network)

## About

A Reader to decode the basic collaterals from Automata On-chain PCCS, and check the missing collaterals for a given quote.

## Supported Version
* Intel SGX V3 Quote
* Intel SGX V4 Quote
* Intel TDX V4 Quote

Now the reader only supports the cert_type = 5 in the quote attestation data, for other format, please refer to the Intel official documentation.

## **Automata On-chain PCCS resources**

### [Testnet](https://docs.ata.network/protocol/testnet)
| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | 0xefE368b17D137E86298eec8EbC5502fb56d27832 |
| PCCS_ROUTER |0xbFDeE7A1f1bFA2267cD0DA50BE76D8c4a3864543 |
| PCCS_Storage | 0xe8599DD2366230B7EfDD526985c64C7325b27569 |
| V3_VERIFIER | 0x67042D171b8B7Da1A4a98Df787bDce79190DAc3c |
| V4_VERIFIER | 0x921B8F6Ec83E405B715111eC1AE8B54A3ea063EB |
| ENCLAVE_ID_DAO | 0x413272890ab9F155a47A5F90a404Fb51aa259087 |
| FMSPC_TCB_DAO | 0x7c04B466DebA13D48116b1339C62b35B9805E5A0 |
| PCK_DAO | 0x6D4cA6AE5315EBBcb4331c82531db0ad8853Eb31 |
| PCS_DAO | 0xD0335cbC73CA2f8EDd98a2BE3909f55642F414D7 |
| ENCLAVE_IDENTITY_HELPER | 0xfd4a34b578B352FE1896CDafaEb0f45f993352Bf |
| FMSPC_TCB_HELPER | 0xC2A662e08A35513596E22D0aC236Ce72e59125EE |
| PCK_HELPER | 0x5213c0e3Ab478dbc83E8afFF8909717332E4f8E1 |
| CRL_HELPER | 0x12C1E13Aa2a238EAb15c2e2b6AC670266bc3C814 |

### [Mainnet(Preview)](https://docs.ata.network/protocol/mainnet-preview)

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | 0xE26E11B257856B0bEBc4C759aaBDdea72B64351F |
| PCCS_ROUTER | 0xb76834729717868fa203b9D90fc88F859A4E594D |
| PCCS_STORAGE | 0xE2636fdbd053da8E798D959304e20fADa934E8c0 |
| V3_VERIFIER | 0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96 |
| V4_VERIFIER | 0xfF47ecA64898692a86926CDDa794807be3f6567D |
| ENCLAVE_ID_DAO | 0x28111536292b34f37120861A46B39BF39187d73a |
| FMSPC_TCB_DAO | 0x868c18869f68E0E0b0b7B2B4439f7fDDd0421e6b |
| PCK_DAO | 0xeCc198936FcA3Ca1fDc97B8612B32185908917B0 |
| PCS_DAO | 0x86f8865BCe8BE62CB8096b5B94fA3fB3a6ED330c |
| ENCLAVE_IDENTITY_HELPER | 0x13BECaa512713Ac7C2d7a04ba221aD5E02D43DFE |
| FMSPC_TCB_HELPER | 0xc99bF04C31bF3d026B5B47b2574FC19C1459B732 |
| X509_HELPER | 0x3e2fe733E444313A93Fa3f9AEd3bB203048dDE70 |
| X509_CRL_HELPER | 0x2567245dE6E349C8B7AA82fD6FF854b844A0aEF9 |

## See also

* [Automata On Chain PCCS](https://github.com/automata-network/automata-on-chain-pccs)
* [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation)
* [Automata DCAP QPL](https://github.com/automata-network/automata-dcap-qpl)
* [SGX DCAP Caching Service Design Guide](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf)
* [Intel SGX ECDSA Quote Lib Reference DCAP API](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/Intel_SGX_ECDSA_QuoteLibReference_DCAP_API.pdf)
* [DCAP ECDSA Orientation](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/DCAP_ECDSA_Orientation.pdf)
* [Intel PCS API](https://api.portal.trustedservices.intel.com/content/documentation.html)

## Contributing

**Before You Contribute**:
* **Raise an Issue**: If you find a bug or wish to suggest a feature, please open an issue first to discuss it. Detail the bug or feature so we understand your intention.  
* **Pull Requests (PR)**: Before submitting a PR, ensure:  
    * Your contribution successfully builds.
    * It includes tests, if applicable.

## License

Apache License

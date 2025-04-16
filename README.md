<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata On-chain PCCS Reader
[![Automata On Chain PCCS Reader](https://img.shields.io/badge/Power%20By-Automata-orange.svg)](https://github.com/automata-network)

## About

A Reader to decode the basic collaterals from Automata On-chain PCCS, and check the missing collaterals for a given quote.

## Supported Version
* Intel SGX V3 Quote
* Intel SGX V4 Quote
* Intel TDX V4 Quote

Now the reader only supports the cert_type = 5 in the quote attestation data, for other format, please refer to the Intel official documentation.

## **Automata On-chain PCCS resources**

### [Automata Testnet](https://docs.ata.network/protocol/testnet)

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF](https://explorer-testnet.ata.network/address/0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF) |
| PCCS_ROUTER | [0x0d089B3fA00CBAD0a5098025519e9e4620622acF](https://explorer-testnet.ata.network/address/0x0d089B3fA00CBAD0a5098025519e9e4620622acF) |
| PCCS_Storage | [0x6726B94566618B17fA475af862aC73C5b7b41C56](https://explorer-testnet.ata.network/address/0x6726B94566618B17fA475af862aC73C5b7b41C56) |
| V3_VERIFIER | [0x69523d25E25e5c78d828Df90459b75F189D40Cf7](https://explorer-testnet.ata.network/address/0x69523d25E25e5c78d828Df90459b75F189D40Cf7) |
| V4_VERIFIER | [0xD34Cf419AF06629e158DB5F67541AaF2230290FB](https://explorer-testnet.ata.network/address/0xD34Cf419AF06629e158DB5F67541AaF2230290FB) |
| ENCLAVE_ID_DAO | [0xd74e880029cd3B6b434f16beA5F53A06989458Ee](https://explorer-testnet.ata.network/address/0xd74e880029cd3B6b434f16beA5F53A06989458Ee)  |
| FMSPC_TCB_DAO | [0xd3A3f34E8615065704cCb5c304C0cEd41bB81483](https://explorer-testnet.ata.network/address/0xd3A3f34E8615065704cCb5c304C0cEd41bB81483)|
| PCK_DAO | [0xa4615C2a260413878241ff7605AD9577feB356A5](https://explorer-testnet.ata.network/address/0xa4615C2a260413878241ff7605AD9577feB356A5)  |
| PCS_DAO | [0xB270cD8550DA117E3accec36A90c4b0b48daD342](https://explorer-testnet.ata.network/address/0xB270cD8550DA117E3accec36A90c4b0b48daD342) |
| ENCLAVE_IDENTITY_HELPER | [0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0](https://explorer-testnet.ata.network/address/0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0) |
| FMSPC_TCB_HELPER | [0x181dc716922c84554aeA8bafa07c906F4e4C15B2](https://explorer-testnet.ata.network/address/0x181dc716922c84554aeA8bafa07c906F4e4C15B2) |
| PCK_HELPER | [0xeD75bb6543c53d49f4445055Ba18380068025370](https://explorer-testnet.ata.network/address/0xeD75bb6543c53d49f4445055Ba18380068025370) |
| CRL_HELPER | [0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C](https://explorer-testnet.ata.network/address/0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C) |

### [Automata Mainnet](https://docs.ata.network/protocol/mainnet)

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF](https://explorer.ata.network/address/0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF) |
| PCCS_ROUTER | [0x0d089B3fA00CBAD0a5098025519e9e4620622acF](https://explorer.ata.network/address/0x0d089B3fA00CBAD0a5098025519e9e4620622acF) |
| PCCS_Storage | [0x6726B94566618B17fA475af862aC73C5b7b41C56](https://explorer.ata.network/address/0x6726B94566618B17fA475af862aC73C5b7b41C56) |
| V3_VERIFIER | [0x69523d25E25e5c78d828Df90459b75F189D40Cf7](https://explorer.ata.network/address/0x69523d25E25e5c78d828Df90459b75F189D40Cf7) |
| V4_VERIFIER | [0xD34Cf419AF06629e158DB5F67541AaF2230290FB](https://explorer.ata.network/address/0xD34Cf419AF06629e158DB5F67541AaF2230290FB) |
| ENCLAVE_ID_DAO | [0xd74e880029cd3B6b434f16beA5F53A06989458Ee](https://explorer.ata.network/address/0xd74e880029cd3B6b434f16beA5F53A06989458Ee)  |
| FMSPC_TCB_DAO | [0xd3A3f34E8615065704cCb5c304C0cEd41bB81483](https://explorer.ata.network/address/0xd3A3f34E8615065704cCb5c304C0cEd41bB81483)|
| PCK_DAO | [0xa4615C2a260413878241ff7605AD9577feB356A5](https://explorer.ata.network/address/0xa4615C2a260413878241ff7605AD9577feB356A5)  |
| PCS_DAO | [0xB270cD8550DA117E3accec36A90c4b0b48daD342](https://explorer.ata.network/address/0xB270cD8550DA117E3accec36A90c4b0b48daD342) |
| ENCLAVE_IDENTITY_HELPER | [0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0](https://explorer.ata.network/address/0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0) |
| FMSPC_TCB_HELPER | [0x181dc716922c84554aeA8bafa07c906F4e4C15B2](https://explorer.ata.network/address/0x181dc716922c84554aeA8bafa07c906F4e4C15B2) |
| PCK_HELPER | [0xeD75bb6543c53d49f4445055Ba18380068025370](https://explorer.ata.network/address/0xeD75bb6543c53d49f4445055Ba18380068025370) |
| CRL_HELPER | [0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C](https://explorer.ata.network/address/0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C) |

#### Ethereum Holesky Testnet

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF](https://holesky.etherscan.io/address/0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF) |
| PCCS_ROUTER | [0xe20C4d54afBbea5123728d5b7dAcD9CB3c65C39a](https://holesky.etherscan.io/address/0xe20C4d54afBbea5123728d5b7dAcD9CB3c65C39a) |
| PCCS_Storage | [0x6726B94566618B17fA475af862aC73C5b7b41C56](https://holesky.etherscan.io/address/0x6726B94566618B17fA475af862aC73C5b7b41C56) |
| V3_VERIFIER | [0x816ADa3B63F3c643fb04152eA32B58Db89aadd89](https://holesky.etherscan.io/address/0x816ADa3B63F3c643fb04152eA32B58Db89aadd89) |
| V4_VERIFIER | [0x1a9E873C041F566fCBAdbD9e1358095b7Ea12AA8](https://holesky.etherscan.io/address/0x1a9E873C041F566fCBAdbD9e1358095b7Ea12AA8) |
| ENCLAVE_ID_DAO | [0xc3ea5Ff40263E16cD2f4413152A77e7A6b10B0C9](https://holesky.etherscan.io/address/0xc3ea5Ff40263E16cD2f4413152A77e7A6b10B0C9) |
| FMSPC_TCB_DAO | [0x63eF330eAaadA189861144FCbc9176dae41A5BAf](https://holesky.etherscan.io/address/0x63eF330eAaadA189861144FCbc9176dae41A5BAf) |
| PCK_DAO | [0x75A2BafFfb2096990246F1a2dA65801Ea2A00b36](https://holesky.etherscan.io/address/0x75A2BafFfb2096990246F1a2dA65801Ea2A00b36)  |
| PCS_DAO | [0x45CF7485A0D394130153a3630EA0729999511C2e](https://holesky.etherscan.io/address/0x45CF7485A0D394130153a3630EA0729999511C2e) |
| ENCLAVE_IDENTITY_HELPER | [0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0](https://holesky.etherscan.io/address/0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0) |
| FMSPC_TCB_HELPER | [0x181dc716922c84554aeA8bafa07c906F4e4C15B2](https://holesky.etherscan.io/address/0x181dc716922c84554aeA8bafa07c906F4e4C15B2) |
| PCK_HELPER | [0xeD75bb6543c53d49f4445055Ba18380068025370](https://holesky.etherscan.io/address/0xeD75bb6543c53d49f4445055Ba18380068025370) |
| CRL_HELPER | [0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C](https://holesky.etherscan.io/address/0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C) |

#### Ethereum Sepolia Testnet

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF](https://sepolia.etherscan.io/address/0x95175096a9B74165BE0ac84260cc14Fc1c0EF5FF) |
| PCCS_ROUTER | [0xe20C4d54afBbea5123728d5b7dAcD9CB3c65C39a](https://sepolia.etherscan.io/address/0xe20C4d54afBbea5123728d5b7dAcD9CB3c65C39a) |
| PCCS_Storage | [0x6726B94566618B17fA475af862aC73C5b7b41C56](https://sepolia.etherscan.io/address/0x6726B94566618B17fA475af862aC73C5b7b41C56) |
| V3_VERIFIER | [0x816ADa3B63F3c643fb04152eA32B58Db89aadd89](https://sepolia.etherscan.io/address/0x816ADa3B63F3c643fb04152eA32B58Db89aadd89) |
| V4_VERIFIER | [0x1a9E873C041F566fCBAdbD9e1358095b7Ea12AA8](https://sepolia.etherscan.io/address/0x1a9E873C041F566fCBAdbD9e1358095b7Ea12AA8) |
| ENCLAVE_ID_DAO | [0xc3ea5Ff40263E16cD2f4413152A77e7A6b10B0C9](https://sepolia.etherscan.io/address/0xc3ea5Ff40263E16cD2f4413152A77e7A6b10B0C9) |
| FMSPC_TCB_DAO | [0x63eF330eAaadA189861144FCbc9176dae41A5BAf](https://sepolia.etherscan.io/address/0x63eF330eAaadA189861144FCbc9176dae41A5BAf) |
| PCK_DAO | [0x75A2BafFfb2096990246F1a2dA65801Ea2A00b36](https://sepolia.etherscan.io/address/0x75A2BafFfb2096990246F1a2dA65801Ea2A00b36)  |
| PCS_DAO | [0x45CF7485A0D394130153a3630EA0729999511C2e](https://sepolia.etherscan.io/address/0x45CF7485A0D394130153a3630EA0729999511C2e) |
| ENCLAVE_IDENTITY_HELPER | [0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0](https://sepolia.etherscan.io/address/0x635A8A01e84cDcE1475FCeB7D57FEcadD3d1a0A0) |
| FMSPC_TCB_HELPER | [0x181dc716922c84554aeA8bafa07c906F4e4C15B2](https://sepolia.etherscan.io/address/0x181dc716922c84554aeA8bafa07c906F4e4C15B2) |
| PCK_HELPER | [0xeD75bb6543c53d49f4445055Ba18380068025370](https://sepolia.etherscan.io/address/0xeD75bb6543c53d49f4445055Ba18380068025370) |
| CRL_HELPER | [0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C](https://sepolia.etherscan.io/address/0xA454FB9522631D586f3A790c6CDc6f1B70Ca903C) |

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

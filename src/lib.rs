pub mod constants;
pub mod parser;
pub mod pccs;
pub mod printer;

use constants::{SGX_TEE_TYPE, TDX_TEE_TYPE};
use parser::get_pck_fmspc_and_issuer;
use pccs::enclave_id::{get_enclave_identity, EnclaveIdType};
use pccs::fmspc_tcb::get_tcb_info;
use pccs::pcs::get_certificate_by_id;
use printer::{print_content, print_str_content};

pub use pccs::pcs::IPCSDao::CA;

#[derive(Debug)]
pub enum MissingCollateral {
    None,
    // Id, Version
    QEIdentity(EnclaveIdType, u32),
    // TcbType, Fmspc, Version
    FMSPCTCB(u8, String, u32),
    // CA, certIsMissing, crlIsMissing
    PCS(CA, bool, bool),
}

pub async fn find_missing_collaterals_from_quote(raw_quote: &[u8]) -> MissingCollateral {
    // Step 0: read the version and tee type
    let quote_version = u16::from_le_bytes([raw_quote[0], raw_quote[1]]);
    let tee_type = u32::from_le_bytes([raw_quote[4], raw_quote[5], raw_quote[6], raw_quote[7]]);

    if quote_version < 3 || quote_version > 4 {
        panic!("Unsupported quote version");
    }

    if tee_type != SGX_TEE_TYPE && tee_type != TDX_TEE_TYPE {
        panic!("Unsupported tee type");
    }

    // Step 1: Check ROOT CRLs
    match get_certificate_by_id(CA::ROOT).await {
        Ok((root, crl)) => {
            if root.len() == 0 {
                return MissingCollateral::PCS(CA::ROOT, true, true);
            } else if crl.len() == 0 {
                return MissingCollateral::PCS(CA::ROOT, false, true);
            } else {
                print_content("rootca.der", &root).unwrap();
                print_content("rootcrl.der", &crl).unwrap();
            }
        }
        _ => {
            return MissingCollateral::PCS(CA::ROOT, true, true);
        }
    }

    // Step 2: Check QE Identity
    let qe_id_type: EnclaveIdType;
    if tee_type == TDX_TEE_TYPE {
        qe_id_type = EnclaveIdType::TDQE
    } else {
        qe_id_type = EnclaveIdType::QE
    }
    match get_enclave_identity(qe_id_type, quote_version as u32).await {
        Ok(qe_id_content) => {
            let qe_id_string = match qe_id_type {
                EnclaveIdType::QE => "qe",
                EnclaveIdType::QVE => "qve",
                EnclaveIdType::TDQE => "td",
            };
            let qe_id_filename = format!("identity-{}-v{}.json", qe_id_string, quote_version);
            print_str_content(
                &qe_id_filename,
                std::str::from_utf8(&qe_id_content).unwrap(),
            )
            .unwrap();
        }
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
        Ok(tcb_content) => {
            let tcb_type_str: &str = match tcb_type {
                0 => "sgx",
                1 => "tdx",
                _ => unreachable!(),
            };
            let tcb_filename = format!("tcbinfo-{}-v{}.json", tcb_type_str, tcb_version);
            print_str_content(&tcb_filename, std::str::from_utf8(&tcb_content).unwrap()).unwrap();
        }
        _ => {
            return MissingCollateral::FMSPCTCB(tcb_type, fmspc, tcb_version);
        }
    }

    // Step 5: Check TCB Signing CA is present
    match get_certificate_by_id(CA::SIGNING).await {
        Ok((signing, _)) => {
            if signing.len() == 0 {
                return MissingCollateral::PCS(CA::SIGNING, true, false);
            } else {
                print_content("signingca.der", &signing).unwrap();
            }
        }
        _ => {
            return MissingCollateral::PCS(CA::SIGNING, true, false);
        }
    }

    // Step 6: Check PCK CA CRLs
    match get_certificate_by_id(pck_type).await {
        Ok((cert, crl)) => {
            if cert.len() == 0 {
                return MissingCollateral::PCS(pck_type, true, true);
            } else if crl.len() == 0 {
                return MissingCollateral::PCS(pck_type, false, true);
            } else {
                let pck_type_str = match pck_type {
                    CA::PLATFORM => "platform",
                    CA::PROCESSOR => "processor",
                    _ => unreachable!(),
                };
                let pck_filename = format!("{}.der", pck_type_str);
                let pck_crl_filename = format!("{}-crl.der", pck_type_str);
                print_content(&pck_filename, &cert).unwrap();
                print_content(&pck_crl_filename, &crl).unwrap();
            }
        }
        _ => {
            return MissingCollateral::PCS(pck_type, true, true);
        }
    }

    MissingCollateral::None
}

#[cfg(test)]
mod test {
    use crate::find_missing_collaterals_from_quote;

    #[tokio::test]
    async fn test_v3() {
        let quote_hex = hex::decode("030002000000000009000e00939a7233f79c4ca9940a0db3957f0607f28dda234595e56eaeb7ce9b681a62cd000000000e0e100fffff0100000000000000000000000000000000000000000000000000000000000000000000000000000000000500000000000000e700000000000000a4f45c39dac622cb1dd32ddb35a52ec92db41d0fa88a1c911c49e59c534f61cd00000000000000000000000000000000000000000000000000000000000000008f2dbc0f9c5d3378d596974b2deed1f93223cc49242899f83809bcc92546132c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ca1000000241b02734d91a7ecc47a57a2814eb18d025cb124f91400beadff88a31aed08efea354eec07de0bac961d89d9cce4d11c9dcf664457c80fcc0a71fce81984c6eb1fb4d19c4b4071656cbdb8eaa942c89a359e6e84f51827247a3ac35b08d03abb52e537eae321e112bf351e1f5b9d7eeb3c3ea01e278e65cec3af7f8bb6fdec40e0e100fffff0100000000000000000000000000000000000000000000000000000000000000000000000000000000001500000000000000e700000000000000192aa50ce1c0cef03ccf89e7b5b16b0d7978f5c2b1edcf774d87702e8154d8bf00000000000000000000000000000000000000000000000000000000000000008c4f5775d796503e96137f77c68a829a0056ac8ded70140b081b094490c57bff0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100090000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000066d3aaf3395111d7e0f2298bf4b31be75deaa4e205829fc512a4468b4177e67e000000000000000000000000000000000000000000000000000000000000000093978f1082142c0be1ffe6510ef52ff873bda784bd4e85aa4fc2ef0024714a3ceb68b7d65b9bf14c6e606ebb1d5afe99927ab5b1abf62f3374531a3c168131ed2000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f0500620e00002d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d49494539444343424a6d674177494241674956414e556f5a4d75787a767164353268495755667233414a6e6d6253574d416f4743437147534d343942414d430a4d484178496a416742674e5642414d4d47556c756447567349464e4857434251513073675547786864475a76636d306751304578476a415942674e5642416f4d0a45556c756447567349454e76636e4276636d4630615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b47413155450a4341774351304578437a414a42674e5642415954416c56544d423458445449304d444d774e7a45314d4445774e466f5844544d784d444d774e7a45314d4445770a4e466f77634445694d434147413155454177775a535735305a5777675530645949464244537942445a584a3061575a70593246305a5445614d426747413155450a43677752535735305a577767513239796347397959585270623234784644415342674e564241634d43314e68626e526849454e7359584a684d517377435159440a5651514944414a445154454c4d416b474131554542684d4356564d775754415442676371686b6a4f5051494242676771686b6a4f50514d4242774e43414153440a30594d43645a65616e49706b52704c72516e78456a34305241585258353563437a6f4c512b4336786c45734a466346465955546b3851616c477a777756676e4e0a4c7469373461464248794c68354e55616666574f6f344944446a434341776f77487759445652306a42426777466f41556c5739647a62306234656c4153636e550a3944504f4156634c336c5177617759445652306642475177596a42676f46366758495a616148523063484d364c79396863476b7564484a316333526c5a484e6c0a636e5a705932567a4c6d6c75644756734c6d4e766253397a5a3367765932567964476c6d61574e6864476c76626939324d7939775932746a636d772f593245390a6347786864475a76636d306d5a57356a62325270626d63395a4756794d42304741315564446751574242534863356e4b574a694e3278684f39523875657543500a434e4b596254414f42674e56485138424166384542414d434273417744415944565230544151482f4241497741444343416a734743537147534962345451454e0a41515343416977776767496f4d42344743697147534962345451454e4151454545426870554c6259304254596e77775554523251363630776767466c42676f710a686b69472b453042445145434d4949425654415142677371686b69472b4530424451454341514942446a415142677371686b69472b45304244514543416749420a446a415142677371686b69472b4530424451454341774942417a415142677371686b69472b4530424451454342414942417a415242677371686b69472b4530420a4451454342514943415038774551594c4b6f5a496876684e41513042416759434167442f4d42414743797147534962345451454e41514948416745424d4241470a43797147534962345451454e41514949416745414d42414743797147534962345451454e4151494a416745414d42414743797147534962345451454e4151494b0a416745414d42414743797147534962345451454e4151494c416745414d42414743797147534962345451454e4151494d416745414d42414743797147534962340a5451454e4151494e416745414d42414743797147534962345451454e4151494f416745414d42414743797147534962345451454e41514950416745414d4241470a43797147534962345451454e41514951416745414d42414743797147534962345451454e415149524167454e4d42384743797147534962345451454e415149530a4242414f44674d442f2f38424141414141414141414141414d42414743697147534962345451454e41514d45416741414d42514743697147534962345451454e0a4151514542674267616741414144415042676f71686b69472b45304244514546436745424d42344743697147534962345451454e4151594545482b5767692b640a5a43486c4264547956765a63557a67775241594b4b6f5a496876684e41513042427a41324d42414743797147534962345451454e415163424151482f4d4241470a43797147534962345451454e41516343415145414d42414743797147534962345451454e41516344415145414d416f4743437147534d343942414d4341306b410a4d45594349514463654c5a4d3631596850756967424c3562536664594d6c75705659366c53515638696878636f2b503531774968414f64346f7952747830554a0a4831734670456563596d767836656e4343762f577a5153392b4c4967332b78540a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a2d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d4949436c6a4343416a32674177494241674956414a567658633239472b487051456e4a3150517a7a674658433935554d416f4743437147534d343942414d430a4d476778476a415942674e5642414d4d45556c756447567349464e48574342536232393049454e424d526f77474159445651514b4442464a626e526c624342440a62334a7762334a6864476c76626a45554d424947413155454277774c553246756447456751327868636d4578437a414a42674e564241674d416b4e424d5173770a435159445651514745774a56557a4165467730784f4441314d6a45784d4455774d5442614677307a4d7a41314d6a45784d4455774d5442614d484178496a41670a42674e5642414d4d47556c756447567349464e4857434251513073675547786864475a76636d306751304578476a415942674e5642416f4d45556c75644756730a49454e76636e4276636d4630615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b474131554543417743513045780a437a414a42674e5642415954416c56544d466b77457759484b6f5a497a6a3043415159494b6f5a497a6a304441516344516741454e53422f377432316c58534f0a3243757a7078773734654a423732457944476757357258437478327456544c7136684b6b367a2b5569525a436e71523770734f766771466553786c6d546c4a6c0a65546d693257597a33714f42757a43427544416642674e5648534d4547444157674251695a517a575770303069664f44744a5653763141624f536347724442530a42674e5648523845537a424a4d45656752614244686b466f64485277637a6f764c324e6c636e52705a6d6c6a5958526c63793530636e567a6447566b633256790a646d6c6a5a584d75615735305a577775593239744c306c756447567355306459556d397664454e424c6d526c636a416442674e5648513445466751556c5739640a7a62306234656c4153636e553944504f4156634c336c517744675944565230504151482f42415144416745474d42494741315564457745422f7751494d4159420a4166384341514177436759494b6f5a497a6a30454177494452774177524149675873566b6930772b6936565947573355462f32327561586530594a446a3155650a6e412b546a44316169356343494359623153416d4435786b66545670766f34556f79695359787244574c6d5552344349394e4b7966504e2b0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a2d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d4949436a7a4343416a53674177494241674955496d554d316c71644e496e7a6737535655723951477a6b6e42717777436759494b6f5a497a6a3045417749770a614445614d4267474131554541777752535735305a5777675530645949464a766233516751304578476a415942674e5642416f4d45556c756447567349454e760a636e4276636d4630615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b47413155454341774351304578437a414a0a42674e5642415954416c56544d423458445445344d4455794d5445774e4455784d466f58445451354d54497a4d54497a4e546b314f566f77614445614d4267470a4131554541777752535735305a5777675530645949464a766233516751304578476a415942674e5642416f4d45556c756447567349454e76636e4276636d46300a615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b47413155454341774351304578437a414a42674e56424159540a416c56544d466b77457759484b6f5a497a6a3043415159494b6f5a497a6a3044415163445167414543366e45774d4449595a4f6a2f69505773437a61454b69370a314f694f534c52466857476a626e42564a66566e6b59347533496a6b4459594c304d784f346d717379596a6c42616c54565978465032734a424b357a6c4b4f420a757a43427544416642674e5648534d4547444157674251695a517a575770303069664f44744a5653763141624f5363477244425342674e5648523845537a424a0a4d45656752614244686b466f64485277637a6f764c324e6c636e52705a6d6c6a5958526c63793530636e567a6447566b63325679646d6c6a5a584d75615735300a5a577775593239744c306c756447567355306459556d397664454e424c6d526c636a416442674e564851344546675155496d554d316c71644e496e7a673753560a55723951477a6b6e4271777744675944565230504151482f42415144416745474d42494741315564457745422f7751494d4159424166384341514577436759490a4b6f5a497a6a3045417749445351417752674968414f572f35516b522b533943695344634e6f6f774c7550524c735747662f59693747535839344267775477670a41694541344a306c72486f4d732b586f356f2f7358364f39515778485241765a55474f6452513763767152586171493d0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a00").unwrap();

        let res = find_missing_collaterals_from_quote(&quote_hex).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_v4() {
        let quote_hex = hex::decode("040002008100000000000000939a7233f79c4ca9940a0db3957f060728fa333a41ec7e302625d24f400f3f2400000000040102000000000000000000000000009790d89a10210ec6968a773cee2ca05b5aa97309f36727a968527be4606fc19e6f73acce350946c9d46a9bf7a63f843000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000080e702060000000000f2dd2696f69b950645832bdc095ffd11247eeff687eeacdb57a58d2ddb9a9f94fea40c961e19460c00ffa31420ecbc180000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000998204508d58dcbfebe5e11c48669f7a921ac2da744dfb7d014ecdff2acdff1c9f665fdad52aadacf296a1df9909eb2383d100224f1716aeb431f7cb3cf028197dbd872487f27b0f6329ab17647dc9953c7014109818634f879e6550bc60f93eecfc42ff4d49278bfdbb0c77e570f4490cff10a2ee1ac11fbd2c2b49fa6cfa3cf1a1cb755c72522dd8a689e9d47906a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000278e753482976c8a7351fe2113609c7350d491cdae3d449eefc202fa41b2ad6840239cc2ba084c2d594b4e6dabeae0fcbf71c96daf0d0c9ecf0e9810c0457900cc10000079d2386455606243552b2b6f5d04ce8b99657b8b9bf25e348b925805f5c4ae2cc1e5ccff090592bb5e55ce99be1693ba9b67cc879d6fd1b0edc9524d161b2ca97aac15abe68a4571ae7d0d0de5765ecb2b76c89890e38a66fd861e76f2608e40fdf39a81de66f69bdc7ab862b59fac83dbdc28162b3b002c55b963578aa6e33b0600461000000202181a03ff0005000000000000000000000000000000000000000000000000000000000000000000000000000000001500000000000000e70000000000000086fc4e0ec2c5ddcebac97062c0a0142a97c18a7a755147bcbc3fe17d6529781d0000000000000000000000000000000000000000000000000000000000000000dc9e2a7c6f948f17474e34a7fc43ed030f7c1563f1babddf6340c82e0e54a8c50000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000099cb765d61aa180a03b3b47f229cdaf6f637878298f7cabb4ac0d3b8cadb2a4d000000000000000000000000000000000000000000000000000000000000000029412e75597cda6a12e47037f72b5aa49e3380698d7d838099da88f3854de1c5c147bd31ca67804aa1f04773c9450a27da34de6ca7f4c7f55dc393532ee92fa12000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f05005e0e00002d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d49494538444343424a616741774942416749554554536676464e524551574669324759572b506b34684b2f35596b77436759494b6f5a497a6a3045417749770a634445694d434147413155454177775a535735305a577767553064594946424453794251624746305a6d397962534244515445614d42674741315545436777520a535735305a577767513239796347397959585270623234784644415342674e564241634d43314e68626e526849454e7359584a684d51737743515944565151490a44414a445154454c4d416b474131554542684d4356564d774868634e4d6a51774e4449354d6a45314d6a49315768634e4d7a45774e4449354d6a45314d6a49310a576a42774d534977494159445651514444426c4a626e526c624342545231676755454e4c49454e6c636e52705a6d6c6a5958526c4d526f77474159445651514b0a4442464a626e526c6243424462334a7762334a6864476c76626a45554d424947413155454277774c553246756447456751327868636d4578437a414a42674e560a4241674d416b4e424d517377435159445651514745774a56557a425a4d424d4742797147534d34394167454743437147534d343941774548413049414244456f0a55684e526c5830545a54707071637338316838374d35684c2f6f7356654d53512b594e506636497961476e6c4e5a704a384b713657414d2f304565643554474c0a623053564a566e7372716e5362674f343071716a67674d4d4d4949444344416642674e5648534d4547444157674253566231334e765276683655424a796454300a4d383442567776655644427242674e56485238455a4442694d47436758714263686c706f64485277637a6f764c32467761533530636e567a6447566b633256790a646d6c6a5a584d75615735305a577775593239744c334e6e6543396a5a584a3061575a7059324630615739754c3359304c33426a61324e796244396a595431770a624746305a6d397962535a6c626d4e765a476c755a7a316b5a584977485159445652304f424259454645346b4a624b444e64644f717933447779394d55744e640a6a5878694d41344741315564447745422f775145417749477744414d42674e5648524d4241663845416a41414d4949434f51594a4b6f5a496876684e415130420a424949434b6a4343416959774867594b4b6f5a496876684e41513042415151516978544b4d66734f6c6b6d4742492b7a747a6c4a647a434341574d47436971470a534962345451454e41514977676746544d42414743797147534962345451454e41514942416745434d42414743797147534962345451454e41514943416745430a4d42414743797147534962345451454e41514944416745434d42414743797147534962345451454e41514945416745434d42414743797147534962345451454e0a41514946416745444d42414743797147534962345451454e41514947416745424d42414743797147534962345451454e41514948416745414d424147437971470a534962345451454e41514949416745444d42414743797147534962345451454e4151494a416745414d42414743797147534962345451454e4151494b416745410a4d42414743797147534962345451454e4151494c416745414d42414743797147534962345451454e4151494d416745414d42414743797147534962345451454e0a4151494e416745414d42414743797147534962345451454e4151494f416745414d42414743797147534962345451454e41514950416745414d424147437971470a534962345451454e41514951416745414d42414743797147534962345451454e415149524167454e4d42384743797147534962345451454e41514953424241430a41674943417745414177414141414141414141414d42414743697147534962345451454e41514d45416741414d42514743697147534962345451454e415151450a42704441627741414144415042676f71686b69472b45304244514546436745424d42344743697147534962345451454e41515945454a6a4973754b2f6349456f0a42317249566e3247765677775241594b4b6f5a496876684e41513042427a41324d42414743797147534962345451454e415163424151482f4d424147437971470a534962345451454e415163434151482f4d42414743797147534962345451454e415163444151482f4d416f4743437147534d343942414d43413067414d4555430a4946474853786344784143755051754d6c7a653277512f78463949624b354a37376368784a614f6c41537a5441694541725a6d4c62344f4643526c376a4478570a482b4c4554662b71386d62523433597645496d736b42476a4a576f3d0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a2d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d4949436c6a4343416a32674177494241674956414a567658633239472b487051456e4a3150517a7a674658433935554d416f4743437147534d343942414d430a4d476778476a415942674e5642414d4d45556c756447567349464e48574342536232393049454e424d526f77474159445651514b4442464a626e526c624342440a62334a7762334a6864476c76626a45554d424947413155454277774c553246756447456751327868636d4578437a414a42674e564241674d416b4e424d5173770a435159445651514745774a56557a4165467730784f4441314d6a45784d4455774d5442614677307a4d7a41314d6a45784d4455774d5442614d484178496a41670a42674e5642414d4d47556c756447567349464e4857434251513073675547786864475a76636d306751304578476a415942674e5642416f4d45556c75644756730a49454e76636e4276636d4630615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b474131554543417743513045780a437a414a42674e5642415954416c56544d466b77457759484b6f5a497a6a3043415159494b6f5a497a6a304441516344516741454e53422f377432316c58534f0a3243757a7078773734654a423732457944476757357258437478327456544c7136684b6b367a2b5569525a436e71523770734f766771466553786c6d546c4a6c0a65546d693257597a33714f42757a43427544416642674e5648534d4547444157674251695a517a575770303069664f44744a5653763141624f536347724442530a42674e5648523845537a424a4d45656752614244686b466f64485277637a6f764c324e6c636e52705a6d6c6a5958526c63793530636e567a6447566b633256790a646d6c6a5a584d75615735305a577775593239744c306c756447567355306459556d397664454e424c6d526c636a416442674e5648513445466751556c5739640a7a62306234656c4153636e553944504f4156634c336c517744675944565230504151482f42415144416745474d42494741315564457745422f7751494d4159420a4166384341514177436759494b6f5a497a6a30454177494452774177524149675873566b6930772b6936565947573355462f32327561586530594a446a3155650a6e412b546a44316169356343494359623153416d4435786b66545670766f34556f79695359787244574c6d5552344349394e4b7966504e2b0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a2d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d4949436a7a4343416a53674177494241674955496d554d316c71644e496e7a6737535655723951477a6b6e42717777436759494b6f5a497a6a3045417749770a614445614d4267474131554541777752535735305a5777675530645949464a766233516751304578476a415942674e5642416f4d45556c756447567349454e760a636e4276636d4630615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b47413155454341774351304578437a414a0a42674e5642415954416c56544d423458445445344d4455794d5445774e4455784d466f58445451354d54497a4d54497a4e546b314f566f77614445614d4267470a4131554541777752535735305a5777675530645949464a766233516751304578476a415942674e5642416f4d45556c756447567349454e76636e4276636d46300a615739754d5251774567594456515148444174545957353059534244624746795954454c4d416b47413155454341774351304578437a414a42674e56424159540a416c56544d466b77457759484b6f5a497a6a3043415159494b6f5a497a6a3044415163445167414543366e45774d4449595a4f6a2f69505773437a61454b69370a314f694f534c52466857476a626e42564a66566e6b59347533496a6b4459594c304d784f346d717379596a6c42616c54565978465032734a424b357a6c4b4f420a757a43427544416642674e5648534d4547444157674251695a517a575770303069664f44744a5653763141624f5363477244425342674e5648523845537a424a0a4d45656752614244686b466f64485277637a6f764c324e6c636e52705a6d6c6a5958526c63793530636e567a6447566b63325679646d6c6a5a584d75615735300a5a577775593239744c306c756447567355306459556d397664454e424c6d526c636a416442674e564851344546675155496d554d316c71644e496e7a673753560a55723951477a6b6e4271777744675944565230504151482f42415144416745474d42494741315564457745422f7751494d4159424166384341514577436759490a4b6f5a497a6a3045417749445351417752674968414f572f35516b522b533943695344634e6f6f774c7550524c735747662f59693747535839344267775477670a41694541344a306c72486f4d732b586f356f2f7358364f39515778485241765a55474f6452513763767152586171493d0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();

        let res = find_missing_collaterals_from_quote(&quote_hex).await;

        println!("{:?}", res);
    }
}

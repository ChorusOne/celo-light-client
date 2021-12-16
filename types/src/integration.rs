use crate::{consensus::*, istanbul::*, state::*, *};
use std::{fs::File, io::BufReader};

//WARNING!!! this is super slow, it takes more than 5 minutes to run
//const BAKLAVA_FILE :&str = "data/baklava.json";
// when tests fail and you want to test on a subset of headers // faster cycle
const BAKLAVA_FILE: &str = "data/baklava_small.json";

pub fn get_genesis() -> LightConsensusState {
    let blocks = read_headers();

    let genesis_header = blocks.first().unwrap();
    assert_eq!(genesis_header.number.as_u64(), 0, "no genesis block found");

    let ista_extra = extract_istanbul_extra(&genesis_header).unwrap();

    LightConsensusState {
        number: genesis_header.number.as_u64(),
        validators: ista_extra
            .added_validators
            .into_iter()
            .zip(ista_extra.added_validators_public_keys)
            .map(ValidatorData::from)
            .collect(),
        hash: genesis_header.root,
    }
}

pub fn get_header(idx: usize) -> Header {
    let blocks = read_headers();
    blocks.get(idx).unwrap().clone()
}

fn read_headers() -> Vec<Header> {
    let data_path = std::env::current_dir().unwrap().join(BAKLAVA_FILE);

    let file = File::open(data_path).unwrap();
    let reader = BufReader::new(file);
    let mut blocks: Vec<Header> = serde_json::from_reader(reader).unwrap();
    blocks.sort_by_key(|b| b.number);
    blocks
}

#[test]
fn run_baklava() {
    let cfg = client::Config {
        chain_id: 1,
        epoch_size: 17280,
        allowed_clock_skew: 5,
        verify_epoch_headers: true,
        verify_non_epoch_headers: true,
        verify_header_timestamp: true,
    };
    let lcons = get_genesis();
    let mut heads = read_headers();
    heads.remove(0);

    let mut state = State::new(lcons.clone(), &cfg);
    for head in heads {
        println!("cons {:?} - header {:?}", lcons.number, head.number);
        let res = state.insert_header(&head, head.time.as_u64());
        assert!(res.is_ok());
    }
}

#[cfg(feature = "bls-support")]
#[test]
fn alfajores_snapshot_to_validators_bls_public_key() {
    let snapshot_json = String::from(
        r#"{"epoch":17280,"number":0,"hash":"0xe423b034e7f0282c1b621f7bbc1cea4316a2a80b1600490769eae77777e4b67e","validators":[{"Address":"0x456f41406b32c45d59e539e4bba3d7898c3584da","BLSPublicKey":"0x11877b768127c8eb0f122fbe69553bc9d142d27c06a85c6eeb7b8b457f511e50c33a57fcbc5fd6d1823f69a111f8010151a17f6a8798a25343f5403b1e6a595c7d9698af3db78b013d26a761fc201b3cf793be5f0a0a849b3f68a8bfa81e7001","UncompressedBLSPublicKey":"EYd7doEnyOsPEi++aVU7ydFC0nwGqFxu63uLRX9RHlDDOlf8vF/W0YI/aaER+AEBUaF/aoeYolND9UA7HmpZXH2WmK89t4sBPSanYfwgGzz3k75fCgqEmz9oqL+oHnABqenLvv4TWrWJfwoQaGk60Y75cP/GYzfkyUtWKTxovKfDOAUr+yrQDHfmPQSY510ABQMe4X4tmS8W871QFm+PwA/iK/6BZtybo01/sNFIdmpMrSi6qUlSTe3+b4X7I1AA"},{"Address":"0xdd1f519f63423045f526b8c83edc0eb4ba6434a4","BLSPublicKey":"0xd882cd4cc09109928e9517644d5303610155978cf5e3b7ad6122daa19c3dab3da8c439bc763d6d3eef18a38ebb0d3200664b94fab11adbb3f44b963969763b590af45931c482396be88a185214c9c8690615aae5197e852bc1d04b3dbd03ab80","UncompressedBLSPublicKey":"2ILNTMCRCZKOlRdkTVMDYQFVl4z147etYSLaoZw9qz2oxDm8dj1tPu8Yo467DTIAZkuU+rEa27P0S5Y5aXY7WQr0WTHEgjlr6IoYUhTJyGkGFarlGX6FK8HQSz29A6sAJc/XrAfVTvqKkglweb4kbXwQZi+eptAdoaQEa/bFcwTzlXFxYiZkIAHNKTYuT24BAv4R5jy9J9u25AXF4vGzHuRQEuTgThr5w00Ggkctko2oC+9TfYijSXGNA1XfDe4A"},{"Address":"0x050f34537f5b2a00b9b9c752cb8500a3fce3da7d","BLSPublicKey":"0x51588d46ba8998d944a30cde93bfe946e774ef1f6fe2fb559a74ffebf60d1ad967b876a038c6e312d0c20752cbc8440012293b6ea417f32a163caedeaaae7aad3c1b31be1fe86c405924b1be7d0aaae6f3ba567ee907d0d4c00dce5091442380","UncompressedBLSPublicKey":"UViNRrqJmNlEowzek7/pRud07x9v4vtVmnT/6/YNGtlnuHagOMbjEtDCB1LLyEQAEik7bqQX8yoWPK7eqq56rTwbMb4f6GxAWSSxvn0KqubzulZ+6QfQ1MANzlCRRCMADQPiHXCZeXU1zLNkW4Cs4ImgnNd1uAst4k/8rR9kBhJujl+ONY9aQiJMSAtwpvAARZ/j7ZGIeCHHyd8NI0MEP3l/TImeQjYg0G4x5Zkr0zNincVMsoBD8gpIhMMozKUB"},{"Address":"0xcda518f6b5a797c3ec45d37c65b83e0b0748edca","BLSPublicKey":"0x1f2becc31c1f0141e8c5768c5f07d02d1342c086c037cce70aaf3629b40ea017884a81163f58697b020b21fe39c440006970bc1f52b847d7262599ae92ee7db45ad38efe5612c8ed42d9db9380da0769bab713f5259b7c015998296bf02a0a01","UncompressedBLSPublicKey":"HyvswxwfAUHoxXaMXwfQLRNCwIbAN8znCq82KbQOoBeISoEWP1hpewILIf45xEAAaXC8H1K4R9cmJZmuku59tFrTjv5WEsjtQtnbk4DaB2m6txP1JZt8AVmYKWvwKgoB5rvuObt0MtUF2l8Iezu8X8t85d7gFkCHCmvwDsjAnXM8UwefaStlDAxqr7HHhX0BLbuSsoWenj5sarXubND3Kpy/fRM/1Bzz9OwCXZghP9QvzD9SJoPlN5Jxl37+3kAA"},{"Address":"0xb4e92c94a2712e98c020a81868264bde52c188cb","BLSPublicKey":"0xd02ec615b916bba4fe7e65a3d79e607aa27bb5a84b0c2f242e9d8f379512cf40051a43030e55aca965d91c905b656d006434d95b7034bfc2e5e2ef7384e8cd640efae740558216f6f9db24c6d1acf755746dfbb68c76961593741105725d5680","UncompressedBLSPublicKey":"0C7GFbkWu6T+fmWj155geqJ7tahLDC8kLp2PN5USz0AFGkMDDlWsqWXZHJBbZW0AZDTZW3A0v8Ll4u9zhOjNZA7650BVghb2+dskxtGs91V0bfu2jHaWFZN0EQVyXVYA+lVOeLQGXz1lgz0vbYb6mSqzym6AX3Ac5BNouTDycB3ot2ID2vj+VaIcyatjjp4A0rfunAjYCMv+ds3TvJzJM5K6e2j7MFMttyEs0wiJA4wjSjh1OCTUT+Ed3AecJAMB"},{"Address":"0xae1ec841923811219b98aceb1db297aade2f46f3","BLSPublicKey":"0xd6e86d5e73db3b3a2c96c6caa1a7e153e17adb13fb541943a44bfa90beab38aa73ad453d918fea2ba57c0a67115d0401c56946d8894f346d796864e9344fd1439dd1345de762f85d7e18e311b35c3cbe492886ef8bc872b4aabfa23c2e38a901","UncompressedBLSPublicKey":"1uhtXnPbOzoslsbKoafhU+F62xP7VBlDpEv6kL6rOKpzrUU9kY/qK6V8CmcRXQQBxWlG2IlPNG15aGTpNE/RQ53RNF3nYvhdfhjjEbNcPL5JKIbvi8hytKq/ojwuOKkBSusWddfKvdk88pyA4P7/Gu3E+852JulDXRQIZjA9uJyQtEFWvktrldMY7tTR2s8AE2bDMgurn7kk463KBCQysAH6kmS80R9ZvdnmNBgG3j16ixTSV6lWIuSkm9OEzBsA"},{"Address":"0x621843731fe33418007c06ee48cfd71e0ea828d9","BLSPublicKey":"0x1cf59939da60cdb9aff09f76e6070a17fa21356ca7016390ef4444243e12ab7ed7a233d7ca48b0d17870ba015a4410014e5cac8d456e03ec2908d347627d5e9ecd496ce990d10900ddc529300eef3d037e48d79f03ad2b6bcd48affe2ddf2681","UncompressedBLSPublicKey":"HPWZOdpgzbmv8J925gcKF/ohNWynAWOQ70REJD4Sq37XojPXykiw0XhwugFaRBABTlysjUVuA+wpCNNHYn1ens1JbOmQ0QkA3cUpMA7vPQN+SNefA60ra81Ir/4t3yYBsKkaWsYAfSNwweFJ1MyB7aG/tnQLkx+KegFbBPR9nG1iO7tEUDzY751KOh/dtVkAlGcc87oygwtkxtYPIV6R7SgD5I5oNCiexlRax7Q/tWOLDK3taK4cMgSKA/51gRcB"},{"Address":"0x2a43f97f8bf959e31f69a894ebd80a88572c8553","BLSPublicKey":"0x1cfe8876c0b89ef15128bb27eb69e7939b4a888b0a81195d5fd1bbda748a29838274e652dcf857f4090bb85343055300ca3e75a980b100403d3b6d34f62c6a86bbd75203391c63dd405725c69241a828e6892f623ed5b35c8dc132b032061201","UncompressedBLSPublicKey":"HP6IdsC4nvFRKLsn62nnk5tKiIsKgRldX9G72nSKKYOCdOZS3PhX9AkLuFNDBVMAyj51qYCxAEA9O2009ixqhrvXUgM5HGPdQFclxpJBqCjmiS9iPtWzXI3BMrAyBhIB78rwTT8unwGkUqgEzvmk+BK7TVWTWoBNpH60oaf4UOFeVcA42kvdjmf3CSSvNQYAMdgYJy23ykObgZKeF3FX0Irx55kf5QuNHYHqQsHdlhI1ojtsBEg3EB76Y1EXB8wA"},{"Address":"0xad682035be6ab6f06e478d2bdab0eab6477b460e","BLSPublicKey":"0xa6fc71d63c5adedb7b30b9e0ba3d83debf86d12ba235c13584a9cbad410f082030427be4f8a9127889979c3eea58860031af128deece487df5aef9d999c8dc2fb51f308eb1ee229e6bbd6860138d4fcf4209eb7bec62ca70dd8643104003c200","UncompressedBLSPublicKey":"pvxx1jxa3tt7MLnguj2D3r+G0SuiNcE1hKnLrUEPCCAwQnvk+KkSeImXnD7qWIYAMa8Sje7OSH31rvnZmcjcL7UfMI6x7iKea71oYBONT89CCet77GLKcN2GQxBAA8IAtfpUiIVD0wrmbCYfwY+CeW/I0evg6qKDewaJu9KFdtFXVkZNQrHRfdSJIx6IPoYAWkiGH4cc458PMSgq4pR1SWyIg/XQTcZ6AykFMqT1XSOLzz9hga4LG5XV1YTODmMA"},{"Address":"0x30d060f129817c4de5fbc1366d53e19f43c8c64f","BLSPublicKey":"0x6b7adb5d01e3fd72ae2c4ff17e6620dc383431e0ebe06c9af5b94207f380287429043e7bbe417b82d0aed2e43dc7b8002bb52886773e4a2c23bf0ebfd401471e8da3cf3a0a7e0949d9ad4de38138a787a975993ba311525ce8be331cd60d6700","UncompressedBLSPublicKey":"a3rbXQHj/XKuLE/xfmYg3Dg0MeDr4Gya9blCB/OAKHQpBD57vkF7gtCu0uQ9x7gAK7Uohnc+Siwjvw6/1AFHHo2jzzoKfglJ2a1N44E4p4epdZk7oxFSXOi+MxzWDWcAOQkguLQ5eOHwtCE0IsZPJcm1hwbw8XMxEnwW8z77akKEvrucUOLpgm6Xvisf+YwBQiylbhiMitry0dLL9/Qc+5F5xI8kdgv77mMKgPKWcnu/yReUUTVvoAK0aiGXn1IA"}]}"#,
    );

    let snapshot: LightConsensusState = serde_json::from_str(&snapshot_json).unwrap();
    let keys: Vec<bls_crypto::PublicKey> = snapshot
        .validators
        .into_iter()
        .map(|validator| bls_crypto::PublicKey::try_from(validator.public_key))
        .collect::<Result<_, _>>()
        .unwrap();
}

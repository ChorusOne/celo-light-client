use crate::errors::Error;
use crate::istanbul::ValidatorData;
use ethereum_types::H256;
use rlp_derive::{RlpDecodable, RlpEncodable};

/// LightConsensusState represents an IBFT consensus state at specified block height
#[derive(RlpDecodable, RlpEncodable, Clone, PartialEq, Debug, Default)]
#[cfg_attr(any(test, feature = "serialize"), derive(serde::Deserialize, serde::Serialize))]
pub struct LightConsensusState {
    /// Block number at which the snapshot was created
    pub number: u64,
    /// Snapshot of current validator set
    pub validators: Vec<ValidatorData>,
    // Hash and aggregated seal are required to validate the header against the validator set
    /// Block H256, or state root
    pub hash: H256,
}

pub fn verify(_lc: &LightConsensusState) -> Result<(), Error> {
    todo!()
}

#[cfg(feature = "web3-support")]
impl From<web3::types::Snapshot<ValidatorData>> for LightConsensusState {
    fn from(snap: web3::types::Snapshot<ValidatorData>) -> Self {
        Self {
            number: snap.number,
            validators: snap.validators,
            hash: snap.hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct JSONRpcResult {
        pub result: LightConsensusState,
    }

    #[test]
    fn deserialize_istanbul_snapshot() {
        let json = String::from(
            r#"{"jsonrpc":"2.0","id":1,"result":{"epoch":17280,"number":16,"hash":"0x8c47e5908b36b1693f5cd1a4db7bcafa99621079afdc3bc389903ef554eb12f4","validators":[{"Address":"0x0cc59ed03b3e763c02d54d695ffe353055f1502d","BLSPublicKey":"0xd5b174e904a2dfbcc181640651bc4c627c86fc9611d92795f18bfe779efb377ae799329419f4ee5a1e5c12103f9a5201257da7a821439d4c86731efe1959bf4076fccf1404c6bca78b6065a466571ef3428f9e203a8ccef519aad1622c9e7401","UncompressedBLSPublicKey":"1bF06QSi37zBgWQGUbxMYnyG/JYR2SeV8Yv+d577N3rnmTKUGfTuWh5cEhA/mlIBJX2nqCFDnUyGcx7+GVm/QHb8zxQExryni2BlpGZXHvNCj54gOozO9Rmq0WIsnnQBbhg6t7mEsooMhSSVnHZzq5QANelsxAMUSEcUqllz8U2RLGohOgQvAOpYUzR/e+QAah76Of6aY4bY6F/k/rEfe21Yt0Fj0x8Y9H7+7woZ8DbAWgdCFQyWERkdjQJVgZcA"},{"Address":"0x3f5084d3d4692cf19b0c98a9b22de614e49e1470","BLSPublicKey":"0x78e6e671e6b9cf6f8fc21fb3f9eb8cf1d64bbdc1fd1213856c7fad711e9b26600f609a2e9cf6322f5cac415eaca793018b9fa902b49d0828258aed10b247e4409c320244adcfc941af6a09e87df18edd5239df5d12e4c38858f9159f5cabb680","UncompressedBLSPublicKey":"eObmcea5z2+Pwh+z+euM8dZLvcH9EhOFbH+tcR6bJmAPYJounPYyL1ysQV6sp5MBi5+pArSdCCgliu0QskfkQJwyAkStz8lBr2oJ6H3xjt1SOd9dEuTDiFj5FZ9cq7YA+k0GAScRF+HD43InS4BHKUsefvGaJ/jFRcfQgY+vOPK+Qgym+SzOF5TMf1Vb5SMArVue/w6c30d1nGC/YVTBAXS/+P70SDrrZ7+hzV59nNpz0R2YMN5si8e4rVeHAQ4B"},{"Address":"0xef0186b8eda17be7d1230eeb8389fa85e157e1fb","BLSPublicKey":"0xdea0e3506854e5d08b932c558da137c231185169d8490cebe070eee539eea00842a3a9787c79859ee82fec298bad0e012e8e4d859b9a13bbd7c97af622a1409276b01d51cb6e8cc36c8ff7a4d2492c35d9c258d37e8eb2cd59b9c9dcd7cc3e80","UncompressedBLSPublicKey":"3qDjUGhU5dCLkyxVjaE3wjEYUWnYSQzr4HDu5TnuoAhCo6l4fHmFnugv7CmLrQ4BLo5NhZuaE7vXyXr2IqFAknawHVHLbozDbI/3pNJJLDXZwljTfo6yzVm5ydzXzD4A9m8k1gg4UK+U4rj1q08bhdzigix+L1UTHv5pZUjGOWrQyAaBOXwSg6nugkeR0H4A4grRhHeXno9H/o4aMSSZdVUh3M8WcXuD3BCDYwfR4mi19coWXCaxajdUvHHhcTgB"},{"Address":"0xedddb60ef5e90fb09707246df193a55df3564c9d","BLSPublicKey":"0xdd794eab9568cea95207f500651867934d5c923bec817eb11536ac8ca4107b127d266ff9e67d407c10a82c4f529ac100080357a69ec30d5e7cf0c68109312b6137ca43cbe7ca4a58a7ff34d1d9c70be34fbf3dce59edd1248709091b80c8b580","UncompressedBLSPublicKey":"3XlOq5VozqlSB/UAZRhnk01ckjvsgX6xFTasjKQQexJ9Jm/55n1AfBCoLE9SmsEACANXpp7DDV588MaBCTErYTfKQ8vnykpYp/800dnHC+NPvz3OWe3RJIcJCRuAyLUAK4/oH7XjixurW4tIaTDDueylLmgf7+3z6TYvdlDRC8LxCvrfya1qUvqa+BEYw6ABVdq21WVXNMiyOz9zpJUGyN8DcK81iD+A26KTri+fLBVzEYrHbbUrihytmg8lsf8A"},{"Address":"0xd5e454462b3fd98b85640977d7a5c783ca162228","BLSPublicKey":"0x49ab79869c44658468d010f831680cf798bc9b7a26145be11b7f6ce547d1e7c13781548663cf710b1d9dfde04fa44700865048c711d0bf2715dc089ed9337e822fb8ce2b889e1512d1de58b17004548e246c14f0fa37988c9edc5cc8f895b400","UncompressedBLSPublicKey":"Sat5hpxEZYRo0BD4MWgM95i8m3omFFvhG39s5UfR58E3gVSGY89xCx2d/eBPpEcAhlBIxxHQvycV3Aie2TN+gi+4ziuInhUS0d5YsXAEVI4kbBTw+jeYjJ7cXMj4lbQADnqJ52NqCkvjsCiCA5j8c8gBkpdRKBNjpNCjhDrTNH8qooXvACJGiaQu7mta2DwAX+yJt/luHZpAdUwNU+Iw4N5/YubHhjKKI63CL8hyjh0QJhZ8hdHsYoFE6w2vPGoA"},{"Address":"0xa4f1bad7996f346c3e90b90b60a1ca8b67b51e4b","BLSPublicKey":"0x855b3dc7efe86a6907eb56a0ab2b663123e28f469359bdf558864b12a867b52e73acf0833534303236c350e2f3489101289db1d0d3f9a9ebe00e18b20109a91e012dda871a62cd513b85f42fbe5684c8131067402c7e944d27ad10c76458fa80","UncompressedBLSPublicKey":"hVs9x+/oamkH61agqytmMSPij0aTWb31WIZLEqhntS5zrPCDNTQwMjbDUOLzSJEBKJ2x0NP5qevgDhiyAQmpHgEt2ocaYs1RO4X0L75WhMgTEGdALH6UTSetEMdkWPoAUAujKi9ue1LO3f4X7qq9maW6K+H+/OcyXfPPwKu2J8B7YkTlwFLnJcsid3E1LPEA8B19AxeKBM5DZ8bsASRoTzKuVnfu+5+YFdAViYGq3giql8e9pt9EKqklILXXFZkB"},{"Address":"0x5b991cc1da0b6d54f8befa9de701d8bc85c92324","BLSPublicKey":"0x1e8b9ec07ff41b4a7b9636470920c7dbc175ac79441f12b3f93da55c26a48c17367ac3421106c955b4913460c9558501410c7dca5cdcd9372a82fa22eed8b9b7c725e390fb97b3a354cafeae18b99b24e07609122ac17924ae40362c48ca3b80","UncompressedBLSPublicKey":"HouewH/0G0p7ljZHCSDH28F1rHlEHxKz+T2lXCakjBc2esNCEQbJVbSRNGDJVYUBQQx9ylzc2Tcqgvoi7ti5t8cl45D7l7OjVMr+rhi5myTgdgkSKsF5JK5ANixIyjsAkKMGKPauOe6BbZOG29i+d8d8kaFvQiDh5R+G+Cgzpcv2LCfny9AXZqCNE4BuqbwATVfZfyek/OL4rVwoBANA2lDMnBW87gHFRlQcvuApzIek2VDXchETus0GvZ7KKKAB"},{"Address":"0x6dfdaa51d146ecff3b97614ef05629ea83f4997e","BLSPublicKey":"0x21eafc9e179b482c450eb092766e79c74bddc02126eb07a56954a74d9cae7bf9aabacd22178a4646fcb2e57ba8826d00c71802db142b2f9d62de6295a9c21835965e0ea5564f37e57241561341bb870723fc828f20100250af23f59f32fa3900","UncompressedBLSPublicKey":"Ier8nhebSCxFDrCSdm55x0vdwCEm6welaVSnTZyue/mqus0iF4pGRvyy5Xuogm0AxxgC2xQrL51i3mKVqcIYNZZeDqVWTzflckFWE0G7hwcj/IKPIBACUK8j9Z8y+jkAI3BFHVQgR55UirXQyKDOxqt4mbhWYnbKYxD5i/UBpFOXXGL3jCY/3+ZCgt25AF0Adpj1V2zEjas1779USgssQhA1dwsDCbfo5CqZaehjmkLgTKxD7PAkR6RNcby2F8oA"},{"Address":"0xd2b16050810600296c9580d947e9d919d0c332ed","BLSPublicKey":"0x3044b1df058de34a35bf47d35a83e346e961a4cb4426fb5ad0ea9fc4d7a2d1f74ca6aa9ff3218f41e23b2c097181a80150f3e56f6ad0aa9d57ec9e71b1538a22761cf430783b73c12c0aed2232318374fdc18347270c1f4ce0fb032a788c0f00","UncompressedBLSPublicKey":"MESx3wWN40o1v0fTWoPjRulhpMtEJvta0OqfxNei0fdMpqqf8yGPQeI7LAlxgagBUPPlb2rQqp1X7J5xsVOKInYc9DB4O3PBLArtIjIxg3T9wYNHJwwfTOD7Ayp4jA8AaH2kIzApgJ9s0I/4cTMl5gZplKKL7CKOZ4RxOY207+tYNPYLN2uHKh50XHUts0cACqrRznYk+3va13L8vJtfCV1JgijRaULPt4b9nZQ8a4gWb0jFeZhncTG4lf/rRdEA"},{"Address":"0xfe144d67068737628effb701207b3eb30ef93c69","BLSPublicKey":"0x796cdfcfcc45ac40ce1c476d419805d59bb3511aebfc8e2c5bfbeb757ae5005713994fa21ced588c3b73d4e469dd1201a41c53b85c2a16a0582b9c0e375dc35b92b4e3fd4fc8a8802337565b840a9537368e16c4c076deebef8e81d463c77101","UncompressedBLSPublicKey":"eWzfz8xFrEDOHEdtQZgF1ZuzURrr/I4sW/vrdXrlAFcTmU+iHO1YjDtz1ORp3RIBpBxTuFwqFqBYK5wON13DW5K04/1PyKiAIzdWW4QKlTc2jhbEwHbe6++OgdRjx3EBRtK9aF6fadDs3R2MRIk76t9XpN6GkowmS1HnD9Mw25B2Jgb8qUpP4/CluTvVHZUBMVEln4mx5NRgiXiXB6FKHR+DspDztfy+DSUl2et3kL0g2nADFPXxjiVNTdE2LI8A"},{"Address":"0x82e64996b355625efeaad12120710706275b5b9a","BLSPublicKey":"0x70534c36b6f06081d3e2228167b4d3f0cf8e235b63850f8b2cbff9ded8c6bb62abb424a9aabfed54ce07143c374b5800d80b39dd3fcce57961858fff9f306bd74dfc576c55a6f8710bd9f35db772a53331fe69cda78ce9ab3ad4c2d2e08a8981","UncompressedBLSPublicKey":"cFNMNrbwYIHT4iKBZ7TT8M+OI1tjhQ+LLL/53tjGu2KrtCSpqr/tVM4HFDw3S1gA2As53T/M5XlhhY//nzBr1038V2xVpvhxC9nzXbdypTMx/mnNp4zpqzrUwtLgiokB4BR+7LHSGCxb4mc6EsKW88cSzImKn43cmN97LiUUOaMVNazAldN0XWQ94DvEtd4AKWQBV6sD9+shWdyrNhHbGLKnmY05WNp35NthrMu7SMkusrVpbyTeGBA0s5WRP/4A"},{"Address":"0x241752a3f65890f4ac3eaec518ff94567954e7b5","BLSPublicKey":"0x51360cc471b15b3d2b1c88d1ed8595cfd89d9b9fab605408ffd1a98b0af0bc1293cbc2fe360a4362ffea907ac22bac0067e3259fb5d4064cc4813ec9449297f7b924034e9631ddc91e088b467eeae97b6133fd275f5f5830a936ba81b2bc9681","UncompressedBLSPublicKey":"UTYMxHGxWz0rHIjR7YWVz9idm5+rYFQI/9GpiwrwvBKTy8L+NgpDYv/qkHrCK6wAZ+Mln7XUBkzEgT7JRJKX97kkA06WMd3JHgiLRn7q6XthM/0nX19YMKk2uoGyvJYBsKXbA0uQSnG0M0u5RqhFascf2nJ4z8gzIiQUgXkJ7EGs+NlqkjQxgBrvcN+PCiQA43FwXP/gJj9EQF1xZiJBGeYXEYE/R4O88R6bIHdL28q8HjOyX8Bvey6rmhlWlocB"},{"Address":"0x1bddeaf571d5da96ce6a127feb3cadadb531f433","BLSPublicKey":"0xae05dd90d739565eae5aca380bef7a2dd3a6bdeb429f2a514864e52d8e60c0a9bc2e022292167ae221adea3ed2d19b009d715d34e3d5924a2cbc11503439c0bbe669e097698cf49a91012640c95dd7a9c77d2e06311a08d338b9541726162000","UncompressedBLSPublicKey":"rgXdkNc5Vl6uWso4C+96LdOmvetCnypRSGTlLY5gwKm8LgIikhZ64iGt6j7S0ZsAnXFdNOPVkkosvBFQNDnAu+Zp4JdpjPSakQEmQMld16nHfS4GMRoI0zi5VBcmFiAAEYRdWZUi1CENwKCpAndSmPgNBUKpH4ZxyzQRVSHXKYp31hIodQcI6tB+FbKLvxoAvbrMHrF42essRYHJD7qIUyoPqrEpGQcDvunVwko9fhH0VGBG1uIAsMtTq6M6/pEA"},{"Address":"0xf86345e9c9b39ab1cbe82d7ad35854f905b8b835","BLSPublicKey":"0xc71dde0acd7a9014a2b4fbc0eee9d56711fa9683b6dfc9469c34ff61d52ba49f983c2de743aae2fa2bf7520de5bb9800333d40b75108cb4208d42597b60600de1bd83b282bed596e36783821af3675fd6267b578eba2613f186a43264d410801","UncompressedBLSPublicKey":"xx3eCs16kBSitPvA7unVZxH6loO238lGnDT/YdUrpJ+YPC3nQ6ri+iv3Ug3lu5gAMz1At1EIy0II1CWXtgYA3hvYOygr7VluNng4Ia82df1iZ7V466JhPxhqQyZNQQgB8cLQ9ypRpsR5T+1V6s7Z3pAaZtbivGqfLlapGhCH1BkKsVL5vtHDSB8poUI9BQsAXIEOYecyIaaB745kTMq7u+xyGamr9yib1Ub4Z8z7bf/T64D5mYUsnJmqKDp89WkA"},{"Address":"0x5c3512b1697302c497b861cbfda158f8a3c5122c","BLSPublicKey":"0x59e868a7daeb24dae2a81a8a397c8b3b4c80b74dbc566c0e8813194c4a0a94850abbc5fb63b56c2c55ad2a65d8534900c9810ecffa3fe2e3b5b759c9716ee3dba16eff9a1785c8b2f1c637322e951a4cc752c31ac400d8222c8ba25c47557680","UncompressedBLSPublicKey":"Wehop9rrJNriqBqKOXyLO0yAt028VmwOiBMZTEoKlIUKu8X7Y7VsLFWtKmXYU0kAyYEOz/o/4uO1t1nJcW7j26Fu/5oXhciy8cY3Mi6VGkzHUsMaxADYIiyLolxHVXYAR4bzq64aC/Y2gpRYHlMphlbugJeBIF8i8KcwstbMnzEqYMN+OKUAMqTtdW5Q1VgB4MtkiIT3dhdO72vky+6t+9tujI0ocJgYY71j4P1RKI0JokO2DyfEFsKLAifIQm4B"},{"Address":"0xa02a692d70fd9a5269397c044aebdf1085ba090f","BLSPublicKey":"0xf5e06a742ce9ee4408a75a38eeb5c06803a87b97709e43308e14fd6a027e048706a31c4ef62084c8758754f95b897801b316a4cb159f60880f69a806ba64cd243727d644d1b10b8890935c2d1d753321f10ea9543418aa7e2b324ff76ab1cc80","UncompressedBLSPublicKey":"9eBqdCzp7kQIp1o47rXAaAOoe5dwnkMwjhT9agJ+BIcGoxxO9iCEyHWHVPlbiXgBsxakyxWfYIgPaagGumTNJDcn1kTRsQuIkJNcLR11MyHxDqlUNBiqfisyT/dqscwARKC4NqRdds/2ONGpvP0zGyCPlXODC0OpJKq93H1bMEyr/h8zNbm/xoLHzHKjSZUBWjkh73KdYWoPY668TUjF7h3PkgGetBpQGxAVm1UOcFGCaqqx6bfYGwMs5HfcOX0B"},{"Address":"0xac91f591f12a8b6531be43e0ccf21cd5fa0e80b0","BLSPublicKey":"0x42cf85912f3b0420d43bc42775d51e77693fcb33a862fbfba18a05d5a8da3cdf7c5f105ee2e3413a9bed07e502af44002e3f48bec159a159db5253ea243d37a2090ff6e9c6214632100f364df129732e57512161b3957cb6f84603afc4859701","UncompressedBLSPublicKey":"Qs+FkS87BCDUO8QnddUed2k/yzOoYvv7oYoF1ajaPN98XxBe4uNBOpvtB+UCr0QALj9IvsFZoVnbUlPqJD03ogkP9unGIUYyEA82TfEpcy5XUSFhs5V8tvhGA6/EhZcBPWe7vGMLooV9Ynz8jVP4bndro3xVLCrkNzuCP1hoj2SqDo3UHGUgl+2MKRfn7XsABKA3VZOP99gN3Bzg0J1KKiRg1WYRvkhDw5RqVC0D1Zb9Xkm4nVdBpPk4HYD5hKEA"},{"Address":"0x718a8ac0943a6d3ffa3ec670086bfb03817ed540","BLSPublicKey":"0xd6592adca17cd2958a84bc1ed35e04aa33abc6ab93de2d3f78cb9e48f3036885fab3282ab7318e4103a4c078659fd9008e4ccde2494eb493f9f851eb1e0d338e2124b393d9491ad2f17dd4f468fa2ab80a7c0c2c4142f6bbb8eb6b33efcf3d01","UncompressedBLSPublicKey":"1lkq3KF80pWKhLwe014EqjOrxquT3i0/eMueSPMDaIX6sygqtzGOQQOkwHhln9kAjkzN4klOtJP5+FHrHg0zjiEks5PZSRrS8X3U9Gj6KrgKfAwsQUL2u7jrazPvzz0BCAeXC3hn7DW05kColTEelPKxxG9NUeicJBNvnc6Pf0BlNY06gzAWl6Uwlh/UxBUBfhmi6WGJgd8nMdqs3W11siCVzhaqBKtgmRd+ujTg2T8AvJnfefRwmXeIUPxqGMkA"},{"Address":"0xb30980ce21679314e240de5cbf437c15ad459eb8","BLSPublicKey":"0x9787a55f5dfa9539d1b0146177f6cd56e7ca3b58a63c36cfcff3b14a78b68d7a5096219162a55e84525080dd834fb000e4157ec4baa6e21c51161f3e8b4aa8f828edcc93e1d8c6fa9a6608d01880064f86c60014c38087a83d8d8acd54375000","UncompressedBLSPublicKey":"l4elX136lTnRsBRhd/bNVufKO1imPDbPz/OxSni2jXpQliGRYqVehFJQgN2DT7AA5BV+xLqm4hxRFh8+i0qo+CjtzJPh2Mb6mmYI0BiABk+GxgAUw4CHqD2Nis1UN1AAJmS3Wv73AJhZWhITr00Zb6XTUoAkm7bAJi+3pVzuLWJ8GVX1KjreJdbv3fC5JygARdzV4YfH0q+HdGOk+BHsuFAnU6PCQQYcLQO3MUy5iaXpH+wSvdD4eUqiCxUMp7kA"},{"Address":"0x99eca23623e59c795eceb0edb666eca9ec272339","BLSPublicKey":"0xeffce8fdc3099723aa7be8d75a6600d3aca652f52aed0191dcb6bfb02cc88b8993fb29627ae5459f8c7affe1349a92001acdc1745ab09d073001521a9ec27158c5bf242db7fdf5745de83e340d7dbe7a9940c207a3e32d27d6406cca25552e00","UncompressedBLSPublicKey":"7/zo/cMJlyOqe+jXWmYA06ymUvUq7QGR3La/sCzIi4mT+ylieuVFn4x6/+E0mpIAGs3BdFqwnQcwAVIansJxWMW/JC23/fV0Xeg+NA19vnqZQMIHo+MtJ9ZAbMolVS4A+e8wuWfHd22Tsog502Hhh5cUVDsGBmfb86twESVcVEb9SkjY3+ExXALVNlsFFIYBuanRc/0PWxHcCqjlksRuL/OmKYwCFSaBCu+2MzS9zPnWAGKiBUOyrjCfLCsg8rAA"},{"Address":"0xc030e92d19229c3efd708cf4b85876543ee1a3f7","BLSPublicKey":"0x4c61d0bf7d775a61a61a5ef721663de02d0b379a1b589fe62d05917f67962459f0f8854f91384ad02b4efeed68ab3301ac31ec73f5f7e4546f329dcfab7666acbcd5022a51b229b7a9fe979d486b83984659a6cddee2bc49317e56af54498781","UncompressedBLSPublicKey":"TGHQv313WmGmGl73IWY94C0LN5obWJ/mLQWRf2eWJFnw+IVPkThK0CtO/u1oqzMBrDHsc/X35FRvMp3Pq3ZmrLzVAipRsim3qf6XnUhrg5hGWabN3uK8STF+Vq9USYcBNJjBNLuW5HYybx+JT1lRZPl5mCYOkE4vkyf7HNvEIw9ZsMm1R1w6OchNjEJmR+cAZYd54Rt9UgnHqDRxGk7fNCbwexW9o9BWw8ou9e0OAKbs4JZGrqYyKqU3eZhwakMB"},{"Address":"0x5c98a3414cb6ff5c24d145f952cd19f5f1f56643","BLSPublicKey":"0xd21657eb1b8e3d38c32873ce326195ee957059968d8926ea219852700d9a884289cd45c5dff6baf2528cb20bb3dc2f00f53c8221eb6c8056b14da363a98e573e4eaf46bc84eed3468364aa24848951c1de38cbe9945e47995e23ce3455dd1880","UncompressedBLSPublicKey":"0hZX6xuOPTjDKHPOMmGV7pVwWZaNiSbqIZhScA2aiEKJzUXF3/a68lKMsguz3C8A9TyCIetsgFaxTaNjqY5XPk6vRryE7tNGg2SqJISJUcHeOMvplF5HmV4jzjRV3RgAnvnhLzrD/sqOLPcahJDF8Ks608gTrUgzL5RZ3VRkOuZYSMxpxQTWBPVyhp0a80YAmrXG9fj1M4r1K9fcC5IHkbJ/WM8SEKB5v1UGxMrKwsgcf1AT+ChkIdv3CWeULjYB"},{"Address":"0x1979b042ae2272197f0b74170b3a6f44c3cc5c05","BLSPublicKey":"0xda25af7a34042ca2e366d359dede467837181b89c6bd8a4d68c47f15952808f7a8c625a524d604a02936b7467f2a6f016a6a9bdf109c3aa4d3c6f6314dc04ff64bc4297e85232b16071eda952b0cf2b297f9a6cbba24e9805c6ea440c04d3f80","UncompressedBLSPublicKey":"2iWvejQELKLjZtNZ3t5GeDcYG4nGvYpNaMR/FZUoCPeoxiWlJNYEoCk2t0Z/Km8Bamqb3xCcOqTTxvYxTcBP9kvEKX6FIysWBx7alSsM8rKX+abLuiTpgFxupEDATT8AjXXDIpuT4uDJTebu7+Ikqbgu0ClJcdadDPOKsg7BoKtt2azY3Oi/uE7zI5jCuxcAEmJ17xLjw7VTIu5d/YH3G8UPToQSxoMsYd8jXNd23SCLRAU0b3JltzCrlX/KGBMB"},{"Address":"0xdb871070334b961804a15f3606fbb4fac7c7f932","BLSPublicKey":"0xced170a03c9d7e505d04e13307eed25e1af3b96317799088881154ab6891fb13ca05a4ddbf7a63c233957502c2ff67002ee5015e2083b498773be347939124008c2f7f856ced900701077b5c4c6a1622c08b0f69fbf7e2e97f92ca8f1c870901","UncompressedBLSPublicKey":"ztFwoDydflBdBOEzB+7SXhrzuWMXeZCIiBFUq2iR+xPKBaTdv3pjwjOVdQLC/2cALuUBXiCDtJh3O+NHk5EkAIwvf4Vs7ZAHAQd7XExqFiLAiw9p+/fi6X+Syo8chwkBhLmJ+lQoa4XTyjASCLudg0RSyM615GichoRPdE8ogEDFdD/S81n2oQwlXgYUcrsAaRQ5rTdelqFQXgfGxjMmzrvqU9JX+dAN2ONwIrBPBxeabUjEYRtRHsRO79POaLYA"},{"Address":"0xc656c97b765d61e0fbcb1197dc1f3a91cc80c2a4","BLSPublicKey":"0xa4c7328ef48a970211572e76c1c3bb2b8a1440233774e12479125808290fbc0e57f276034fa2735f1550fdf7a1b56101c97845c23dba5023aa922c6152028bbc93f39f222b4c58a015db70094efde2f5567f03c7eda6355e8ef9ad9c43065c81","UncompressedBLSPublicKey":"pMcyjvSKlwIRVy52wcO7K4oUQCM3dOEkeRJYCCkPvA5X8nYDT6JzXxVQ/fehtWEByXhFwj26UCOqkixhUgKLvJPznyIrTFigFdtwCU794vVWfwPH7aY1Xo75rZxDBlwBEh/fFaVwAVwsUf3RX/7UHXwTSpSszmd7I0eUpNJK9XMzBBXOr2WFyWFfD7BXB1QBeqPfnGGZwO4lvfH8mvqVVhj5AqPldosPq+8gveUGju1NnVFRvOS04Y2+cI/HGX8B"},{"Address":"0xad95a2f518c197dc9b12ee6381d88bba11f2e0e5","BLSPublicKey":"0x3b07a92815d862c4be414d89f03c9f5354190954ed4c3dd0282e5964aef7954469dee36726340db09aadfba322354c016077b90479c8c5301b362f83b71d9c351ffb57ca39ef611ca4f5d6a8d614411e77a2fcab5de1e6adb9f1a6e3c5f77a80","UncompressedBLSPublicKey":"OwepKBXYYsS+QU2J8DyfU1QZCVTtTD3QKC5ZZK73lURp3uNnJjQNsJqt+6MiNUwBYHe5BHnIxTAbNi+Dtx2cNR/7V8o572EcpPXWqNYUQR53ovyrXeHmrbnxpuPF93oA2jWRwaIAq441UTLFErWUQXWrC15yEEt2w9/ZsH9IUpl/zYQy5fwXY3yzRP2sRhUB//VM49MnhhwD/D/lggezUa8LtWvebIZrm13YSUbBLoE4Te/73LhZEir583ZZBCMB"},{"Address":"0x4d4b5bf033e4a7359146c9ddb13b1c821fe1d0d3","BLSPublicKey":"0xaa6f0e66fe03056fdd2a7a003e0b9f29133f2ebab7c26d3699500b6ac51248f1e3f747a3c3a5b2d11d817b1252679400291d023ae71f3a9f5723b122081a6932075a974f75467aef517944c8d206d307c4a5ebe8fc8aa374baf75dfbece96c00","UncompressedBLSPublicKey":"qm8OZv4DBW/dKnoAPgufKRM/Lrq3wm02mVALasUSSPHj90ejw6Wy0R2BexJSZ5QAKR0COucfOp9XI7EiCBppMgdal091RnrvUXlEyNIG0wfEpevo/IqjdLr3Xfvs6WwADrwiUlODp0pn28BmDQ3zPoNjywlWGZQVHt76eGMbiwD6K27ZTS6nFOZ+YL3Qg1wBKS7fxCijCHfCdKQPOW+1Iu7rTklk/33VNcRwqeID1mQS0wDqKzmnEhWeCUnFPiAA"},{"Address":"0x9c64da169d71c57f85b3d7a17db27c1ce94fbde4","BLSPublicKey":"0x38e7f70fab621ef588c0fe24d9f288387913faaf52a5813f7da47d5fa8a9e32a098e6323ad2c1133221a43bd97b40c01d6cad22a6c7c2969beb123a42b41539c3a38da1b0ed7d762aa9ab67239a1d0afdfc3466287ca4ad33bb69b647f144600","UncompressedBLSPublicKey":"OOf3D6tiHvWIwP4k2fKIOHkT+q9SpYE/faR9X6ip4yoJjmMjrSwRMyIaQ72XtAwB1srSKmx8KWm+sSOkK0FTnDo42hsO19diqpq2cjmh0K/fw0Zih8pK0zu2m2R/FEYA3ijo7x9/howt9Dn0LoSvY+hlhQ/Oq9dId5YXr9f9KHxMbOI8Ibmdiszlu7/vwp4BKf8pBPFbeidBlc4+JfVXanJV1ej78vjwTQzWUgki3+R/wcOBgr+g2feMSHQ61woA"},{"Address":"0xb5f32e89ccad3d396f50da32e0a599e43ce87dd7","BLSPublicKey":"0xacde65212b2f8f1e15bba36f0b2cc0d345678718695e78c7bfbe1566c4691c0352237896e812217184cdc897bf96f500cf616132520278666d00d27a07610de649687cfc28f8c568dce232583004b21d0ab8ea4361d84eb6f9540a1d0b3c3800","UncompressedBLSPublicKey":"rN5lISsvjx4Vu6NvCyzA00VnhxhpXnjHv74VZsRpHANSI3iW6BIhcYTNyJe/lvUAz2FhMlICeGZtANJ6B2EN5klofPwo+MVo3OIyWDAEsh0KuOpDYdhOtvlUCh0LPDgAMIS5fa7/V5aPWMo2HBii9Dcq9hz82KAWe0IHX5eOtmeSwD1Kpd0gdbQ9wi8ShlMA1FfiaUqAEjCIdYKVcSO8v6ncElCFXZh26GMEpBpElhXeYoz/W+EaPIzscdjJKQAA"},{"Address":"0xba40db8ab5325494c9e7e07a4c4720990a39305c","BLSPublicKey":"0x761f7d6dd8c2df51b358a60bd809c693a364b0aefcfa4cb492525ab82b9a3c506e1ba17347f82cff57411ddd6bfd9d01cf9bd337de5916008358cf6f1b411131f43818607db22f439ecb431ae773cea4d39b8c3057b35c64b40beedfdd9e6d80","UncompressedBLSPublicKey":"dh99bdjC31GzWKYL2AnGk6NksK78+ky0klJauCuaPFBuG6FzR/gs/1dBHd1r/Z0Bz5vTN95ZFgCDWM9vG0ERMfQ4GGB9si9DnstDGudzzqTTm4wwV7NcZLQL7t/dnm0AcfoEFN29oODVFHCpOeBWX7AWDcic/WluThihkQC1XaydbxMxCeNHt7FDx4XYLpgBfAGJBbgXVc9eqVdJoS/C0u7aIAmfdfAS/ds1qhaeL3FzyVh1X4+Il34fsBR7++EA"}]}}"#,
        );

        let result: JSONRpcResult = serde_json::from_str(&json).unwrap();
    }
}

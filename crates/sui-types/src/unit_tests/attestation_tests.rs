// // Copyright (c) Mysten Labs, Inc.
// // SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;

use super::attestation_verify_inner;
use fastcrypto::encoding::Encoding;
use fastcrypto::encoding::Hex;

#[test]
fn attestation_parse() {
    let curr = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let res = attestation_verify_inner(
        &Hex::decode("8444a1013822a0591121a9696d6f64756c655f69647827692d30663733613462346362373463633966322d656e633031393265343138386665663738316466646967657374665348413338346974696d657374616d701b000001932d1239ca6470637273b0005830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000015830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000025830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000035830639a8b65f68b0223cbb14a0032487e5656d260434e3d1a10e7ec1407fb86143860717fc8afee90df7a1604111709af460458309ab5a1aba055ee41ee254b9b251a58259b29fa1096859762744e9ac73b5869b25e51223854d9f86adbb37fe69f3e5d1c0558300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000658300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000758300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000858300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000958300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b636572746966696361746559027e3082027a30820201a00302010202100192e4188fef781d0000000067366a8d300a06082a8648ce3d04030330818e310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533139303706035504030c30692d30663733613462346362373463633966322e75732d656173742d312e6177732e6e6974726f2d656e636c61766573301e170d3234313131343231323432365a170d3234313131353030323432395a308193310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753313e303c06035504030c35692d30663733613462346362373463633966322d656e63303139326534313838666566373831642e75732d656173742d312e6177733076301006072a8648ce3d020106052b810400220362000442e0526fc41af71feac64fc6f68a8ac8aae831a9e945ab7d482b842acaf05d6b762d00cbc2115da270187c44597b1c16dcf497c70e543b41612e9041ea143d11d58bd1c847496e5d41ec78a49fe445348cf9a47af9387e0451d9ec145b56ec12a31d301b300c0603551d130101ff04023000300b0603551d0f0404030206c0300a06082a8648ce3d0403030367003064023078001466c0c64293b9bde3d0834edb67ff18417f6075a8f7d137701e10164ce6cf45c508bf383ed0d8d41c51a5977a43023033cb8e4a6ad2686b86c2533accbab5dd5e98cf25d3612b1a48502f327ce00acc921641242d5a3a27d222df1f7dfc3e2c68636162756e646c65845902153082021130820196a003020102021100f93175681b90afe11d46ccb4e4e7f856300a06082a8648ce3d0403033049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c61766573301e170d3139313032383133323830355a170d3439313032383134323830355a3049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b8104002203620004fc0254eba608c1f36870e29ada90be46383292736e894bfff672d989444b5051e534a4b1f6dbe3c0bc581a32b7b176070ede12d69a3fea211b66e752cf7dd1dd095f6f1370f4170843d9dc100121e4cf63012809664487c9796284304dc53ff4a3423040300f0603551d130101ff040530030101ff301d0603551d0e041604149025b50dd90547e796c396fa729dcf99a9df4b96300e0603551d0f0101ff040403020186300a06082a8648ce3d0403030369003066023100a37f2f91a1c9bd5ee7b8627c1698d255038e1f0343f95b63a9628c3d39809545a11ebcbf2e3b55d8aeee71b4c3d6adf3023100a2f39b1605b27028a5dd4ba069b5016e65b4fbde8fe0061d6a53197f9cdaf5d943bc61fc2beb03cb6fee8d2302f3dff65902c2308202be30820245a003020102021100ab314210a819b4842e3be045e7daddbe300a06082a8648ce3d0403033049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c61766573301e170d3234313131333037333235355a170d3234313230333038333235355a3064310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533136303406035504030c2d343834633637303131656563376235332e75732d656173742d312e6177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b8104002203620004cbd3e3fe8793852d952a214ee1c7f17e13eff238c5952ffc6c48f2b8e70beec10194585089829f4818d012a6061cdc9f4d8c5a67aada1233f75b65d3f7704e1c02460cfcc74f0e94193c8d4030f6d1662de0427836c1d32c571c919230fae73aa381d53081d230120603551d130101ff040830060101ff020102301f0603551d230418301680149025b50dd90547e796c396fa729dcf99a9df4b96301d0603551d0e04160414b5f0f617140aa7057c7977f361eee896fd9a58b4300e0603551d0f0101ff040403020186306c0603551d1f046530633061a05fa05d865b687474703a2f2f6177732d6e6974726f2d656e636c617665732d63726c2e73332e616d617a6f6e6177732e636f6d2f63726c2f61623439363063632d376436332d343262642d396539662d3539333338636236376638342e63726c300a06082a8648ce3d04030303670030640230038362cf11e189755d6a2306d728a7f356740eefe623d5e0e9e7c33c1b061ade2224127ac3a2e4bce60b43fc8c53326902306aceccf6f45a8d5c066bd10ce3ffaeeebdee56eedb86deb18ea22172c07196750924dd8f4656c70bd95eb6714cb8ecdd59031a308203163082029ba0030201020211009a0f4f29c1649826edb5b5f9f93b6326300a06082a8648ce3d0403033064310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533136303406035504030c2d343834633637303131656563376235332e75732d656173742d312e6177732e6e6974726f2d656e636c61766573301e170d3234313131343034323230325a170d3234313132303033323230325a308189313c303a06035504030c33373532313933346262636164353432622e7a6f6e616c2e75732d656173742d312e6177732e6e6974726f2d656e636c61766573310c300a060355040b0c03415753310f300d060355040a0c06416d617a6f6e310b3009060355040613025553310b300906035504080c0257413110300e06035504070c0753656174746c653076301006072a8648ce3d020106052b810400220362000496f4565c489625767e8e2d3006ba06bd48ba3e384027a205b93d1ad4958128887c38ddbb2f4922888708ef0985e1e5d3bd73b33f86785ac66a204eed3a6b663686434f64e19fb39cd7b33068edb2108b79774a961e7080cb1b4eaa60a5e63e22a381ea3081e730120603551d130101ff040830060101ff020101301f0603551d23041830168014b5f0f617140aa7057c7977f361eee896fd9a58b4301d0603551d0e0416041484b6dc9994365b56081f5d1bc8ee21f58e45d7df300e0603551d0f0101ff0404030201863081800603551d1f047930773075a073a071866f687474703a2f2f63726c2d75732d656173742d312d6177732d6e6974726f2d656e636c617665732e73332e75732d656173742d312e616d617a6f6e6177732e636f6d2f63726c2f34396230376261342d303533622d346435622d616434612d3364626533653065396637652e63726c300a06082a8648ce3d0403030369003066023100d00c2999e66fbcce624d91aedf41f5532b04c300c86a61d78ed968716a7f7ff565e2c361f4f46fe5c5486a9d2bfe0d60023100bc46872a45820fb552b926d420d4f6a1be831bb26821d374e95bff5ed042b3313465b5b4cde79f16f6a57bd5b541353c5902c3308202bf30820245a003020102021500eaa3f0b662c2a61c96f94194fa33d5baf26eeb84300a06082a8648ce3d040303308189313c303a06035504030c33373532313933346262636164353432622e7a6f6e616c2e75732d656173742d312e6177732e6e6974726f2d656e636c61766573310c300a060355040b0c03415753310f300d060355040a0c06416d617a6f6e310b3009060355040613025553310b300906035504080c0257413110300e06035504070c0753656174746c65301e170d3234313131343130313032345a170d3234313131353130313032345a30818e310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533139303706035504030c30692d30663733613462346362373463633966322e75732d656173742d312e6177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b81040022036200040fe46adf864a558a00a9ca4b64ece5ba124ed1d29656a1f16ca71d0dc8fca56b0fb15aafd309f6258374e8c7b4a5b0521c76d1812a7873474dae9322aef1cd782db19fc2ece4d36fa08acbe65e4bec2a3cfe70960d179778ea7e7711f827b36ea366306430120603551d130101ff040830060101ff020100300e0603551d0f0101ff040403020204301d0603551d0e041604143e40d423bf86e9565c378487843389bd2f471a56301f0603551d2304183016801484b6dc9994365b56081f5d1bc8ee21f58e45d7df300a06082a8648ce3d0403030368003065023100c2767f29cc6e40e087617cf680d81e3b77962c29d8ace426b3c4a62a560354da73de6f80986d44da2593a3c268fea94302306056e2f3c88c30170c4940f578acc279a01fe689123e81def4f8c313e1f0cbc44a562a171d12810e847e441aee233f676a7075626c69635f6b6579f669757365725f6461746158205a264748a62368075d34b9494634a3e096e0e48f6647f965b81d2a653de684f2656e6f6e6365f65860284d57f029e1b3beb76455a607b9a86360d6451370f718a0d7bdcad729eea248c25461166ab684ad31fb52713918ee3e401d1b56251d6f9d85bf870e850e0b47559d17091778dbafc3d1989a94bd54c0991053675dcc3686402b189172aae196").unwrap(),
        &Hex::decode("5a264748a62368075d34b9494634a3e096e0e48f6647f965b81d2a653de684f2").unwrap(),
        &Hex::decode("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        &Hex::decode("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        &Hex::decode("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        1731627987382,
    );
    println!("{} {:?}", curr, res);
    assert!(res.is_ok());
}

#[test]
fn test_over_wrong_pk_in_user_data() {}

#[test]
fn test_over_wrong_pcrs() {}

#[test]
fn test_over_certificate_expired() {
    let now = 1731622252 - (3 * 60 * 60); // subtract 3 hours in seconds
    let res = attestation_verify_inner(
        &Hex::decode("8444a1013822a0591123a9696d6f64756c655f69647827692d30663733613462346362373463633966322d656e633031393265343138386665663738316466646967657374665348413338346974696d657374616d701b000001932792c9136470637273b0005830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000015830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000025830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000035830639a8b65f68b0223cbb14a0032487e5656d260434e3d1a10e7ec1407fb86143860717fc8afee90df7a1604111709af460458309ab5a1aba055ee41ee254b9b251a58259b29fa1096859762744e9ac73b5869b25e51223854d9f86adbb37fe69f3e5d1c0558300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000658300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000758300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000858300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000958300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f58300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b636572746966696361746559027f3082027b30820201a00302010202100192e4188fef781d0000000067352387300a06082a8648ce3d04030330818e310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533139303706035504030c30692d30663733613462346362373463633966322e75732d656173742d312e6177732e6e6974726f2d656e636c61766573301e170d3234313131333232303930385a170d3234313131343031303931315a308193310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753313e303c06035504030c35692d30663733613462346362373463633966322d656e63303139326534313838666566373831642e75732d656173742d312e6177733076301006072a8648ce3d020106052b8104002203620004798b8c7777b59f89cbd2c81f50aaa3fcb5ffacc59abc97c9fe6bd894c8ae1b18d18071f4ac3e743b15ecdc641b75ee287a3ab3ebb8c3b37c7d8be8e4e85140269f7057b8dbb0e812283f178dede848ea74b070d67ce621ec73381426fad115d2a31d301b300c0603551d130101ff04023000300b0603551d0f0404030206c0300a06082a8648ce3d0403030368003065023071158599f7dcfae250c3e23f5f9ba5f6005e217a6f7c92b1efb6a34d66955605a6a14e27ea5826dc056795ea23c9196a023100b172e3aadd314c5408bdc2a69b3f578bd3fde0203676c51d3e004505060eaedba2aa35ab00985c04dca89a2ae2b9b77c68636162756e646c65845902153082021130820196a003020102021100f93175681b90afe11d46ccb4e4e7f856300a06082a8648ce3d0403033049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c61766573301e170d3139313032383133323830355a170d3439313032383134323830355a3049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b8104002203620004fc0254eba608c1f36870e29ada90be46383292736e894bfff672d989444b5051e534a4b1f6dbe3c0bc581a32b7b176070ede12d69a3fea211b66e752cf7dd1dd095f6f1370f4170843d9dc100121e4cf63012809664487c9796284304dc53ff4a3423040300f0603551d130101ff040530030101ff301d0603551d0e041604149025b50dd90547e796c396fa729dcf99a9df4b96300e0603551d0f0101ff040403020186300a06082a8648ce3d0403030369003066023100a37f2f91a1c9bd5ee7b8627c1698d255038e1f0343f95b63a9628c3d39809545a11ebcbf2e3b55d8aeee71b4c3d6adf3023100a2f39b1605b27028a5dd4ba069b5016e65b4fbde8fe0061d6a53197f9cdaf5d943bc61fc2beb03cb6fee8d2302f3dff65902c3308202bf30820244a003020102021027ba28d7a20b82c9501b83e907245e96300a06082a8648ce3d0403033049310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c03415753311b301906035504030c126177732e6e6974726f2d656e636c61766573301e170d3234313130383038303734365a170d3234313132383039303734365a3064310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533136303406035504030c2d356235383665343733353365393830342e75732d656173742d312e6177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b8104002203620004ba771bac15495418aec8e2b381cbff4c7fc827fa8cdf92c787513f7f661fcc2e1866b37757323e4f0f6c041ba1bccc8e42f0535e9f5617648e6596dba6924d3007292decfbd8cfcf77259903a689cf19979a592018864655946798a1d5a9f1cba381d53081d230120603551d130101ff040830060101ff020102301f0603551d230418301680149025b50dd90547e796c396fa729dcf99a9df4b96301d0603551d0e041604140c949bc3b12640ef968371bcfc9679538a4811b8300e0603551d0f0101ff040403020186306c0603551d1f046530633061a05fa05d865b687474703a2f2f6177732d6e6974726f2d656e636c617665732d63726c2e73332e616d617a6f6e6177732e636f6d2f63726c2f61623439363063632d376436332d343262642d396539662d3539333338636236376638342e63726c300a06082a8648ce3d0403030369003066023100c5bbd86d806aa21b7ddf7ef0a112c6d8584d70773efed83015f8fcfbda3d8555a215454faabda5a3270c3941e956814b023100b5ca56e8bc9d8a8020e51e2839c763dd7836603943c54498bb823c9119c20da699c1c9cfb56394d3b8ccb9281cf1271f59031a308203163082029ba003020102021100a27d816cc9947f058c60f2c6a4a3876c300a06082a8648ce3d0403033064310b3009060355040613025553310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533136303406035504030c2d356235383665343733353365393830342e75732d656173742d312e6177732e6e6974726f2d656e636c61766573301e170d3234313131333036323132365a170d3234313131383233323132355a308189313c303a06035504030c33643866343936613631386333366165642e7a6f6e616c2e75732d656173742d312e6177732e6e6974726f2d656e636c61766573310c300a060355040b0c03415753310f300d060355040a0c06416d617a6f6e310b3009060355040613025553310b300906035504080c0257413110300e06035504070c0753656174746c653076301006072a8648ce3d020106052b8104002203620004e83ae31ddd7efd78042402dc857a26be0b4a9f0dd9694270416d9727abc28f7ba5fe8b004c13aa11d5b71e25fcc3ac47915a98a58054e4d1f7bda84c059aec1bd113222810299a34fd66e8aed090b09e675600862bcf1a5aa878d790a22f2423a381ea3081e730120603551d130101ff040830060101ff020101301f0603551d230418301680140c949bc3b12640ef968371bcfc9679538a4811b8301d0603551d0e04160414dbe8aa9bc80145d672dd75307edbe05f85c0b701300e0603551d0f0101ff0404030201863081800603551d1f047930773075a073a071866f687474703a2f2f63726c2d75732d656173742d312d6177732d6e6974726f2d656e636c617665732e73332e75732d656173742d312e616d617a6f6e6177732e636f6d2f63726c2f65333562623161362d376265362d346265392d396633322d3264643139373938393437662e63726c300a06082a8648ce3d0403030369003066023100903d1cdefdd6ba3701c57d5d34b579b07b0c9efec5bf1a9572f59862efced769f6ffc001e184e5491e700f9cab6cbbe00231009b2061cbc56215c20689e36df43a3aed0bcc2fb1d18aab309d65597e822cc8afc428435f66789c03e08645f51fb472595902c3308202bf30820245a003020102021500aefa8defc39a5c32a5cd2979d2d74da408a5fe39300a06082a8648ce3d040303308189313c303a06035504030c33643866343936613631386333366165642e7a6f6e616c2e75732d656173742d312e6177732e6e6974726f2d656e636c61766573310c300a060355040b0c03415753310f300d060355040a0c06416d617a6f6e310b3009060355040613025553310b300906035504080c0257413110300e06035504070c0753656174746c65301e170d3234313131333130313032325a170d3234313131343130313032325a30818e310b30090603550406130255533113301106035504080c0a57617368696e67746f6e3110300e06035504070c0753656174746c65310f300d060355040a0c06416d617a6f6e310c300a060355040b0c034157533139303706035504030c30692d30663733613462346362373463633966322e75732d656173742d312e6177732e6e6974726f2d656e636c617665733076301006072a8648ce3d020106052b81040022036200040fe46adf864a558a00a9ca4b64ece5ba124ed1d29656a1f16ca71d0dc8fca56b0fb15aafd309f6258374e8c7b4a5b0521c76d1812a7873474dae9322aef1cd782db19fc2ece4d36fa08acbe65e4bec2a3cfe70960d179778ea7e7711f827b36ea366306430120603551d130101ff040830060101ff020100300e0603551d0f0101ff040403020204301d0603551d0e041604143e40d423bf86e9565c378487843389bd2f471a56301f0603551d23041830168014dbe8aa9bc80145d672dd75307edbe05f85c0b701300a06082a8648ce3d0403030368003065023076202ca0ab91a30a76367c211e9998b8c710bf5aec5bbe6277fb113861772967f4b39985765a265c4520576bc54ffcd002310095ee7dce931a349a265f50acc1d644a8e1d0545a518b64bc1f6bc5c33898e52f6299650c64b78fa18697184811b6a04c6a7075626c69635f6b6579f669757365725f6461746158205a264748a62368075d34b9494634a3e096e0e48f6647f965b81d2a653de684f2656e6f6e6365f65860cf61d84badba2c9b66d19a181365ec8b0a287c70e462d604d026888ea78661f2e2c79e309f030974984bdca6951e8d7e92e23fad90d6133af0de4922596a042c42604c4cb55bcf14138b4e00a32a55e10288cac11fe1a528f33aab8e015028c8").unwrap(),
        &Hex::decode("5a264748a62368075d34b9494634a3e096e0e48f6647f965b81d2a653de684f2").unwrap(),
        &Hex::decode("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        &Hex::decode("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        &Hex::decode("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
        now,
    );
    // todo: assert cert error
    assert!(res.is_err());
}

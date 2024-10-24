use bitcoin::hashes::Hash;
use bitcoin::key::{Secp256k1, TapTweak, UntweakedPublicKey};
use bitcoin::script::Script;
use bitcoin::{TapLeafHash, TapNodeHash, XOnlyPublicKey};
use hex::decode;

fn main() {
    // example transaction: https://blockstream.info/tx/905ecdf95a84804b192f4dc221cfed4d77959b81ed66013a7e41a6e61e7ed530
    let taproot_pk_expect = "2fcad7470279652cc5f88b8908678d6f4d57af5627183b03fc8404cb4e16d889";
    let taproot_pk_expect = decode(taproot_pk_expect).expect("decode taproot_pk_expect failed");
    let taproot_pk_expect = XOnlyPublicKey::from_slice(&taproot_pk_expect)
        .expect("taproot_pk_expect from slice failed");

    let control = "c02e44c9e47eaeb4bb313adecd11012dfad435cd72ce71f525329f24d75c5b9432774e148e9209baf3f1656a46986d5f38ddf4e20912c6ac28f48d6bf747469fb1";
    let tap_spend_script = "20febe583fa77e49089f89b78fa8c116710715d6e40cc5f5a075ef1681550dd3c4ad20d0fa46cb883e940ac3dc5421f05b03859972639f51ed2eccbf3dc5a62e2e1b15ac";
    let tap_spend_script = decode(tap_spend_script).expect("decode hex failed");
    let tap_spend_script = Script::from_bytes(&tap_spend_script);

    println!("Redeem Script: {}", tap_spend_script);

    let script_hash =
        TapLeafHash::from_script(&tap_spend_script, bitcoin::taproot::LeafVersion::TapScript);

    let node_hash = &control[66..66 + 64];
    let node_hash = decode(node_hash).expect("decode node hash failed");
    let node_hash =
        TapNodeHash::from_byte_array(node_hash.try_into().expect("node hash must be 32 bytes"));
    let merkle_root = TapNodeHash::from_node_hashes(script_hash.into(), node_hash);

    let internal_key = &control[2..66];
    let internal_key = decode(internal_key).expect("decode internal key failed");
    let internal_key =
        UntweakedPublicKey::from_slice(&internal_key).expect("decode internal key failed");

    let secp = Secp256k1::new();

    let taproot_pk = internal_key.tap_tweak(&secp, Some(merkle_root)).0;

    assert_eq!(taproot_pk_expect, taproot_pk.into(), "taproot pk not match");
}

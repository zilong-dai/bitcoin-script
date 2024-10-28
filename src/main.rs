use bitcoin::consensus::{deserialize, serialize};
use bitcoin::hashes::Hash;
use bitcoin::key::{Secp256k1, TapTweak, UntweakedPublicKey};
use bitcoin::script::Script;
use bitcoin::sighash::SighashCache;
use bitcoin::{TapLeafHash, TapNodeHash, Transaction, XOnlyPublicKey};
use hex::decode;

use sha2::{Digest, Sha256};

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn dsha256(data: &[u8]) -> [u8; 32] {
    sha256(&sha256(data))
}

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

    // println!("Redeem Script: {}", tap_spend_script);

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

    // example transaction: cityrollup p2sh transaction
    let tx = "0200000002952da86dd9c8e4f2587be7adfa9ebe531cc05a8e733f7bd6c02c7ac9f03503b300000000fd020330a1cf4fd86ec455bbe8b983c0dd64abe78ef065724960fecce845873a9df15e10e16c60e948fddd722bd68ed469cca20e30a9cb9215f93c6d926a22367fab2e95d54560a06412ca51304da82d2ee75521a6f58996298c14fe566795d2f654efda0130f23b6a58474bfc3e89499bce8bf7b280b2d84707ffa7f86bc9c94a8174b99f7df7be7e531e8ea759f60abdb4864ad7023016e9e0aa1e58d75831dd90aef95657a5a5e96e9196e6a02f6cb8c6371d311d5654dda6dfd522626f4c90aa0861b067024c50b9d96fecdd99ff136355dc90772a2ef7788afe1f8a332b34e8a30402477f8e38e874eba3946f6f4ba8c794a09d266507dd66ecb824f911036f2b6bf63ee7a5feb52105bdf466dac06ca427711d5b52c24de9012000f3a40258113d7544ec3a1c548047ab9a14e5488320414a311d8de59b7414007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a69cab1f76133032b85ee7e850977dd1fe578f3d9663bb43a08502a8fd7cb8c7f79c39fbe49f9cee082bf68dfd65e70ccdbfb4c6f834d5dcb3e4619bc44de9ca8aca12b502e74b7b50451b36d6d6d6d6d6d51ffffffffa75fe7d127ffba6a51a6aa750aec49d6861b7e3cc809f259cfcf3d95c71574b900000000fd02033091d0f9aff7bd3da433eb5706a7914bc49d19ea781a9b8eee183929cdfc7a79773a244992d5afd3e9ddc61d70f73b0194300739ce49ac859b19f03d3c0c2bbb7ffb24e64c32e4e632fe74e3a53f5e5aa10ee3551061b3818aa3e126d960cb593f163035c36460f49449043c7892acebd0518122551501a7abd5a00df0962ce1f6977399a5ef4762c0d80ceec2c9833891c617301120658e06a7ede8415aded94c242e8a6d661732a1b8826114814421aff13fec59f847961ae3181b7a533d40f4aa3a924c50b9d96fecdd99ff136355dc90772a2ef7788afe1f8a332b34e8a30402477f8e38e874eba3946f6f4ba8c794a09d266507dd66ecb824f911036f2b6bf63ee7a5feb52105bdf466dac06ca427711d5b52c24de9012000f3a40258113d7544ec3a1c548047ab9a14e5488320414a311d8de59b7414007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a69cab1f76133032b85ee7e850977dd1fe578f3d9663bb43a08502a8fd7cb8c7f79c39fbe49f9cee082bf68dfd65e70ccdbfb4c6f834d5dcb3e4619bc44de9ca8aca12b502e74b7b50451b36d6d6d6d6d6d51ffffffff01003e6e560200000017a9149faff0ec8c48761a48a8023ed8fd7b8af103c0888700000000";
    let tx = decode(tx).expect("decode tx failed");
    let mut tx: Transaction = deserialize::<Transaction>(&tx).expect("parse tx failed");

    let redeem_script = tx.input[0].script_sig.clone();
    let redeem_script = redeem_script
        .redeem_script()
        .expect("get redeem script failed");

    let redeem_script_expected = decode("2000f3a40258113d7544ec3a1c548047ab9a14e5488320414a311d8de59b7414007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a69cab1f76133032b85ee7e850977dd1fe578f3d9663bb43a08502a8fd7cb8c7f79c39fbe49f9cee082bf68dfd65e70ccdbfb4c6f834d5dcb3e4619bc44de9ca8aca12b502e74b7b50451b36d6d6d6d6d6d51").expect("pay script");
    let redeem_script_expected = Script::from_bytes(&redeem_script_expected);

    let sighash_cache = SighashCache::new(tx.clone());
    let sighash_expected = sighash_cache
        .legacy_signature_hash(0, redeem_script, 1)
        .expect("calc sighash failed");

    assert_eq!(
        redeem_script, redeem_script_expected,
        "redeem script not match"
    );

    let index = 0;
    for (i, input) in tx.input.iter_mut().enumerate() {
        if i != index {
            input.script_sig = Script::new().into();
        } else {
            input.script_sig = redeem_script.into();
        }
    }
    let ser_tx = "0200000002952da86dd9c8e4f2587be7adfa9ebe531cc05a8e733f7bd6c02c7ac9f03503b300000000fde9012000f3a40258113d7544ec3a1c548047ab9a14e5488320414a311d8de59b7414007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a69cab1f76133032b85ee7e850977dd1fe578f3d9663bb43a08502a8fd7cb8c7f79c39fbe49f9cee082bf68dfd65e70ccdbfb4c6f834d5dcb3e4619bc44de9ca8aca12b502e74b7b50451b36d6d6d6d6d6d51ffffffffa75fe7d127ffba6a51a6aa750aec49d6861b7e3cc809f259cfcf3d95c71574b90000000000ffffffff01003e6e560200000017a9149faff0ec8c48761a48a8023ed8fd7b8af103c0888700000000";

    assert_eq!(ser_tx, hex::encode(serialize(&tx)), "sigall tx failed");

    let ser_tx_sighash = format!("{}{}", ser_tx, "01000000");

    let sighash = hex::decode(ser_tx_sighash).expect("decode sighash failed");
    let sighash = dsha256(&sighash);

    assert_eq!(
        &sighash,
        sighash_expected.as_byte_array(),
        "sighash not match"
    );

    // example transaction: https://www.blockchain.com/explorer/transactions/btc/6adf27a500eb592d49a1732ab338e38815f6d4986636566755a68b1147c57d18
    let p2pkh_tx = "0200000002952da86dd9c8e4f2587be7adfa9ebe531cc05a8e733f7bd6c02c7ac9f03503b300000000fde9012000f3a40258113d7544ec3a1c548047ab9a14e5488320414a311d8de59b7414007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a69cab1f76133032b85ee7e850977dd1fe578f3d9663bb43a08502a8fd7cb8c7f79c39fbe49f9cee082bf68dfd65e70ccdbfb4c6f834d5dcb3e4619bc44de9ca8aca12b502e74b7b50451b36d6d6d6d6d6d51ffffffffa75fe7d127ffba6a51a6aa750aec49d6861b7e3cc809f259cfcf3d95c71574b90000000000ffffffff01003e6e560200000017a9149faff0ec8c48761a48a8023ed8fd7b8af103c0888700000000";
    let p2pkh_tx = decode(p2pkh_tx).expect("decode tx failed");
    let mut p2pkh_tx: Transaction = deserialize::<Transaction>(&p2pkh_tx).expect("parse tx failed");

    let p2pkh_script = "76a9146f51100d3334c06fffdd47c45123879357978d8b88ac";
    let p2pkh_script = hex::decode(p2pkh_script).expect("decode script failed");
    let p2pkh_script = Script::from_bytes(&p2pkh_script);

    let p2pkh_sighash_cache = SighashCache::new(p2pkh_tx.clone());
    let p2pkh_sighash_expected = p2pkh_sighash_cache
        .legacy_signature_hash(0, p2pkh_script, 1)
        .expect("calc sighash failed");

    let index = 0;
    for (i, input) in p2pkh_tx.input.iter_mut().enumerate() {
        if i != index {
            input.script_sig = Script::new().into();
        } else {
            input.script_sig = p2pkh_script.into();
        }
    }

    let mut p2pkh_sighash = serialize(&p2pkh_tx);
    p2pkh_sighash.extend_from_slice(&1u32.to_le_bytes());
    let p2pkh_sighash = dsha256(&p2pkh_sighash);
    assert_eq!(
        &p2pkh_sighash,
        p2pkh_sighash_expected.as_byte_array(),
        "calc sighash failed",
    );
}

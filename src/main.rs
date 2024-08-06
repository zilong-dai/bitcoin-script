use bitcoin::script::Script;
use hex::decode;

// use bc::SigScript;

fn main() {
    // OP_PUSHDATA1 b9d96fecdd99ff136355dc90772a2ef7788afe1f8a332b34e8a30402477f8e38e874eba3946f6f4ba8c794a09d266507dd66ecb824f911036f2b6bf63ee7a5feb52105bdf466dac06ca427711d5b52c2
    // OP_PUSHDATA2 see redeem script
    let script_hex = "30edeae0b0c2244b073dd5e6331ca6b1867909edb7e486dd363ed2188f09ee25194164d7f7b8ec910192f63d97d2aab01430dca757b5d81ec7fd27ca9e35ede8de527af54e082ba6dd79af6aa27d014bfe47b04031ae6a56df8665ffe8559dbf0906308cd29973c112bccb3a95398df8df8cb2f5483774f07bed87ff3a70beee948d940559cc77a1a8d9cc8f2173127ac3a21830c0d6c27245f9461f44413f5690773435d8c98af6a8baedb328c805794462009ed5d485709745941027a52a8e17cebb874c50b9d96fecdd99ff136355dc90772a2ef7788afe1f8a332b34e8a30402477f8e38e874eba3946f6f4ba8c794a09d266507dd66ecb824f911036f2b6bf63ee7a5feb52105bdf466dac06ca427711d5b52c24de9012066cf50f0fbac9928f8bcda00df07535d4cbb61b29b656d9a714fc1cca1f187007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a69cab1f76133032b85ee7e850977dd1fe578f3d9663bb43a08502a8fd7cb8c7f79c39fbe49f9cee082bf68dfd65e70ccdbfb4c6f834d5dcb3e4619bc44de9ca8aca12b502e74b7b50451b36d6d6d6d6d6d51";
    let script_bytes = decode(script_hex).expect("decode hex failed");
    let script = Script::from_bytes(&script_bytes);

    println!("Script: {}", script);

    let redeem_script_hex = "2066cf50f0fbac9928f8bcda00df07535d4cbb61b29b656d9a714fc1cca1f187007c76a82081c202329a17f5756142a72734f6832f7784e22b1e17260901039f504f9098a3884c5090c10d96e70d136fac9a3634428f3792e9b8d02eb903783f71fc8e6937f7f8b1360c8bee9146041a79251614eee9689073af20af273a280e841b078837e65a862279849c1251e1761bb6ac35f2649ea34c50fe63d02ab4af4ab3448fe5e8d4fff135d510a38558c9daee4ae190303c18368ce2ba9fd21e8d1ab0f617a9d249621b56f224b69f6c3a3e008a40b3aea5a5ab77eed73711e93c896023475b27e3a4f6124c50189647ab2c8de2fc37ef189f803704cc55087bfe1c56dcabbb2f343dc145a0470d18317696e1023a7574e8745e0ea301cc6cd679a61133a1c560d5aa3d38d91a999a8f666109495402c553bd7c82056b4c50bf358e5097046487c370c1dd6781dc11d6518717e23b334d4b09892a9763f09059687a7c136f6189568edd6d6f357c1c199a39fa0f723d2218762766f67fa8171b10e8b7e5dd88155651d37ca6b59c754c50092d3dfea8804a";
    let redeem_script_bytes = decode(redeem_script_hex).expect("decode hex failed");
    let redeem_script = Script::from_bytes(&redeem_script_bytes);

    println!("Redeem Script: {}", redeem_script);

    // OP_HASH160 OP_PUSHBYTES_20 62d3deeddad02451dba3af4456e077da43566e05 OP_EQUAL
    // redeem script hash: 62d3deeddad02451dba3af4456e077da43566e05
    let lock_script_hex = "a91462d3deeddad02451dba3af4456e077da43566e0587";
    let lock_script_bytes = decode(lock_script_hex).expect("decode hex failed");
    let lock_script = Script::from_bytes(&lock_script_bytes);

    println!("Lock Script: {}", lock_script);

    println!("Script is_p2sh: {:?}", lock_script.is_p2sh());

    // let script2 = SigScript::from(script_hex);

    // println!("Script2: {:?}", script2);
}

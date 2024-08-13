# Bitcoin script test

A simple test of bitcoin script. 

## Output script

### P2PKH

ASM: OP_DUP OP_HASH160 OP_PUSHBYTES_20 `<PK_Hash>` OP_EQUALVERIFY OP_CHECKSIG

Check:
* OP_HASH160(PK) = PK_Hash
* OP_EQUALVERIFY verify signature

Ref: [rust-bitcoin/bitcoin/src/blockdata/script/borrowed.rs#L190](https://github.com/rust-bitcoin/rust-bitcoin/blob/5cca2f271d04141e1ec7d28cc07add8f2bc9b404/bitcoin/src/blockdata/script/borrowed.rs#L190)


### P2SH

ASM: OP_HASH160 OP_PUSHBYTES_20 `<Script_Hash>` OP_EQUAL

Check:
* OP_HASH160(redeem_script_bytes) = Script_Hash
* execute redeem script

Ref: [rust-bitcoin/bitcoin/src/blockdata/script/borrowed.rs#L181](https://github.com/rust-bitcoin/rust-bitcoin/blob/5cca2f271d04141e1ec7d28cc07add8f2bc9b404/bitcoin/src/blockdata/script/borrowed.rs#L181)

### P2WPKH

The signature (witness) part is separated from the input part of the transaction, thereby improving the efficiency and scalability of the transaction. The locking script only contains the public key hash, while the unlocking script (witness) contains the signature and the public key.

Witness: signature + public key

ASM: OP_0 OP_PUSHBYTES_20 `<PK_Hash>`

Check: same as P2PKH

Ref: [rust-bitcoin/bitcoin/src/blockdata/script/borrowed.rs#L285](https://github.com/rust-bitcoin/rust-bitcoin/blob/5cca2f271d04141e1ec7d28cc07add8f2bc9b404/bitcoin/src/blockdata/script/borrowed.rs#L285)

### P2WSH

Witness: redeem script

ASM: OP_0 OP_PUSHBYTES_32 `<Script_Hash>`

Check: same as P2SH

Ref: 
[rust-bitcoin/bitcoin/src/blockdata/script/borrowed.rs#L277](https://github.com/rust-bitcoin/rust-bitcoin/blob/5cca2f271d04141e1ec7d28cc07add8f2bc9b404/bitcoin/src/blockdata/script/borrowed.rs#L277)

[rust-bitcoin/bitcoin/src/blockdata/witness.rs#L468](https://github.com/rust-bitcoin/rust-bitcoin/blob/5cca2f271d04141e1ec7d28cc07add8f2bc9b404/bitcoin/src/blockdata/witness.rs#L468)

### P2TR

ASM: OP_1 OP_PUSHBYTES_32 `<Taproot_PK>`

Check: todo!()

Ref: [rust-bitcoin/bitcoin/src/blockdata/script/borrowed.rs#L293](https://github.com/rust-bitcoin/rust-bitcoin/blob/5cca2f271d04141e1ec7d28cc07add8f2bc9b404/bitcoin/src/blockdata/script/borrowed.rs#L293)

## Input script

todo!()

## Utils 

sha256 online: https://www.lzltool.com/data-sha256

ripemd160 online: http://web.chacuo.net/charsetripemd160


## Ref

### OP_HASH160

res = ripemd160(sha256(data))

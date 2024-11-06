# Ordinals Introduction

## Staoshi

### BTC 经济模型

总供应量21M, 平均每10分钟开采一个区块, 最终的比特币将在2140年左右开采完. 初始区块奖励50BTC, 每210K块($210000/365/24/6 \approx 3.995$年)奖励减半.

比特币网络每产生2016个区块($2016/24/6 = 14$天)后, 会根据之前2016个区块的计算时间以及算力进行数学难题的难度调整, 通过减少或增加难度使每个区块的计算时间维持在10分钟左右. 

### Staoshi 编号

```rust
const INITIAL_REWARD_SATOSHI: u64 = 50 * 100000000;
const HALVING_INTERVAL: u64 = 210000;

fn calculate_coinbase_satoshi_reward(block_height: u64) -> u64 {
    let halvings = block_height / HALVING_INTERVAL;
    INITIAL_REWARD_SATOSHI >> halvings
}

fn calculate_cumulative_coinbase_satoshi_range(current_block_height: u64) -> (u64, u64) {
    let mut cumulative_start: u64 = 0;
    for height in 0..current_block_height {
        let reward_satoshi = calculate_coinbase_satoshi_reward(height);
        let next_cumulative_start = cumulative_start + reward_satoshi;
        cumulative_start = next_cumulative_start;
    }
    let reward_satoshi_current = calculate_coinbase_satoshi_reward(current_block_height);
    (cumulative_start, cumulative_start + reward_satoshi_current)
}
```

### Staoshi 流转

因为每个BTC都是通过挖矿奖励产生的, 所以他们都是可溯源的. 比特币账户使用UTXO模型. 假设用户A通过挖矿获得了第100-110个聪(10个聪是一个整体存放在同一个id为adc123的UTXO中). 当用户A要支付给用户B 5个聪时, 他选择使用id为abc123作为交易的输入, 其中5个聪给到用户B, 5个聪作为找零返回给用户A. 这两份5个聪都是一个整体, 分别存放在两个id为abc456和abc789的UTXO中. 上述UTXO id和聪的数量仅作为例子展示, 在实际情况下发送的聪的数量最小限制为546个以及UTXO id也并非以此形式表达.

### 稀有度(Rare Satoshi)

作为Ordinals协议的衍生玩法, 聪的稀有度可以根据它们的挖掘顺序来定义. 这将导致一些特殊的聪具有不同的稀有度. 以下是不同聪的稀有程度: 

- `common`普通级: 除区块第一个聪外的任何聪(总供应量为2100万亿)

- `uncommon`优良级: 每个区块的第一个聪(总供应量为6929999)

- `rare`稀有级: 每个难度调整期的第一个聪(总供应量为3437)

- `epic`史诗级: 每次减半后的第一个聪(总供应量为32)

- `legendary`传奇级: 每个周期期的第一个聪(总供应量为5)

- `mythic`神话级: 创世区块的第一个聪(总供应量为1)

这种稀有聪的概念可以为比特币生态增加更多的趣味性和价值. 不同稀有度的聪可能在市场上具有不同的价值, 吸引收藏家和投资者. 


## Inscription

Ordinals与其他非比特币链上的NFT显著不同. 其中, 最主要的差异在于, Ordinals的元数据并没有存储在一个特定的位置上. 相反, 这些元数据被嵌入到交易的见证数据(witness data, witness field)中, 这就是为何我们称之为 "铭文(inscription)" 的原因, 因为这些数据被像铭文一样"刻”在比特币交易的特定部分上, 而这些数据正是附着在特定聪上的. 这一铭文过程通过隔离见证(Segregated Witness, SegWit)和"向Taproot支付”(Pay-to-Taproot, P2TR)的方式实现, 其中包含了提交(commit)和揭露(reveal)两个阶段, 能够将任何形式的内容(如文本, 图像或视频)铭刻在指定的聪上. 

### OP_RETURN

在 Bitcoin Core 客户端 0.9 版中, 通过采用 RETURN 操作符最终实现了妥协. OP_RETURN 允许开发者在交易输出上增加 80 字节的非支付数据.  与伪支付不同, RETURN 创造了一种明确的可验证不可消费型输出, 此类数据无需存储于 UTXO 集. RETURN 输出被记录在区块链上, 它们会消耗磁盘空间, 也会导致区块链规模的增加, 但它们**不存储在 UTXO 集中**, 因此也不会使得 UTXO 内存池膨胀, 更不会增加全节点昂贵的内存代价. 

虽然OP_RETURN是一个非常直接的用以存储信息至比特币区块链的手段, 它也是一个潜在的铭文方式. 但是OP_RETURN的限制使得其在处理元数据存储时面临一些挑战. 首先, OP_RETURN只能存储80字节的数据, 对于需要存储更大量数据的情况来说, 这种限制显然是无法满足的. 其次, OP_RETURN数据被存储在交易输出部分, 虽然这种数据不存储在UTXO集中, 但是它们佔用了区块链的存储空间, 导致区块链规模的增加. 最后, 使用OP_RETURN会导致交易费用的提高, 因为它需要支付更多的费用来发布这些交易. 

### Segregated Witness

SegWit是比特币的一个重要协议升级, 由比特币核心开发者 Pieter Wuille 在 2015 年提出, 最终在 2017 年的 0.16.0 版本中被正式采纳, SegWit是将某些交易签名数据(见证数据)与交易分开. 

将签名与交易相关数据分离的主要好处是减少了比特币块中数据的大小. 这样每个块具有额外的容量来存储更多的交易, 也意味著网络可以处理更多的交易, 并且发送者支付更低的手续费. 从技术上来说就是把脚本签名(scriptSig)信息从基本结构 (base block)里拿出来, 放在一个新的数据结构当中. 做验证工作的节点和矿工也会验证这个新的数据结构裡的脚本签名, 以确保交易有效.  

Segwit 升级在交易输出中引入了一个新的见证字段, 以确保隐私和性能. 虽然见证数据不是为了数据存储而设计的, 但它实际上给了我们一个存储铭文元数据等内容的机会. 

### Taproot

P2TR是比特币的一种交易输出类型, 它是在2021年进行的Taproot升级中引入的, 它使得不同的交易条件可以更加隐私地存储在区块链中. 在Ordinals的铭文中, P2TR 扮演著至关重要的角色. 铭文本质上是将特定的数据内容嵌入到比特币交易中, 而Taproot升级, 尤其是P2TR, 使得这种嵌入数据变得更加灵活和经济. 

首先, 由于Taproot脚本的存储方式, 我们可以在Taproot脚本路径支出脚本中存储铭文内容, 这些脚本在内容方面几乎没有任何限制, 同时还能获得见证数据的折扣, 使得存储铭文内容相对经济. 由于Taproot脚本的消费只能从已经存在的Taproot输出中进行, 因此, 铭文采用了两阶段的提交/揭示流程. 首先, 在提交交易中, 创建了一个承诺包含铭文内容的脚本的Taproot输出. 然后, 在揭示交易中消费由提交交易创建的输出, 从而在链上揭示了铭文内容. 

这种做法大大降低了对资源的消耗. 如果不使用P2TR, 见证信息会存储在交易的输出中. 只要这笔输出未被消费, 见证信息就会一直存储在UTXO集中. 相反, 如果使用了P2TR, 见证信息不会出现在提交阶段生成的交易中, 因此它不会被写入UTXO集. 只有当这笔UTXO被消费时, 见证信息才会在揭示阶段的交易输入中出现.  P2TR让元数据能够写入比特币区块链, 但却从未出现在UTXO集中. 由于维护/修改UTXO集需要更多的资源, 因此这种做法可以节省大量资源. 

### Inscription

铭文中最少 546 staoshi. 

Ordinals 协议利用了SegWit 放宽了对写入比特币网络内容的大小限制, 将铭文内容存储在见证数据中, 使其可以存储最大4MB的元数据.  Taproot 使得在比特币交易中存储任意见证数据变得更加容易, 允许将旧操作码(OP_FALSE, OP_IF, OP_PUSH)重新用于他所描述的"信封”为被称为"铭文”存储任意数据. 

铸造铭文的流程包含以下两个步骤: 

1. 首先, 需要在提交交易中创建一个承诺到包含铭文内容的脚本的Taproot输出. 存储的格式是Taproot, 即前一笔交易的输出是P2TR (Pay-To-Taproot), 后一笔交易的输入, 在见证的Taproot script中嵌入特定格式的内容；首先将字符串`ord` 入栈, 以消除铭文有其他用途的歧义. `OP_PUSH 1`指示下一次推送包含内容类型, 并`OP_PUSH 0`指示后续数据推送包含内容本身. 此时铭文的数据已对应到交易输出的UTXO上, 但是未被公开. 

2. 其次, 需要在揭示交易中消费提交交易创建的那个输出. 在这个阶段, 通过将那笔铭文对应的UTXO作为输入, 发起交易. 此时, 其对应的铭文内容被公开至全网. 

通过上述两个步骤, 铭文内容已与被铭刻的UTXO进行绑定. 再根据上文介绍的对于聪的定位, 铭刻是在其输入的UTXO对应的第一个聪上进行, 铭文内容包含在显示交易的输入中. 根据上文介绍的对于聪的流转、跟踪的介绍, 这个被铭刻上特殊内容的聪可以被转移、购买、出售、丢失和恢复. 需要注意的是, 不可以重複铭刻, 否则后面的铭文是无效的. 

### examples

```text
{
  "txid": "c748a10a0b4d28b9a44cb0637aad24fa60ace435951fb7b83bdf43adc30c2281",
  "size": 463,
  "version": 1,
  "locktime": 0,
  "fee": 187,
  "inputs": [
    {
      "coinbase": false,
      "txid": "06f1e1cdf5999246b2caba294dad266e7ad7d430d929398e14b0787f7fa67915",
      "output": 0,
      "sigscript": "",
      "sequence": 4294967293,
      "pkscript": "51208bcd1d0a8d2164cfbd20f005d6834fbbde2f796b352d6d1510597114bb077cf2",
      "value": 10000,
      "address": "bc1p30x36z5dy9jvl0fq7qzadq60h00z77ttx5kk69gst9c3fwc80neq42454r",
      "witness": [
        "a744a3418e2dac8f075e249bb47eff74d60fd4e3c180e465d1f953f347a52b0675abb3312174948290a1d4f50d5d2bd764b88e0cf99570acc539e736000677f4",
        "201fbe831a16a7baffc4256c8b96043ac56fea715e1457efd114313bdb694aeebaac0063036f7264010109696d6167652f706e67004cd089504e470d0a1a0a0000000d4948445200000033000000330103000000006fa26900000006504c5445ffffff00000055c2d37e000000854944415418d363a0226840a51d1a21b442130acd21d0c201a259045a044034134787028866e4e070002b687ed000d6ee23c908a20d0458d840348f029304886676604c00ababff0f31779e4d0312cdf8af8161ff01b0331a407c04edc008a6985d04c1fa79d48c24c0e60b3f6183d907b31fe61e98fb60ee85b91fcd5f30ff628403f90000b2fa1dc15c75c8110000000049454e44ae42608268",
        "c01fbe831a16a7baffc4256c8b96043ac56fea715e1457efd114313bdb694aeeba"
      ]
    }
  ],
  "outputs": [
    {
      "address": "bc1p0nv4x0r82aepguwnt039q64c48tg6cyc8895q0ylzrlusrtqxz4ss4a96k",
      "pkscript": "51207cd9533c6757721471d35be2506ab8a9d68d609839cb403c9f10ffc80d6030ab",
      "value": 9813,
      "spent": true,
      "spender": {
        "txid": "4651dc5e964879b1eb9183d467d1187dcd252504698002b01853446c460db2c5",
        "input": 0
      }
    }
  ],
  "block": {
    "height": 771290,
    "position": 1324
  },
  "deleted": false,
  "time": 1673359619,
  "rbf": false,
  "weight": 745
}
```

spend script:

```text
201fbe831a16a7baffc4256c8b96043ac56fea715e1457efd114313bdb694aeebaac0063036f7264010109696d6167652f706e67004cd089504e470d0a1a0a0000000d4948445200000033000000330103000000006fa26900000006504c5445ffffff00000055c2d37e000000854944415418d363a0226840a51d1a21b442130acd21d0c201a259045a044034134787028866e4e070002b687ed000d6ee23c908a20d0458d840348f029304886676604c00ababff0f31779e4d0312cdf8af8161ff01b0331a407c04edc008a6985d04c1fa79d48c24c0e60b3f6183d907b31fe61e98fb60ee85b91fcd5f30ff628403f90000b2fa1dc15c75c8110000000049454e44ae42608268

asm: OP_PUSHBYTES_32 1fbe831a16a7baffc4256c8b96043ac56fea715e1457efd114313bdb694aeeba OP_CHECKSIG OP_0 OP_IF OP_PUSHBYTES_3 6f7264 OP_PUSHBYTES_1 01 OP_PUSHBYTES_9 696d6167652f706e67 OP_0 OP_PUSHDATA1 89504e470d0a1a0a0000000d4948445200000033000000330103000000006fa26900000006504c5445ffffff00000055c2d37e000000854944415418d363a0226840a51d1a21b442130acd21d0c201a259045a044034134787028866e4e070002b687ed000d6ee23c908a20d0458d840348f029304886676604c00ababff0f31779e4d0312cdf8af8161ff01b0331a407c04edc008a6985d04c1fa79d48c24c0e60b3f6183d907b31fe61e98fb60ee85b91fcd5f30ff628403f90000b2fa1dc15c75c8110000000049454e44ae426082 OP_ENDIF
```

这里 0x6f7264 = b"ord", 0x696d6167652f706e67 = b"image/png"

图片数据可以在 https://www.lzltool.com/hextoimage 在线解析.

Ref: example tx: [c748a10a0b4d28b9a44cb0637aad24fa60ace435951fb7b83bdf43adc30c2281](https://www.blockchain.com/explorer/transactions/btc/c748a10a0b4d28b9a44cb0637aad24fa60ace435951fb7b83bdf43adc30c2281)

## Ref

* https://news.marsbit.co/20230703091523288965.html
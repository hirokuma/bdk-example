# bdk-example

## Abstract

BDK を使った regtet 向けの簡単なサンプル。

## Build

`cargo build` でバイナリを作っても良いし、`cargo run` に続けてコマンドを打ち込んでも良い。

## Usage

初回の実行時、ウォレットがなければ作成する。  
拡張秘密鍵 `tprv` をテキストファイルにも保存する。  
暗号化などはしていないのでテストでしか使わないこと。

```console
# ウォレットに入金するアドレス
$ cargo run newaddr
address: bcrt1pu458qa09x8302x8wh6e9z30wqmnlxzcjrzml6v4akvvurrwd76lsn0nwrt
$ cargo run addr
0: bcrt1pu458qa09x8302x8wh6e9z30wqmnlxzcjrzml6v4akvvurrwd76lsn0nwrt

# ウォレットに入金
$ bitcoin-cli sendtoaddress bcrt1pu458qa09x8302x8wh6e9z30wqmnlxzcjrzml6v4akvvurrwd76lsn0nwrt 0.001
789031174f7c85e703cd961a7494386b813efe3baaf9e7ada1ff52e9435ba199
$ bitcoin-cli getrawtransaction 789031174f7c85e703cd961a7494386b813efe3baaf9e7ada1ff52e9435ba199
020000000001014ae67d3e7bbec0521b60682404fb39cfaa000af4b211637ed334ee4323610a520000000000fdffffff02bb6a042a01000000225120970f08134a580b99fe97634eacc4d594e7e6cd7401879f842da50f7b635ff8dba086010000000000225120e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf0247304402206c1a08b1891dee5ef1f64741ed85f236f0b360c14de90b2851e8741725ae8d9b02201e7c3c8b12358a9be1c46b640d0f92ac29505c49041959e02a46a659796b9b00012103d064fc7cd163a7ee49e43dbdd35391ecd909cf29c4eaeb0210273e2d1640248c6c000000

# トランザクションのデコード
$ cargo run tx 020000000001014ae67d3e7bbec0521b60682404fb39cfaa000af4b211637ed334ee4323610a520000000000fdffffff02bb6a042a01000000225120970f08134a580b99fe97634eacc4d594e7e6cd7401879f842da50f7b635ff8dba086010000000000225120e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf0247304402206c1a08b1891dee5ef1f64741ed85f236f0b360c14de90b2851e8741725ae8d9b02201e7c3c8b12358a9be1c46b640d0f92ac29505c49041959e02a46a659796b9b00012103d064fc7cd163a7ee49e43dbdd35391ecd909cf29c4eaeb0210273e2d1640248c6c000000
Transaction {
    version: Version(

...中略...

    output: [
        TxOut {
            value: 4999899835 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 970f08134a580b99fe97634eacc4d594e7e6cd7401879f842da50f7b635ff8db),
        },
        TxOut {
            value: 100000 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf),
        },
    ],
}

# 送金先アドレス
$ bitcoin-cli getnewaddress
bcrt1q9y2ml66p3apq5lsv423emas5flp0hx9mv7acdz

# ウォレットから送金先アドレスに送金
$ cargo run spend bcrt1q9y2ml66p3apq5lsv423emas5flp0hx9mv7acdz 10000 1.0
02000000000101b9014d58a9a0b2774c659b9a3b5c745f814539a59f22eec94ca447c15cd9d51b0100000000fdffffff02e16e98000000000022512018ea7e28a90c7b9d7c59b4029a87610dadff1c17f46a635a91916dd8936db29110270000000000001600142915bfeb418f420a7e0caaa39df6144fc2fb98bb0140ece1518925ac9f40dd7398f918f90f1ea2874618fe5c4c74e237445f72be33f5dc78b9e6900310e133565b4a4c666462ba8cdf84ceb062735e12f661ffb32a246f000000
Transaction {
    version: Version(
        2,
    ),

...中略...

    output: [
        TxOut {
            value: 9989857 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 18ea7e28a90c7b9d7c59b4029a87610dadff1c17f46a635a91916dd8936db291),
        },
        TxOut {
            value: 10000 SAT,
            script_pubkey: Script(OP_0 OP_PUSHBYTES_20 2915bfeb418f420a7e0caaa39df6144fc2fb98bb),
        },
    ],
}
vsize: 142

# トランザクション展開
$ bitcoin-cli sendrawtransaction 02000000000101...中略...6f000000

# ブロック生成
$ ./generate.sh

# 送金先アドレスへの送金を確認
$ bitcoin-cli getreceivedbyaddress bcrt1q9y2ml66p3apq5lsv423emas5flp0hx9mv7acdz
0.00010000
```

### newaddr

新規アドレスを割り振る。

```console
$ cargo run newaddr
address: bcrt1pu458qa09x8302x8wh6e9z30wqmnlxzcjrzml6v4akvvurrwd76lsn0nwrt
```

### addr

今まで割り振ったアドレスを閲覧する(お釣りアドレスは表示しない)。

```console
$ cargo run addr
0: bcrt1pu458qa09x8302x8wh6e9z30wqmnlxzcjrzml6v4akvvurrwd76lsn0nwrt
```

### tx

トランザクションのデコード

```console
$ cargo run tx 020000000001014ae67d3e7bbec0521b60682404fb39cfaa000af4b211637ed334ee4323610a520000000000fdffffff02bb6a042a01000000225120970f08134a580b99fe97634eacc4d594e7e6cd7401879f842da50f7b635ff8dba086010000000000225120e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf0247304402206c1a08b1891dee5ef1f64741ed85f236f0b360c14de90b2851e8741725ae8d9b02201e7c3c8b12358a9be1c46b640d0f92ac29505c49041959e02a46a659796b9b00012103d064fc7cd163a7ee49e43dbdd35391ecd909cf29c4eaeb0210273e2d1640248c6c000000
Transaction {
    version: Version(

...中略...

    output: [
        TxOut {
            value: 4999899835 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 970f08134a580b99fe97634eacc4d594e7e6cd7401879f842da50f7b635ff8db),
        },
        TxOut {
            value: 100000 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf),
        },
    ],
}
```

### spend

ウォレットからアドレスに送金するトランザクション作成

```console
$ cargo run spend bcrt1q9y2ml66p3apq5lsv423emas5flp0hx9mv7acdz 10000 1.0
02000000000101b9014d58a9a0b2774c659b9a3b5c745f814539a59f22eec94ca447c15cd9d51b0100000000fdffffff02e16e98000000000022512018ea7e28a90c7b9d7c59b4029a87610dadff1c17f46a635a91916dd8936db29110270000000000001600142915bfeb418f420a7e0caaa39df6144fc2fb98bb0140ece1518925ac9f40dd7398f918f90f1ea2874618fe5c4c74e237445f72be33f5dc78b9e6900310e133565b4a4c666462ba8cdf84ceb062735e12f661ffb32a246f000000
Transaction {
    version: Version(
        2,
    ),

...中略...

    output: [
        TxOut {
            value: 9989857 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 18ea7e28a90c7b9d7c59b4029a87610dadff1c17f46a635a91916dd8936db291),
        },
        TxOut {
            value: 10000 SAT,
            script_pubkey: Script(OP_0 OP_PUSHBYTES_20 2915bfeb418f420a7e0caaa39df6144fc2fb98bb),
        },
    ],
}
vsize: 142
```

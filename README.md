# bdk-example

## Abstract

BDK を使った regtet 向けの簡単なサンプル。  
オフラインで動作するが、raw transaction を入力する必要がある。

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
bcrt1qetq09smm08unrs6vff8spycps24jss57qmcyt5

# ウォレットから送金先アドレスに送金
$ cargo run spend 020000000001014ae67d3e7bbec0521b60682404fb39cfaa000af4b211637ed334ee4323610a520000000000fdffffff02bb6a042a01000000225120970f08134a580b99fe97634eacc4d594e7e6cd7401879f842
da50f7b635ff8dba086010000000000225120e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf0247304402206c1a08b1891dee5ef1f64741ed85f236f0b360c14de90b2851e8741725ae8d9b02201e7c3c8b12358a9be1c46b640d0f92ac
29505c49041959e02a46a659796b9b00012103d064fc7cd163a7ee49e43dbdd35391ecd909cf29c4eaeb0210273e2d1640248c6c000000 1 bcrt1qetq09smm08unrs6vff8spycps24jss57qmcyt5 10000 1
signers len: 1
0200000000010199a15b43e952ffa1ade7f9aa3bfe3e816b3894741a96cd03e7857c4f173190780100000000ffffffff021027000000000000160014cac0f2c37b79f931c34c4a4f00930182ab28429e015f010000000000225120acf1091d5fd593184b042078cf58c79599c3a5e2a94fec2bad38e54a5c2a4d900140cb9c93708f08527f1dcf629fb3d984268e52ad53ee3e3a6aa034aa09385ac40067a13194f08998bdd0aade877e479415d96102ce145230d8872c645f7b581e6400000000
Transaction {
    version: Version(

...中略...

    ],
    output: [
        TxOut {
            value: 10000 SAT,
            script_pubkey: Script(OP_0 OP_PUSHBYTES_20 cac0f2c37b79f931c34c4a4f00930182ab28429e),
        },
        TxOut {
            value: 89857 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 acf1091d5fd593184b042078cf58c79599c3a5e2a94fec2bad38e54a5c2a4d90),
        },
    ],
}
vsize: 142

# ブロック生成
$ ./generate.sh

# 送金先アドレスへの送金を確認
$ bitcoin-cli getreceivedbyaddress bcrt1qetq09smm08unrs6vff8spycps24jss57qmcyt5
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
$ cargo run spend 020000000001014ae67d3e7bbec0521b60682404fb39cfaa000af4b211637ed334ee4323610a520000000000fdffffff02bb6a042a01000000225120970f08134a580b99fe97634eacc4d594e7e6cd7401879f842da50f7b635ff8dba086010000000000225120e5687075e531e2f518eebeb25145ee06e7f30b1218b7fd32bdb319c18dcdf6bf0247304402206c1a08b1891dee5ef1f64741ed85f236f0b360c14de90b2851e8741725ae8d9b02201e7c3c8b12358a9be1c46b640d0f92ac29505c49041959e02a46a659796b9b00012103d064fc7cd163a7ee49e43dbdd35391ecd909cf29c4eaeb0210273e2d1640248c6c000000 1 bcrt1qetq09smm08unrs6vff8spycps24jss57qmcyt5 10000 1
signers len: 1
0200000000010199a15b43e952ffa1ade7f9aa3bfe3e816b3894741a96cd03e7857c4f173190780100000000ffffffff021027000000000000160014cac0f2c37b79f931c34c4a4f00930182ab28429e015f010000000000225120acf1091d5fd593184b042078cf58c79599c3a5e2a94fec2bad38e54a5c2a4d900140cb9c93708f08527f1dcf629fb3d984268e52ad53ee3e3a6aa034aa09385ac40067a13194f08998bdd0aade877e479415d96102ce145230d8872c645f7b581e6400000000
Transaction {
    version: Version(

...中略...

    ],
    output: [
        TxOut {
            value: 10000 SAT,
            script_pubkey: Script(OP_0 OP_PUSHBYTES_20 cac0f2c37b79f931c34c4a4f00930182ab28429e),
        },
        TxOut {
            value: 89857 SAT,
            script_pubkey: Script(OP_PUSHNUM_1 OP_PUSHBYTES_32 acf1091d5fd593184b042078cf58c79599c3a5e2a94fec2bad38e54a5c2a4d90),
        },
    ],
}
vsize: 142
```

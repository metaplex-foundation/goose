# Goose

<img align="left" width="200" height="200" src="goose.png">


A simple CLI to interact with the [mpl-migration-validator](https://github.com/metaplex-foundation/mpl-migration-validator) program.

## Installation

Install from source with `cargo` as normal or use this bash script to download and install the latest binary for your MacOs or Linux system:

```bash
bash <(curl -sSf https://raw.githubusercontent.com/metaplex-foundation/goose/main/scripts/install.sh)
```

## Usage

### Configuration

By default, `goose` looks in your Solana config file for both the keypair and RPC URL to use. However, you can override either of the values found there by passing them in with the `-k` and `-r` flags.

### Commands

Open a migration state account for a given collection parent NFT to start the timed countdown:

```bash
goose init -c <collection-parent-nft-address> -m <Timed|Vote> -s <COLLECTION_SIZE>
```

There are two unlock methods: `Timed` and `Vote`. `Timed` unlocks the collection after a two weeks. `Vote` unlocks the collection after the two week period but also requires proof of a community vote.

The collection size is the number of NFTs in the collection and is not checked by the on-chain program so must be entered correctly by the user.

Cancel a migration countdown by closing the migration state account:

```bash
goose cancel -c <collection-parent-nft-address>
```

Get the migration state of an existing migration state account:

```bash
goose get-state -c <collection-parent-nft-address>
```

Get all migration state accounts on a particular cluster:

```bash
goose get-all-states
```

Update a migration state and check if it's ready to be unlocked:

Updateable values:
* collection size
* rule set pubkey
* collection update authority (USE WITH CAUTION, this should match your NFTs update authority)

```bash
goose update -c <collection-parent-nft-address> -s <COLLECTION_SIZE> -R <RULE_SET_PUBKEY> -n <NEW_AUTHORITY_PUBKEY>
```

You can only specify the specific value you want to update:

```bash
goose update -c <collection-parent-nft-address> -n <NEW_AUTHORITY_PUBKEY>
```

Or leave them all off to simply crank the timestamp check:

```bash
goose update -c <collection-parent-nft-address>
```



Enable migration for a collection:

```bash
goose start -c <collection-parent-nft-address>
```

Migrate items from a mint list, if the migration is enabled:

```bash
goose migrate -c <collection-parent-nft-address> -m <mint-list-file>
```

Check mint list for migrated and unmigrated items:

```bash
goose check -m <mint-list-file>
```

This will print out three files:

`migrated_mints.json`, `unmigrated_mints.json`, and `errors.json`

You can then run the `migrate` command to migrate the items in `unmigrated_mints.json` and repeated until all items are successfully migrated. The `errors.json` file will contain any itesm that failed the check.
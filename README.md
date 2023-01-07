# Goose

A simple CLI to interact with the [mpl-migration-validator](https://github.com/metaplex-foundation/mpl-migration-validator) program.

## Usage

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
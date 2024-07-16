 # Address Book

These are commonly used addresses among Ethereum-like chains.

## Usage

This is the primary usage for the address book. Currently the method `on()` returns `Address` but in the future as other chains are added where certain tokens do not exist then the return type will be `Option<Address>`

```rust
    use plutonium_address_book::AddressBook;

    let weth = AddressBook::Weth.on(NamedChain::Mainnet);
```

For a more certain approach

```rust
    let weth = plutonium_address_book::mainnet::WETH;
```

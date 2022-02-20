# Orion Protocol

Orion is a protocol for recurring payments on Solana, targeting a web2 tier UX for consumers. The protocol is effectively onchain subscription based payments infrastructure.

The real key feature of the protocol is that the consumer need not be present for every payment and can schedule for payments to leave their wallet without giving up the entire (or any) amount upfront.

The roles within the protocol are between that of Merchants and Consumers. Merchants are service providers that are looking for a web3 native format for hosting subscription payments and Consumers are basically the Merchant's customers or users.

Merchants control two different accounts: `MerchantAuthority` and `PaymentConfig`, while consumers only need to make a `PaymentMetadata` account to subscribe to whatever said Merchant is offering. It is more than likely that most, if not all, merchants will need some web2 presence to effectively track their users payments status but that is to be expected. Nevertheless, the repo includes some sample code for merchants so they dont have to do all the work themselves and have a
ready-to-go solution out of the box.

Both Merchants and Consumers can create as many of these accounts as they like as the needs of the Merchant's service require. The next sections will outline a high level workflow for both Merchants and Consumers from the perspective of the protocol.

## Workflow for merchants

Ok. So you're an entrepeneur with a vision that requires your users have the ability to pay directly from their Solana wallet on a periodic basis, but without any manual intervention. Perfect! You're in the right place at the right time, buddy. Here's how it's going to work for you: it all starts onchain.

First, you'll need to create a `MerchantAuthority` account. Here's what that looks like:

```rust
pub struct MerchantAuthority {
    pub init_authority: Pubkey, // The first authority - necessary for account derivation
    pub current_authority: Pubkey, // The current authority
    pub pending_authority: Pubkey, // The pending authority (Pubkey::default() to begin with)
    pub bump: u8, // Seed bump - necessary for account derivation
}
```

The seeds for a `MerchantAuthority` are:

```rust
[b"merchant_authority", merchant_authority.key().as_ref(), init_authority.key().as_ref(), bump],
```

An account can be initialized using the `initialize_merchant_authority()` instruction in the `recurring` smart contract.

Great! Now that you have a `MerchantAuthority` account, you can move onto the fun part: defining the structure of how'd you like payments for your service.

To do this, you'll need to make a `PaymentConfig` account. Here's what that looks like:

## Workflow for consumers

## Explaining the accounts

### `MerchantAuthority`

### `PaymentConfig`

### `PaymentMetadata`

## Calculating important values from account data

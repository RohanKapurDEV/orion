# Orion Protocol

Orion is a protocol for recurring payments on Solana, targeting a web2 tier UX for consumers. The protocol is effectively onchain subscription based payments infrastructure.

The real key feature of the protocol is that the consumer need not be present for every payment and can schedule for payments to leave their wallet without giving up the entire (or any) amount upfront.

The roles within the protocol are between that of Merchants and Consumers. Merchants are service providers that are looking for a web3 native format for hosting subscription payments and Consumers are basically the Merchant's customers or users.

Merchants control two different accounts: `MerchantAuthority` and `PaymentConfig`, while consumers only need to make a `PaymentMetadata` account to subscribe to whatever said Merchant is offering. It is more than likely that most, if not all, merchants will need some web2 presence to effectively track their users payments status but that is to be expected. Nevertheless, the repo will include some sample code for merchants so they dont have to do all the work themselves and have a
ready-to-go solution out of the box.

Both Merchants and Consumers can create as many of these accounts as they like as the needs of the Merchant's service require. The next sections will outline a high level workflow for both Merchants and Consumers from the perspective of the protocol.

## Docs coming soon

Feel free to examine the source code in the meantime :)

# Learn Casper Caller Stack
Elements are pushed to the caller stack in order. \

# Examples

## **Scenario 1**: 
Account calls Contract directly: \
Len( Caller Stack ) = 2 \
Caller Stack [0] = account_hash of caller
inverse of caller stack [-1] = contract_hash of contract \
=> if we want to store an element from the caller stack, we will need an enumerator ( Address ) \

## **Scenario 2**: 
Account calls Contract A, Contract A calls Contract B. \
Caller Stack in Contract B execution: \
Len( Caller Stack ) = 3 \
Caller Stack [0] = account_hash of caller \ 
Caller Stack [1] = contract_hash of Contract A \
Caller Stack [2] = contract_hash of Contract B
** More info in detail.rs **

## Reason for Address type to exist

We can't know whether a caller is a contract or account in advance, \
therefore the Address enumerator exists and represents an Option<contract_hash, account_hash>

### Scripts

## Script as a Language
Scripts is the smart contract language used to lock and unlock funds on Bitcoin. From the Transaction module, we can see that each transaction contains a scriptSig (in the Input) and a ScriptPubKey (in the Output).

These are created using the Script language. This module details how this works.

## Script Commands

The Script commands operate on a Stack.

There are 2 types of commands that a script processes:

* Elements: are technically data.
* Operations: operate on the data. This will take data from the stack and push zero or more data back.
### Programming Transactions

### Summary of Transactions

Transactions are essentially value transfers between entities (in the case of Bitcoin, the entities are smart contracts).

A Bitcoin transaction is made up of 4 main components:

<ol>
    <li><strong>Version</strong>: Indicates the version, this is useful to specify the features of the transaction. Could be 1 or 2</li>
    <li><strong>Inputs</strong>: A list of transaction inputs. The inputs are UTXOs from a previous transaction</li>
    <li>
    <strong>Outputs</strong>: UTXOs emerging from this tramsaction. It's usually gotten by splitting the value(s) from the input.</li>
    <li>
        <strong>Locktime</strong>: Specifies a time when the transaction can start being valid
    </li>
</ol>


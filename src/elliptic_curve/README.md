### Elliptic Curves

Elliptic curves are also very useful in the process of Elliptic Curve Cryptography .

Elliptic curves have the form

``
  y<sup>2</sup> = x<sup>3</sup> + ax + b
``

<em>a</em> and <em>b</em> are constants and choosing any point on the curve, it should solve the equation.

<em>y<sup>2</sup></em> term on the LHS makes sure that the curve is symmetric over the x-axis, and also less steep than a corresponding cubic curve.

Some example elliptic curves are 
  1. secp256k1 (Bitcoin)
  2. ECDSA (Ethereum)
  3. EdDSA (Cardano)
  4. Sr25519 (Polkadot)

In these blockchains, elliptic curves are applied in the cryptography used to generate Keys and Addresses.

EC curves are also useful because we can add Points, called _Points addition_. This has all the properties of the usual addition, such as

- Identity: meaning given Point A, there exists a Point I where `I + A = A`
- Invertibility: given a Point A, there exists a Point (-A) such that `A + (-A) = I`, where I is the identity point.
- Commutativity: given 2 Points A and B, `A + B = B + A`
- Associativity: given 3 Points A, B, C `A + (B + C) = (A + B) + C`.
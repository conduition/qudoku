# qudoku

A nested threshold system to complement [Shamir's Secret Sharing (SSS)][SSS] groups with an arbitrary amount of additional secrets, at no additional storage burden on the part of the shareholders.

> [!CAUTION]
>
> This library is **highly experimental** cryptographic software, designed for a specialized use-case.
>
> Do not use this code in production.

Consider a [SSS] group with threshold $t$ and $n$ shares in some shared secret $k$.

The dealer wishes to generate an additional secret $c$ which can only be reconstructed if _at least_ some subset $t' \le t$ of the shareholders from the group give their explicit consent. Giving such consent cannot compromise the shareholders' shares in the primary group secret $k$. Shareholders should not need to store any additional data besides their original share.

**Example**: A threshold recovery system stores an encrypted blob of data on a server, while the decryption key is distributed among the members of a [SSS] group. Contact information for all shareholders is encrypted on the server under a key $c$, such that if one shareholder initiates the recovery process, all shareholders can be notified. However, if nobody in the group initiates recovery, the contact information remains secret.

This is sort of like nesting one secret sharing scheme inside another, with a potentially different (smaller) threshold.

## Simple Solution

If shareholder storage and synchronization is not a problem, the simplest solution is to create a second [SSS] group which distributes the secret $c$ in addition to the original SSS secret $k$. Shareholders would then store two shares: one in $k$ and one in $c$.

However, there are situations where storing additional shares is not practical.

- What if we need distinct and unrelated secrets for every one of 100,000,000 distinct situations? That would necessecitate storing 100,000,000 + 1 shares with this approach.
- What if the share storage medium demands an extremely dense encoding, such as [BIP39]?

## Subtle Solution

Instead of storing additional secrets with each shareholder, we will define some fixed constant parameters which shareholders can use in combination with their shares in order to reconstruct the secret $c$. The reconstruction of $c$ can be executed by a third party who learns no new information about the primary group secret $k$. This approach can be applied to existing [SSS] groups retroactively, without any new information stored by shareholders.


# Prerequisites

First, let's review our [SSS] group and its parameters.

| Symbol | Meaning |
|:------:|:-------:|
| $t$ | [SSS] group security threshold. |
| $n$ | [SSS] group size. |
| $k$ | Primary secret shared through [SSS]. |
| $f(x)$ | [SSS] secret sharing polynomial. |
| $c$ | Secondary secret shared using `qudoku`. |

The secret sharing polynomial $f(x)$ is a degree $t-1$ polynomial which encodes the secret $k$ at some fixed output. For example, $k = f(0)$. This means recovering $f(x)$ implies recovering $k$.

Since $f(x)$ has degree $t-1$, it cannot be recovered without knowing $t$ or more distinct evaluations of $f(x)$. Each of these evaluations, $(i, f(i))$, are [SSS] shares.

## Elliptic Curves

[SSS] can be extended with [Elliptic Curve Cryptography](https://conduition.io/cryptography/ecc-resources/) to add support for more complex use cases, such as [Verifiable Secret Sharing (VSS)][VSS]. We will make use of the homomorphic properties of Elliptic Curve groups to allow a single [SSS] group to derive multiple distinct secrets.

For this library, we'll be using the secp256k1 curve. Noteworthy parameters of the curve:

| Symbol | Meaning |
|:------:|:-------:|
| $q$ | Prime order of the elliptic curve. |
| $G$ | Elliptic curve base-point. Multiplying $G$ by a scalar integer in the range $[1, q)$ produces a point on the curve. |

Elliptic curve points can be added and subtracted, but not multiplied or divided by one-another. This means that scalar-point multiplication is a one-way operation which cannot be reversed (without a quantum computer).

# Algorithm

The [SSS] protocol designer must first fix a chosen constant point $Q$ (from which **qu**doku derives its name).

This point must have an important property: It must have an unknown discrete logarithm with respect to the curve base point $G$. This means that if $xG = Q$, then $x$ must exist but should not be known by anybody, including the dealer or protocol designer.

The best way to prove such a point has this property is to derive it using a nothing-up-my-sleeve approach, which demonstrates the point was selected in a way nobody could have chosen it intentionally while knowing its discrete log.

One such example is to generate a point by hashing some honest-looking input data, and then interpreting that hash as the X coordinate of an elliptic curve point. The pseudo-random and unpredictable nature of a secure hash function ensures that the X coordinate is effectively random. This prevents the generator from choosing a specific point which a known discrete log. The hash can be repeated to verify the point is indeed random and honestly chosen.

Not all hash outputs are valid X coordinates on the secp256k1 curve though, so you may need to increment the output a couple of times to find a valid curve point. When you find a valid X coordinate, you can select either the odd or even parity Y-coordinate which comes along with it - Either are equally honest and usable.

TODO

[SSS]: https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing
[BIP39]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
[VSS]: https://en.wikipedia.org/wiki/Verifiable_secret_sharing

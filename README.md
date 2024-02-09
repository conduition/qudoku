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
| $j$ | The input at which $f$ outputs $k$, i.e. $f(s) = k$ |
| $c$ | Secondary secret shared using `qudoku`. |

The secret sharing polynomial $f(x)$ is a degree $t-1$ polynomial which encodes the secret $k$ at some fixed output. For example, $k = f(0)$. This means recovering $f(x)$ implies recovering $k$.

Since $f(x)$ has degree $t-1$, it cannot be recovered without knowing $t$ or more distinct evaluations of $f(x)$. Each of these evaluations, $(i, f(i))$, are [SSS] shares.

## Elliptic Curves

[SSS] can be extended with [Elliptic Curve Cryptography](https://conduition.io/cryptography/ecc-resources/) to add support for more complex use cases, such as [Verifiable Secret Sharing (VSS)][VSS]. We will make use of the homomorphic properties of Elliptic Curve groups to allow a single [SSS] group to derive multiple distinct secrets.

For this library, we'll be using the secp256k1 curve. Noteworthy parameters of the curve:

| Symbol | Meaning |
|:------:|:-------:|
| $q$ | Prime order of the elliptic curve. |
| $G$ | Elliptic curve base-point. Multiplying $G$ by a scalar integer in the range $[1, q)$ produces another point on the curve. |

Elliptic curve points can be added and subtracted, but not multiplied or divided by one-another. This means that scalar-point multiplication is a one-way operation which cannot be reversed (without a quantum computer).

# Algorithm

The [SSS] protocol designer must first fix a chosen constant point $Q$ (from which **qu**doku derives its name).

## Choosing $Q$

This point demands an important property: It must have an unknown discrete logarithm with respect to the curve base point $G$. This means that if $xG = Q$, then $x$ must exist but should not be known by anybody, including the dealer or protocol designer. Someone who knows $x$ would be able to break the security properties of qudoku. <!-- TODO elaborate -->

The best way to prove such a point has this property is to derive it using a nothing-up-my-sleeve approach, which demonstrates the point was selected in a way nobody could have chosen it intentionally while knowing its discrete log.

One such example is to generate a point by hashing some honest-looking input data, and then interpreting that hash as the X coordinate of an elliptic curve point. The pseudo-random and unpredictable nature of a secure hash function ensures that the X coordinate is effectively random. This prevents the generator from choosing a specific point which has a known discrete log. An observer can be given the input data, and the hash operation can be repeated to verify the point is indeed random and honestly chosen.

Not all hash outputs are valid X coordinates on the secp256k1 curve though, so you may need to increment the output a couple of times to find a valid curve point.

Once you find a valid X coordinate, you can select either the odd or even parity Y-coordinate which comes along with it - Either are equally honest and usable.

## Computing $c$

$c$ is the symbol we give to the independent secret which the [SSS] group wishes to make shareable.

Let $j$ be the index at which the primary secret $k$ is computed; i.e. $f(j) = k$. In most cases of [SSS], $j = 0$, so that the secret is conviently available as the constant term of $f(x)$.

Let $Z(x) := f(x) \cdot Q$

Then $c$ is defined as a hash of $Z(j)$:

$$
\begin{align}
c\ :=&\ H(Z(j)) \\
    =&\ H(f(j) \cdot Q) \\
    =&\ H(kQ) \\
\end{align}
$$

$c$ can thus be derived by the dealer (who knows $f(x)$ and $k$), or by a group of $t$ or more shareholders (who can interpolate $f(x)$ and $k$). However - interestingly for our purposes - it is also possible to learn $c$ _without_ learning anything about $f(x)$ or $k$.

Suppose you are given $t$ or more evaluations of $Z(x)$. This would allow you to fully interpolate $Z(x)$ and thus compute $c = H(Z(j))$. But because elliptic curve point-scalar multiplication is a one-way homomorphic operation, it's completely impractical to use your knowledge of $Z(j)$ to compute $f(x)$ or $k$. This property holds for the same reasons that Feldman's [Verifiable Secret Sharing (VSS) scheme][VSS] is secure.

## Recovering $Z(x)$

Each secret sharing group member possesses an evaluation of $Z(x)$. Each share $(i, f(i))$ can be converted into a share of $Z(x)$ by simply multiplying the evaluation with the constant point $Q$, to yield the share $(i, f(i) \cdot Q) = (i, Z(i))$.

The shareholder doesn't need to store any extra information alongside their share; the fixed point $Q$ can be hardcoded into software for shareholders, or selected ex-post-facto by shareholders. The ability to compute $Z(i)$ is an implied capability of any shareholder who possesses a share at index $i$.

To distribute the secret $c$ then, all the shareholders need to do is compute a set of $t$ or more shares of $Z(i)$, and then send those shares to the intended recipient.

## Easing the Threshold

What if the group wishes to ease the minimum security threshold needed to learn $c$?

Currently the [SSS] group needs to send $t$ or more shares of $Z(x)$ to the recipient, because $Z(x)$ shares the same polynomial degree ($t-1$) as its factor function $f(x)$ from which it was derived.

To reduce this threshold from $t$ to any $t' \le t$, the group or dealer must pre-share a set of $t - t'$ evaluations of $Z(x)$ with the intended recipient. This set of pre-shared evaluations allows the recipient to interpolate $Z(x)$ using only an additional $t'$ shares given later by the shareholders.

These pre-shared evaluations must each use an input $i$ which is distinct from those of the shareholders' shares. Reusing input indexes would mean some shareholders cannot have a meaningful inpact in assisting the recipient with interpolating $Z(x)$, because their shares of $Z(x)$ may already be known to the recipient.

The pre-shared shares can be computed by, for example:
- reserving specific indexes for the pre-shared shares, e.g. $\\{q-1, q-2, ... q-t'\\}$, which the existing shareholders are guaranteed not to be using.
- sampling random indexes from $[1, q)$. Because $q$ is very large, so collision with a shareholder is very unlikely.

If the [SSS] dealer is available, they can compute these values directly, because they know $Z(x)$.

If not, the shareholders may need to [use multi-party computation to issue new shares of $Z(x)$ directly to the recipient](https://conduition.io/cryptography/shamir/).

Following our earlier example with the encrypted recovery server, the [SSS] group would fix $Q$ in advance, and pre-share $t-1$ shares of $Z(x) = f(x) \cdot Q$ with the recovery server. The contact information would be encrypted with $c = H(Z(s))$. When any of the shareholders with share $(i, f(i))$ initiates the recovery procedure, they provide $(i, Z(i))$ to the server, which can then interpolate $Z(x)$ and compute $c = H(Z(s))$, allowing it to decrypt the contact information.

## Scaling to Many Secrets

If shareholders wish to pre-arrange a large collection of independent secrets, they can pre-arrange a set of $m$ fixed constant points $\\{Q_1, Q_2, ... Q_m\\}$, all of which must have _unknown discrete logarithms_ relative to one-another, effectively making them [co-prime](https://en.wikipedia.org/wiki/Coprime_integers).

<sub>Technically, no two points on the secp256k1 curve can be co-prime, as common factors always _exist,_ but as long as their common factors are not _computable_ that's good enough for security to hold.</sub>

For each point $Q_i$, the shareholders define $Z_i(x) := f(x) \cdot Q_i$, and compute independent the secret $c_i := H(Z_i(s))$. Pre-sharing can be conducted on a per-secret basis, as-needed.

> [!WARNING]
>
> Why must all the $Q$ points be effectively co-prime?
>
> Let's assume there are two points $Q_1$ and $Q_2$, and that we know $p$ such that $p Q_1 = Q_2$. Then if we can interpolate $Z_1(x)$, we can also interpolate $Z_2(x)$ by computing $Z_1(x) \cdot p$.
>
> $$
> \begin{align}
> Z_2(x) &= f(x) \cdot Q_2 \\
> &= f(x) \cdot Q_1 \cdot p \\
> &= Z_1(x) \cdot p \\
> \end{align}
> $$

Assuming all $Q$ points are co-prime, then nobody can multiplicatively relate shares of any $Z_a(x)$ polynomial with another polynomial $Z_b(x)$, and so all derivative secrets $\\{c_1, c_2, ... c_m\\}$ are completely independent.


[SSS]: https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing
[BIP39]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
[VSS]: https://en.wikipedia.org/wiki/Verifiable_secret_sharing

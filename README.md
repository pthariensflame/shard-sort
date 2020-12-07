
# Shard sort

*Shard sort* is a general-purpose non-recursive incremental
out-of-place comparison-sorting algorithm that exploits runs of sorted
data.  In the best case it (completely) runs in linear time and
constant space; in the worst case it (completely) runs in quadratic
time and linear space.

Let's break that down:

- **General-purpose…comparison-sorting algorithm**: shard sort can be
  used for any kind of comparable data, needing only a way to compare
  any two elements
- **Non-recursive**: shard sort doesn't use recursion in its most
  straightforward pseudocode (although some implementations might use
  recursion incidentally)
- **Incremental**: shard sort makes a single pass over the input in
  guaranteed linear time and at most linear space, then exposes an
  iteration-like interface to incrementally compute the elements in
  sorted order; each such computation uses at most linear time and
  guaranteed constant space
- **out-of-place**: shard sort does not modify the data it sorts;
  instead it produces the sorted elements as an incrementally driven
  sequence, copying them in a configurable fashion from the original
  data
- **exploits runs**: shard sort behaves optimally (linear-time
  constant-space initial setup and constant-time constant-space
  incremental continues) when the data its sorting is already either
  sorted or reverse-sorted; more generally it behaves better than
  worst-case when the data contains many sorted and/or reverse-sorted
  “runs”, and when those runs are longer and/or fewer in number

This repository contains implementations of shard sort in a number of
programming languages, each in their own folder.  A basic effort has
been made to provide reasonably standard building and packaging of the
code for each language; where applicable, publication to community
package repositories has been done (or else is planned).

All of the code is in the public domain, as specified by [the
Unlicense](https://unlicense.org), which can be found next to this
README in the file `UNLICENSE`.

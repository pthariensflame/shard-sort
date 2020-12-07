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
  iteration-like “pull” interface to incrementally compute the
  elements in sorted order; each such computation uses at most linear
  time and guaranteed constant space
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

A pseudocode explanation of the algorithm, with running commentary, is
in [`Pseudocode.md`](./Pseudocode.md).

Additionally, this repository contains implementations of shard sort in a number of
programming languages, each in their own folder.  A basic effort has
been made to provide reasonably standard building and packaging of the
code for each language; where applicable, publication to community
package repositories has been done (or else is planned).  Currently,
the following languages are planned (if unchecked) or have
implementations in them (if checked):

- [ ] Ada
- [ ] Agda
- [ ] C
- [ ] C++
- [ ] COBOL
- [ ] Chapel
- [ ] Clojure
- [ ] Common Lisp
- [ ] Coq
- [ ] C♯
- [ ] D
- [ ] Dart
- [ ] Elixir
- [ ] Elm
- [ ] Emacs Lisp
- [ ] Erlang
- [ ] Fantom
- [ ] Fortran
- [ ] Futhark
- [ ] F★
- [ ] F♯
- [ ] Gleam
- [ ] Gluon
- [ ] Go
- [ ] Groovy
- [ ] [Haskell](./Haskell)
- [ ] Haxe
- [ ] ISPC
- [ ] Idris
- [ ] Java
- [ ] JavaScript
- [ ] Julia
- [ ] Kotlin
- [ ] LaTeX3
- [ ] Lean
- [ ] Lua
- [ ] Nim
- [ ] Objective-C
- [ ] Octave
- [ ] O’Caml
- [ ] Pascal
- [ ] Perl 5
- [ ] Pony
- [ ] Prolog
- [ ] Purescript
- [ ] Python 3
- [ ] R
- [ ] Raku
- [ ] Ruby
- [ ] [Rust](./Rust)
- [ ] Scala
- [ ] Scheme
- [ ] Smalltalk
- [ ] Swift
- [ ] Tcl
- [ ] TypeScript
- [ ] Typed Racket
- [ ] Visual Basic .NET
- [ ] Zig
- [ ] λProlog

Contributions of implementations for these language or for any other
languages, or of improvements to the existing implementations or any
documentation, are all welcome!

All of the code is in the public domain, as specified by [the
Unlicense](https://unlicense.org), which can be found next to this
README in the file [`UNLICENSE`](./UNLICENSE).  All contributors are expected to
agree to this too.

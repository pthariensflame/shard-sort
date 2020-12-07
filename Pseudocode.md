# Pseudocode for shard sort

The idea behind shard sort depends on the notion of a **shard**: a
(weakly) sorted or (strictly) reverse-sorted contiguous run of
elements in the original data, represented by a pair of an index where
the run “starts” and an offset to where it “ends”, inclusively.  Note
that if the run is reverse-sorted, the index will be the *last*
element, and the offset will be negative.  In the degenerate case of a
run of a single element, the index will be that of that element and
the offset will be 0.

The pseudocode is also written in terms of a **shard set**, which can
be any set or sequence data structure that can be grown once in order,
then iterated in some (unimportant) order and have elements
individually removed; the elements of the shard set are disjoint
shards, and the shard set will be built up from empty during the
initial scan and then reduced back down to empty over the course of
the incremental pulls.

## Core helper operations on shards and shard sets

***[TODO]***

## Initial scan

***[TODO]***

## Incremental pull

***[TODO]***

#![forbid(unsafe_code)]

use std::{convert::TryInto, marker::PhantomData};

pub fn shard_sort<SrcElem, Src, Lt, DestElem, ExtractDest>(
    src: Src,
    mut lt: Lt,
    extract_dest: ExtractDest,
) -> Sorted<SrcElem, Src, Lt, ExtractDest>
where
    Src: AsRef<[SrcElem]>,
    Lt: FnMut(&SrcElem, &SrcElem) -> bool,
    ExtractDest: FnMut(&SrcElem) -> DestElem,
{
    let mut shards = ShardSet::default();

    for wdw in src.as_ref().windows(2) {
        let &[ref a, ref b]: &[SrcElem; 2] = wdw.try_into().unwrap();
        if lt(b, a) {
            // reverse case
        } else {
            // forward case
        }
    }

    Sorted {
        src,
        lt,
        extract_dest,
        shards,
        phantom_src_elems: PhantomData,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
struct Shard {
    index: usize,
    offset: isize,
}

impl Shard {
    fn is_reverse(&self) -> bool {
        self.offset < 0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct ShardSet(Vec<Shard>);

impl Default for ShardSet {
    fn default() -> ShardSet {
        ShardSet(Vec::with_capacity(1))
    }
}

impl ShardSet {
    fn add(&mut self, shard: Shard) {
        self.0.push(shard)
    }

    fn advance_at(&mut self, shard: Shard) {
        todo!()
    }

    fn argmin_in<SrcElem, Src, Lt>(&self, src: &Src, lt: Lt)
    where
        Src: AsRef<[SrcElem]>,
    {
        todo!()
    }
}

#[must_use]
#[derive(Clone, Debug)]
pub struct Sorted<SrcElem, Src, Lt, ExtractDest>
where
    Src: ?Sized,
{
    phantom_src_elems: PhantomData<[SrcElem]>,
    lt: Lt,
    extract_dest: ExtractDest,
    shards: ShardSet,
    pub src: Src,
}

impl<SrcElem, Src, Lt, DestElem, ExtractDest> Sorted<SrcElem, Src, Lt, ExtractDest>
where
    Src: ?Sized + AsRef<[SrcElem]>,
    Lt: FnMut(&SrcElem, &SrcElem) -> bool,
    ExtractDest: FnMut(&SrcElem) -> DestElem,
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

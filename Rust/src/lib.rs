#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! TODO

pub use interface::*;

mod implementation {
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

        for (i, wdw) in src.as_ref().windows(2).enumerate() {
            let &[ref a, ref b]: &[_; 2] = wdw.try_into().unwrap();
            shards.extend_to(i + 1, a, b, &mut lt);
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
        const fn is_reverse(&self) -> bool {
            self.offset < 0
        }

        const fn is_singleton(&self) -> bool {
            self.offset == 0
        }

        const fn is_forward(&self) -> bool {
            self.offset > 0
        }

        const fn advance(&mut self) -> bool {
            if self.is_singleton() {
                // done case
                false
            } else {
                if self.is_reverse() {
                    // reverse case
                    self.index -= 1;
                    self.offset += 1;
                } else {
                    // forward case
                    self.index += 1;
                    self.offset -= 1;
                }
                true
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    struct ShardSet(Vec<Shard>);

    impl Default for ShardSet {
        fn default() -> ShardSet {
            ShardSet(vec![Shard::default()])
        }
    }

    impl ShardSet {
        fn add(&mut self, shard: Shard) {
            self.0.push(shard)
        }

        fn extend_to<SrcElem, Lt>(
            &mut self,
            new_index: usize,
            prev_elem: &SrcElem,
            next_elem: &SrcElem,
            lt: &mut Lt,
        ) where
            Lt: FnMut(&SrcElem, &SrcElem) -> bool,
        {
            todo!()
        }

        fn argmin_in<'src, SrcElem, Src, Lt>(
            &self,
            src: &'src Src,
            lt: Lt,
        ) -> (usize, &'src SrcElem)
        where
            Src: AsRef<[SrcElem]> + 'src,
            Lt: FnMut(&SrcElem, &SrcElem) -> bool,
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
}

mod interface {
    /// TODO
    pub use super::implementation::shard_sort as shard_sort_with_to;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

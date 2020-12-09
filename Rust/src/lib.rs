#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::type_complexity)]

//! TODO

pub use interface::*;

mod implementation {
    use std::{convert::TryInto, marker::PhantomData};

    pub fn shard_sort_impl<SrcElem, Src, Lt, DestElem, ExtractDest>(
        src: Src,
        mut lt: Lt,
        extract_dest: ExtractDest,
    ) -> Sorted<SrcElem, Src, Lt, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Lt: FnMut(&SrcElem, &SrcElem) -> bool,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        let mut shards = ShardSetBuilder::default();

        // compute shard set
        for (i, wdw) in src.as_ref().windows(2).enumerate() {
            let &[ref a, ref b]: &[_; 2] = wdw.try_into().unwrap();
            shards.extend_to(i + 1, a, b, &mut lt);
        }

        Sorted {
            src,
            lt,
            extract_dest,
            shards: shards.into(),
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

        fn advance(&mut self) -> bool {
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
    // needs to be a collection that handles pushing to one end and mutably peeking at that same
    // end efficiently
    struct ShardSetBuilder(Vec<Shard>);

    impl Default for ShardSetBuilder {
        fn default() -> ShardSetBuilder {
            ShardSetBuilder(vec![Shard::default()])
        }
    }

    impl ShardSetBuilder {
        fn extend_to<SrcElem, Lt>(
            &mut self,
            new_index: usize,
            prev_elem: &SrcElem,
            next_elem: &SrcElem,
            lt: &mut Lt,
        ) where
            Lt: FnMut(&SrcElem, &SrcElem) -> bool,
        {
            let curr_shard = self.0.last_mut().unwrap();
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    // needs to be a collection that handles random removal and mutable in-order traversal
    // efficiently, and that stores its data in either insertion order
    pub struct ShardSet(Box<[Option<Shard>]>);

    impl From<ShardSetBuilder> for ShardSet {
        fn from(ShardSetBuilder(shards): ShardSetBuilder) -> ShardSet {
            ShardSet(shards.into_iter().map(Some).collect())
        }
    }

    impl ShardSet {}

    #[must_use]
    #[derive(Clone, Debug)]
    pub struct Sorted<SrcElem, Src, Lt, ExtractDest>
    where
        Src: ?Sized,
    {
        phantom_src_elems: PhantomData<[SrcElem]>,
        pub lt: Lt,
        pub extract_dest: ExtractDest,
        pub shards: ShardSet,
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
    use super::implementation::*;
    use std::cmp::Ordering;

    /// TODO
    #[must_use]
    #[derive(Clone, Debug)]
    #[repr(transparent)]
    pub struct ShardSorted<SrcElem, Src: ?Sized, Lt, ExtractDest>(
        Sorted<SrcElem, Src, Lt, ExtractDest>,
    );

    /// TODO
    pub fn shard_sort_by_lt_to<SrcElem, Src, Lt, DestElem, ExtractDest>(
        src: Src,
        lt: Lt,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, Lt, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Lt: FnMut(&SrcElem, &SrcElem) -> bool,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<SrcElem, Src, Lt, DestElem, ExtractDest>(
            src,
            lt,
            extract_dest,
        ))
    }

    /// TODO
    pub fn shard_sort_by_gt_to<SrcElem, Src, Gt, DestElem, ExtractDest>(
        src: Src,
        mut gt: Gt,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Gt: FnMut(&SrcElem, &SrcElem) -> bool,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<SrcElem, Src, _, DestElem, ExtractDest>(
            src,
            move |x, y| gt(y, x),
            extract_dest,
        ))
    }

    /// TODO
    pub fn shard_sort_by_le_to<SrcElem, Src, Le, DestElem, ExtractDest>(
        src: Src,
        mut le: Le,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Le: FnMut(&SrcElem, &SrcElem) -> bool,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<SrcElem, Src, _, DestElem, ExtractDest>(
            src,
            move |x, y| !le(y, x),
            extract_dest,
        ))
    }

    /// TODO
    pub fn shard_sort_by_ge_to<SrcElem, Src, Ge, DestElem, ExtractDest>(
        src: Src,
        mut ge: Ge,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Ge: FnMut(&SrcElem, &SrcElem) -> bool,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<SrcElem, Src, _, DestElem, ExtractDest>(
            src,
            move |x, y| !ge(x, y),
            extract_dest,
        ))
    }

    /// TODO
    pub fn shard_sort_by_cmp_to<SrcElem, Src, Cmp, DestElem, ExtractDest>(
        src: Src,
        mut cmp: Cmp,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Cmp: FnMut(&SrcElem, &SrcElem) -> Ordering,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<SrcElem, Src, _, DestElem, ExtractDest>(
            src,
            move |x, y| cmp(x, y) == Ordering::Less,
            extract_dest,
        ))
    }

    /// TODO
    pub fn shard_sort_by_key_to<SrcElem, Src, Key, ExtractKey, DestElem, ExtractDest>(
        src: Src,
        mut extract_key: ExtractKey,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, ExtractDest>
    where
        Src: AsRef<[SrcElem]>,
        Key: PartialOrd,
        ExtractKey: FnMut(&SrcElem) -> Key,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<SrcElem, Src, _, DestElem, ExtractDest>(
            src,
            move |x, y| extract_key(x) < extract_key(y),
            extract_dest,
        ))
    }

    /// TODO
    pub fn shard_sort_to<SrcElem, Src, DestElem, ExtractDest>(
        src: Src,
        extract_dest: ExtractDest,
    ) -> ShardSorted<SrcElem, Src, fn(&SrcElem, &SrcElem) -> bool, ExtractDest>
    where
        SrcElem: PartialOrd,
        Src: AsRef<[SrcElem]>,
        ExtractDest: FnMut(&SrcElem) -> DestElem,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            fn(&SrcElem, &SrcElem) -> bool,
            DestElem,
            ExtractDest,
        >(src, SrcElem::lt, extract_dest))
    }

    /// TODO
    pub fn shard_sort_by_lt<SrcElem, Src, Lt>(
        src: Src,
        lt: Lt,
    ) -> ShardSorted<SrcElem, Src, Lt, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone,
        Src: AsRef<[SrcElem]>,
        Lt: FnMut(&SrcElem, &SrcElem) -> bool,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            Lt,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(src, lt, SrcElem::clone))
    }

    /// TODO
    pub fn shard_sort_by_gt<SrcElem, Src, Gt>(
        src: Src,
        mut gt: Gt,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone,
        Src: AsRef<[SrcElem]>,
        Gt: FnMut(&SrcElem, &SrcElem) -> bool,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            _,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(src, move |x, y| gt(y, x), SrcElem::clone))
    }

    /// TODO
    pub fn shard_sort_by_le<SrcElem, Src, Le>(
        src: Src,
        mut le: Le,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone,
        Src: AsRef<[SrcElem]>,
        Le: FnMut(&SrcElem, &SrcElem) -> bool,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            _,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(src, move |x, y| !le(y, x), SrcElem::clone))
    }

    /// TODO
    pub fn shard_sort_by_ge<SrcElem, Src, Ge>(
        src: Src,
        mut ge: Ge,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone,
        Src: AsRef<[SrcElem]>,
        Ge: FnMut(&SrcElem, &SrcElem) -> bool,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            _,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(src, move |x, y| !ge(x, y), SrcElem::clone))
    }

    /// TODO
    pub fn shard_sort_by_cmp<SrcElem, Src, Cmp>(
        src: Src,
        mut cmp: Cmp,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone,
        Src: AsRef<[SrcElem]>,
        Cmp: FnMut(&SrcElem, &SrcElem) -> Ordering,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            _,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(
            src,
            move |x, y| cmp(x, y) == Ordering::Less,
            SrcElem::clone,
        ))
    }

    /// TODO
    pub fn shard_sort_by_key<SrcElem, Src, Key, ExtractKey>(
        src: Src,
        mut extract_key: ExtractKey,
    ) -> ShardSorted<SrcElem, Src, impl FnMut(&SrcElem, &SrcElem) -> bool, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone,
        Src: AsRef<[SrcElem]>,
        Key: PartialOrd,
        ExtractKey: FnMut(&SrcElem) -> Key,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            _,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(
            src,
            move |x, y| extract_key(x) < extract_key(y),
            SrcElem::clone,
        ))
    }

    /// TODO
    pub fn shard_sort<SrcElem, Src>(
        src: Src,
    ) -> ShardSorted<SrcElem, Src, fn(&SrcElem, &SrcElem) -> bool, fn(&SrcElem) -> SrcElem>
    where
        SrcElem: Clone + PartialOrd,
        Src: AsRef<[SrcElem]>,
    {
        ShardSorted(shard_sort_impl::<
            SrcElem,
            Src,
            fn(&SrcElem, &SrcElem) -> bool,
            SrcElem,
            fn(&SrcElem) -> SrcElem,
        >(src, SrcElem::lt, SrcElem::clone))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

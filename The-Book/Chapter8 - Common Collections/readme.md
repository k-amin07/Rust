This chapter covers the common collections (data structures) included in the rust std lib. Unlike the standard arrays, these collections are stored on the heap. Rust's collections can be grouped into four major categories

- Sequences: Vec (vectors), VecDeque (queue), LinkedList (a doubly linkedlist).
- Maps: HashMap (key value pair, like python dict or JS object), BtreeMap (ordered map based on Btrees).
- Sets: HashSet (implementation of hashmap where value is ()), BtreeSet (ordered set based on Btrees).
- Misc: BinaryHeap

More info on these (like when to use what) can be found in the [official docs](https://doc.rust-lang.org/std/collections/index.html). Look into implementation of these as they can help learn/refresh best practices of Rust.

This chapter discusses 

- [Vectors](1.vectors.md)
- [Strings](2.strings.md)
- [HashMaps](3.hashmaps.md)
struct MutatorSequence {

}

struct MutatorSegment {
    length: SegmentLength
}

enum SegmentLength {
    Fixed(u32),
    Range(u32, u32),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

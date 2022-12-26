use std::str::FromStr;
use std::fmt::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Range is a closed integer range, [l,h].
/// Closed means l and h are part of the range.
/// TODO: it should ensure the invariant that l is less than or equal to h at construction and after.
struct Range {
    l: i32,
    h: i32,
    _private: (),
  }

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ls, hs) = s.split_once('-').ok_or(ParseRangeError)?;
        let lp = ls.parse::<i32>().map_err(|_| ParseRangeError)?;
        let hp = hs.parse::<i32>().map_err(|_| ParseRangeError)?;
        if hp < lp {
            return Err(ParseRangeError);
        }
        Ok(Range::new(lp, hp))
    }
}

impl Range {

    // Create a new Range object.  Panics if l > h;
    fn new(l: i32, h: i32) -> Self {
        assert!(l <= h);  // TODO: figure out the idiomatic way to handle h > l instead of panicing.
        Range{ l:l, h:h, _private: ()}
    }

    /// r1.disjoins(r2) is true iff no integer in range r1 is also in r2.
    /// This is a synonym for "not overlaps", that is:
    ///   !r1.overlaps(r2) iff r1.disjoins(r2)
    fn disjoins(self, r2: Range) -> bool {
        // Given: r1 = Range{a,b}, and r2 = Range{c,d}:  c-d can be the following ways and _not_ overlap (be disjoint)
        //        a___b        
        //   c__d
        //             c__d
        r2.h<self.l || r2.l>self.h
    }
 
    /// r1.overlaps(r2) is true iff some integer in range r1 is also in r2.
    /// See also: is_disjoint_with()
    fn overlaps(self, r2: Range) -> bool {
        !self.disjoins(r2)
    }

    // True iff r1 fully contains r2.
    fn fully_contains(self, r2: Range) -> bool {
        self.l <= r2.l && r2.h <= self.h
    }
}

#[test]
fn test_from_str_for_range_good() {
    let res = String::from("1-2").parse::<Range>();
    assert!(res.is_ok());
    assert_eq!(Range::new(1, 2), res.unwrap());
    let res2 = String::from("1-1").parse::<Range>();
    assert!(res2.is_ok());
    assert_eq!(Range::new(1, 1), res2.unwrap());
    let res3 = String::from("103-567").parse::<Range>();
    assert!(res3.is_ok());
    assert_eq!(Range::new(103, 567), res3.unwrap());
}

#[test]
fn test_from_str_for_range_bad() {
    let res = String::from("100-99").parse::<Range>();
    assert!(!res.is_ok());
}

fn parse_range_pair_from_str(s: &str) -> Result<(Range, Range), Error> {
    let (r1s, r2s) = s.split_once(',').ok_or(Error)?;
    let r1 = r1s.parse::<Range>().map_err(|_| Error)?;
    let r2 = r2s.parse::<Range>().map_err(|_| Error)?;
    Ok((r1, r2))
}

#[test]
fn test_parse_range_pair_from_str() {
    let res = parse_range_pair_from_str(&String::from("1-2,3-5"));
    assert!(res.is_ok());
    assert_eq!((Range::new(1,2), Range::new(3,5)), res.unwrap());
}

#[test]
fn test_fully_contains() {
    let r14 = Range::new(1, 4);
    let r23 = Range::new(2, 3);
    let r12 = Range::new(1, 2);
    assert_eq!(r14.fully_contains(r23), true);
    assert_eq!(r23.fully_contains(r14), false);
    assert_eq!(r12.fully_contains(r23), false);
    assert_eq!(r23.fully_contains(r12), false);

}

#[test]
fn test_overlaps(){
    let r12 = Range::new(1, 2);
    let r13 = Range::new(1, 3);
    let r14 = Range::new(1, 4);
    let r22 = Range::new(2, 2);
    let r23 = Range::new(2, 3);
    let r24 = Range::new(2, 4);
    let r33 = Range::new(3, 3);
    let r34 = Range::new(3, 4);
    assert_eq!(r14.overlaps(r23), true);
    assert_eq!(r13.overlaps(r23), true);
    assert_eq!(r12.overlaps(r23), true);
    assert_eq!(r22.overlaps(r23), true);
    assert_eq!(r33.overlaps(r23), true);
    assert_eq!(r23.overlaps(r14), true);
    assert_eq!(r23.overlaps(r13), true);
    assert_eq!(r23.overlaps(r12), true);
    assert_eq!(r23.overlaps(r22), true);
    assert_eq!(r23.overlaps(r33), true);
    assert_eq!(r12.overlaps(r34), false);
    assert_eq!(r12.overlaps(r34), false);
    assert_eq!(r13.overlaps(r24), true);
}

pub fn do_day4(input: &str, mode: i32)-> String {
    let mut containments = 0;
    let mut overlaps = 0;

    for line in input.split("\n") {
        let l = line.len();
        if l == 0 { continue; }
        let (r1, r2) = parse_range_pair_from_str(line).expect("Should be able to parse range pair.");
        if  r1.fully_contains(r2) || r2.fully_contains(r1) {
            containments += 1;
        }
        if r1.overlaps(r2) {
            overlaps += 1;
        }
    }
    match mode {
        1 => format!("{}", containments),
        2 => format!("{}", overlaps),
        _ => panic!("Bad mode {}", mode),
    }
}

#[test]
fn test_day4() {
    let advent_test_input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(do_day4(advent_test_input, 1), "2");
    assert_eq!(do_day4(advent_test_input, 2), "4");
}
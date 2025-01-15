/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Zerograph {
    pub name:String,
    pub tag: String,
    pub start: usize,
    pub startstrand: String,
    pub tagadd: String,
    pub end: usize,
    pub endstrand: String,
    pub cigar: String,
    pub connectnode: String,
    pub node:(String, String, usize)
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Segment {
   pub name: String,
   pub tag: String,
   pub seq: String,
}


#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GraphWrite {
   pub name: String,
   pub tag: String,
   pub start: usize,
   pub startstrand: String,
   pub tagadd: String,
   pub end: usize,
   pub endstrand: String,
   pub cigar: String,
   pub connection: String,
   pub asmstart: String,
   pub asmend: String,
}

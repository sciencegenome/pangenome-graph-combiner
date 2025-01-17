# rust-pangenome-graph-combiner
 - rust pangenome linerarization and combination approach
 - writes the 0 segment approach as asm for the minigraph alignment.
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.

 ```
 cargo build
 
 ```

 ```
 ╭─gauravsablok@fedora ~/Desktop/work/rust-pangenome-graph-combiner  ‹main*› 
 ╰─➤  ./target/debug/rust-pangenome-graph-combiner -h
 Usage: rust-pangenome-graph-combiner <GRAPH>

  Arguments:
  <GRAPH>  please provide the path to the graph file

 Options:
  -h, --help     Print help
  -V, --version  Print version
 
 ```
 - to run the binary 

 ```
 ╭─gauravsablok@fedora ~/Desktop/rust/pangenome-graph-combiner  ‹main*›
 ╰─➤  ./rust-pangenome-graph-combiner ./sample-file/sample-pangenome.gfa
 L       MTh0    0       +       MTh4001 4001    +       0M      SR:i:0  L1:i:4001       L2:i:501
 L       MTh4001 4001    +       MTh4502 4502    +       0M      SR:i:0  L1:i:501        L2:i:5003
 L       MTh4502 4502    +       MTh9505 9505    +       0M      SR:i:0  L1:i:5003       L2:i:3509
 L       MTh9505 9505    +       MTh13014        13014   +       0M      SR:i:0  L1:i:3509       L2:i:502
 L       MTh13014        13014   +       MTh13516        13516   +       0M      SR:i:0  L1:i:502        L2:i:3053
 Results have been written:graph asm have been written

 Gaurav Sablok

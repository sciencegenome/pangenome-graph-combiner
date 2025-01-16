mod args;
mod graph;
use crate::args::AsmArgs;
use crate::graph::GraphWrite;
use crate::graph::Segment;
use crate::graph::Zerograph;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-1-15

 rust-asm-prepare: converts the pangenome graph into the
 asm format for writing the asm format.

*/

fn main() {
    let args = AsmArgs::parse();
    let output = graph_asm(&args.graph).unwrap();
    println!("Results have been written:{}", output);
}

fn graph_asm(path: &str) -> Result<String, Box<dyn Error>> {
    let graphopen = File::open(path).expect("file not found");
    let graphread = BufReader::new(graphopen);
    let mut segment_hold: Vec<Segment> = Vec::new();
    let mut graphadd: Vec<Zerograph> = Vec::new();
    for i in graphread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("L") {
            let linevec = line.split("\t").collect::<Vec<_>>();
            let startsplit: usize = linevec[1]
                .split(|c: char| !c.is_numeric())
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| x.is_empty())
                .collect::<Vec<_>>()
                .join("")
                .parse::<usize>()
                .unwrap();
            let endsplit = linevec[3]
                .split(|c: char| !c.is_numeric())
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| x.is_empty())
                .collect::<Vec<_>>()
                .join("")
                .parse::<usize>()
                .unwrap();
            let connectionadd: (String, String, usize) = (
                linevec[6].split(":").collect::<Vec<_>>()[0].to_string(),
                linevec[6].split(":").collect::<Vec<_>>()[1].to_string(),
                linevec[6].split(":").collect::<Vec<_>>()[2]
                    .parse::<usize>()
                    .unwrap(),
            );
            graphadd.push(Zerograph {
                name: linevec[0].to_string(),
                tag: linevec[1].to_string(),
                start: startsplit,
                startstrand: linevec[2].to_string(),
                tagadd: linevec[3].to_string(),
                end: endsplit,
                endstrand: linevec[4].to_string(),
                cigar: linevec[5].to_string(),
                connectnode: linevec[6].to_string(),
                node: connectionadd,
            });
        } else if line.starts_with("S") {
            let newline = line
                .split("\t")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            segment_hold.push(Segment {
                name: newline[0].to_string(),
                tag: newline[1].to_string(),
                seq: newline[2].to_string(),
            })
        }
    }

    // making a zero node graph by the comparative approach as there will be
    // only one node traversal, so there is no need to add a left child instead
    // make a struct based approach. Iterate till the last node prior to the leaf node.

    let mut graphsort_write: Vec<GraphWrite> = Vec::new();
    for i in 0..graphadd.len() - 1 {
        if graphadd[i].node.2 == 0 {
            graphsort_write.push(GraphWrite {
                name: graphadd[i].name.clone(),
                tag: graphadd[i].tag.clone(),
                start: graphadd[i].start,
                startstrand: graphadd[i].startstrand.clone(),
                tagadd: graphadd[i].tagadd.clone(),
                end: graphadd[i].end,
                endstrand: graphadd[i].endstrand.clone(),
                cigar: graphadd[i].cigar.clone(),
                connection: graphadd[i].connectnode.clone(),
                asmstart: format!("{}:{}:{}", "L1", "i", graphadd[i].end - graphadd[i].start),
                asmend: format!("{}:{}:{}", "L2", "i", graphadd[i + 1].end - graphadd[i].end),
            });
        }
    }

    // separate the leaf node and add that information as a last push with the end being the
    // length of the leaf node seq.

    let mut last_node: Vec<GraphWrite> = Vec::new();
    for i in 0..graphadd.len() - graphadd.len() - 1 {
        for j in segment_hold.iter() {
            if graphadd[i].tag == j.name {
                last_node.push(GraphWrite {
                    name: graphadd[i].name.clone(),
                    tag: graphadd[i].tag.clone(),
                    start: graphadd[i].start,
                    startstrand: graphadd[i].startstrand.clone(),
                    tagadd: graphadd[i].tagadd.clone(),
                    end: graphadd[i].end,
                    endstrand: graphadd[i].endstrand.clone(),
                    cigar: graphadd[i].cigar.clone(),
                    connection: graphadd[i].connectnode.clone(),
                    asmstart: format!("{}:{}:{}", "L1", "i", graphadd[i].end - graphadd[i].start),
                    asmend: format!("{}:{}:{}", "L2", "i", j.seq.len()),
                });
            }
        }
    }

    Ok("graph asm have been written".to_string())
}

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
*Date 2024-1-17

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
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
                .join("")
                .parse::<usize>()
                .unwrap();
            let endsplit = linevec[3]
                .split(|c: char| !c.is_numeric())
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| !x.is_empty())
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
            // checks the node connection here so that it only takes the 0 segment node.
            if connectionadd.2 == 0 {
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
            }
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

    let mut graphsort_write: Vec<GraphWrite> = Vec::new();
    for i in 0..graphadd.len() - 1 {
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
            asmend: format!(
                "{}:{}:{}",
                "L2",
                "i",
                graphadd[i + 1].end - graphadd[i + 1].start
            ),
        });
    }

    // adding the last leaf node by clone value so that you dont have to iterate till the end and
    // if the leaf node is missing.

    let new = graphadd.clone();
    let leafnode = new[new.len() - 1].clone();
    let mut leafnode_write: Vec<GraphWrite> = Vec::new();
    for i in segment_hold.iter() {
        if i.tag == leafnode.tagadd {
            leafnode_write.push(GraphWrite {
                name: leafnode.name.clone(),
                tag: leafnode.tag.clone(),
                start: leafnode.start,
                startstrand: leafnode.startstrand.clone(),
                tagadd: leafnode.tagadd.clone(),
                end: leafnode.end,
                endstrand: leafnode.endstrand.clone(),
                cigar: leafnode.cigar.clone(),
                connection: leafnode.connectnode.clone(),
                asmstart: format!("{}:{}:{}", "L1", "i", leafnode.end - leafnode.start),
                asmend: format!("{}:{}:{}", "L2", "i", i.seq.len()),
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

    let mut graphwrite = File::create("graph-write.txt").expect("file not present");
    for i in graphsort_write.clone().into_iter() {
        writeln!(
            graphwrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.name,
            i.tag,
            i.start,
            i.startstrand,
            i.tagadd,
            i.end,
            i.endstrand,
            i.cigar,
            i.connection,
            i.asmstart,
            i.asmend
        )
        .expect("line not present");
    }

    for j in leafnode_write.clone().into_iter() {
        writeln!(
            graphwrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            j.name,
            j.tag,
            j.start,
            j.startstrand,
            j.tagadd,
            j.end,
            j.endstrand,
            j.cigar,
            j.connection,
            j.asmstart,
            j.asmend
        )
        .expect("line not present");
    }

    for i in graphsort_write.into_iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.name,
            i.tag,
            i.start,
            i.startstrand,
            i.tagadd,
            i.end,
            i.endstrand,
            i.cigar,
            i.connection,
            i.asmstart,
            i.asmend
        );
    }

    for j in leafnode_write.into_iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            j.name,
            j.tag,
            j.start,
            j.startstrand,
            j.tagadd,
            j.end,
            j.endstrand,
            j.cigar,
            j.connection,
            j.asmstart,
            j.asmend
        )
    }

    Ok("graph asm have been written".to_string())
}

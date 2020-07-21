use bio::io::fasta;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use structopt::StructOpt;

/// Combine alignment and tree files into a nexus file.
#[derive(Debug, StructOpt)]
pub struct Config {
    /// Path to tree file
    #[structopt(parse(from_os_str))]
    pub tree: PathBuf,

    /// Path to alignment fasta
    #[structopt(parse(from_os_str))]
    pub alignment: PathBuf,
}

/// Ensures that all sequences are the same length.
fn read_fasta(fasta: &Path) -> Vec<fasta::Record> {
    let reader = fasta::Reader::from_file(fasta).expect("problem opening fasta file!");

    let mut alignment_length = 0;
    let mut records = Vec::new();

    for (idx, rec) in reader.records().enumerate() {
        let rec = rec.expect("couldn't get record from fasta file!");

        if idx == 0 {
            alignment_length = rec.seq().len();
        } else if alignment_length != rec.seq().len() {
            panic!("All the sequences aren't the same length! Is this an alignment?");
        }

        records.push(rec);
    }

    records
}

fn write_nexus(records: Vec<fasta::Record>, tree: String) {
    let alignment_length = records[0].seq().len();
    let record_ids = records
        .iter()
        .map(|rec| format!("\t{}", rec.id()))
        .collect::<Vec<String>>()
        .join("\n");

    let alignment_records = records
        .iter()
        .map(|rec| format!("\t{}\t{}", rec.id(), str::from_utf8(rec.seq()).unwrap()))
        .collect::<Vec<String>>()
        .join("\n");

    let taxa_section = format!(
        "begin taxa;\ndimensions ntax={};\n\ttaxlabels\n{}\n;\nend;",
        records.len(),
        record_ids
    );

    // Todo this assumes the gap char is a '-'.
    let alignment_section = format!(
        "begin characters;\n\
        \tdimensions nchar={};\n\
        \tformat datatype=dna missing=? gap=-;\n\
        \tmatrix\n\
        {}\n\
        ;\n\
        end;",
        alignment_length, alignment_records
    );

    let tree_section = format!("begin trees;\n\ttree tree_1 = [&R] {}end;", tree);

    let output = format!(
        "#NEXUS\n{}\n\n{}\n\n{}\n",
        taxa_section, alignment_section, tree_section
    );

    println!("{}", output);
}

pub fn run(config: Config) {
    let records = read_fasta(&config.alignment);
    let tree = fs::read_to_string(&config.tree).expect("couldn't read tree file!");

    write_nexus(records, tree);
}

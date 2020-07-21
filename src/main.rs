use geneious_nexus::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();

    geneious_nexus::run(config);
}

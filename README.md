# geneious-nexus

Combine Newick tree files with alignment files to produce a Nexus file for Geneious, letting you view tree files with the alignment that generated them.

## Install

If you don't have Rust, [install it](https://www.rust-lang.org/tools/install).

Clone this repo:

```
git clone https://github.com/mooreryan/geneious-nexus && cd geneious-nexus
```

Compile:

```
cargo build --release
```

You should now have a compiled binary in `./target/release/geneious-nexus`.  Move it or link it somewhere on your path.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. This file may not be copied, modified, or distributed except according to those terms.
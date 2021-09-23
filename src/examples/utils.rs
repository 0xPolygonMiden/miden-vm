use winterfell::{FieldExtension, HashFunction, ProofOptions};

pub fn parse_args(args: &[String]) -> (usize, ProofOptions) {
    let default_options = ProofOptions::new(
        32,
        8,
        0,
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,
        128,
    );
    if args.len() == 1 {
        return (6, default_options);
    }

    let n: usize = args[1].parse().unwrap();
    if args.len() == 2 {
        return (n, default_options);
    }

    let ext_factor: usize;
    let num_queries: usize;
    let grind_factor: u32;

    if args.len() == 3 {
        ext_factor = args[2].parse().unwrap();
        num_queries = default_options.num_queries();
        grind_factor = default_options.grinding_factor();
    } else if args.len() == 4 {
        ext_factor = args[2].parse().unwrap();
        num_queries = args[3].parse().unwrap();
        grind_factor = default_options.grinding_factor();
    } else {
        ext_factor = args[2].parse().unwrap();
        num_queries = args[3].parse().unwrap();
        grind_factor = args[4].parse().unwrap();
    }

    let options = ProofOptions::new(
        num_queries,
        ext_factor,
        grind_factor,
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,
        128,
    );

    (n, options)
}

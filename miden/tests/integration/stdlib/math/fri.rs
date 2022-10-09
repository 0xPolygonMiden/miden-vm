use super::build_test;
use super::Felt;
use air::StarkField;
use math::log2;
use miden::AdviceSet;
use serde_json_any_key::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

mod data;
use data::*;

#[test]
fn preprocess() {
    let source = "
        use.std::math::fri
        begin
            exec.fri::outer
        end
        ";
    let depth = 13;
    let domain_size = 1 << depth;
    let num_queries = 32;
    let blowup_exp = 3;
    let (tape, set) = prepare_advice(depth, domain_size, num_queries, blowup_exp);

    let test = build_test!(source, &[], &tape, set);

    test.expect_stack(&[]);
}

// Helper functions

fn prepare_advice(
    depth: usize,
    domain_size: u32,
    num_queries: usize,
    blowup_exp: usize,
) -> (Vec<u64>, Vec<AdviceSet>) {
    let mut tape = vec![];
    let com: Vec<u64> = COM.into_iter().flat_map(|a| a.into_iter()).collect();
    let domain_generator = Felt::get_root_of_unity(log2(domain_size as usize)).as_int();

    tape.push(depth as u64);
    tape.push(domain_generator);
    tape.push(domain_size as u64);
    tape.push(num_queries as u64);

    for i in (0..(depth - blowup_exp)).rev() {
        tape.extend_from_slice(&com[(4 * i)..(4 * i + 4)]);
        tape.extend_from_slice(&ALPHAS[(4 * i)..(4 * i + 4)]);
    }

    tape.push(REMAINDER[0]);
    tape.push(REMAINDER[1]);
    tape.extend_from_slice(&ALL_QUERIES[..]);
    tape.extend_from_slice(&COM[COM.len() - 1]);

    let mut info_str = String::new();

    let mut f = File::open("./tests/integration/stdlib/math/fri/set.txt").expect("Unable to open file");
    f.read_to_string(&mut info_str)
        .expect("Unable to read string");

    let map: BTreeMap<Vec<u64>, BTreeMap<u64, Vec<Vec<u64>>>> = json_to_iter(&info_str)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let mut mp_set_all = vec![];
    let mut depth = depth as u32;
    for c in COM[..10].iter() {
        let mut mp_set = AdviceSet::new_merkle_path_set(depth).unwrap();
        let set = map.get(&c.to_vec()).unwrap();
        set.iter().for_each(|(k, v)| {
            mp_set
                .add_path(*k, to_word(&v[0]), to_path(&v[1..]))
                .unwrap()
        });
        depth -= 1;
        mp_set_all.push(mp_set);
    }

    (tape, mp_set_all)
}

fn to_path(v: &[Vec<u64>]) -> Vec<[Felt; 4]> {
    let mut result = vec![];
    for v in v.iter() {
        result.push([
            Felt::new(v[0]),
            Felt::new(v[1]),
            Felt::new(v[2]),
            Felt::new(v[3]),
        ])
    }
    result
}

fn to_word(root: &[u64]) -> [Felt; 4] {
    [
        Felt::new(root[0]),
        Felt::new(root[1]),
        Felt::new(root[2]),
        Felt::new(root[3]),
    ]
}
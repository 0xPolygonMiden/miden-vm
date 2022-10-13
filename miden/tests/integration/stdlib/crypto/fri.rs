use super::build_test;
use super::Felt;
use air::FieldElement;
use air::StarkField;
use math::log2;
use miden::AdviceSet;
use rand_utils::rand_value;
use serde_json_any_key::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use vm_core::QuadExtension;

mod data;
use data::*;

type ExtElement = QuadExtension<Felt>;

#[test]
fn verify() {
    let source = "
        use.std::crypto::fri
        begin
            exec.fri::preprocess
            exec.fri::verify
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

#[test]
fn fold_2() {
    let source = "
        use.std::crypto::fri
        begin
            exec.fri::fold_2
        end";

    // --- simple case ----------------------------------------------------------------------------
    let b = ExtElement::new(
        Felt::new(11306949585462557770),
        Felt::new(11306949585462557770),
    );
    let a = ExtElement::new(
        Felt::new(2433767798173919658),
        Felt::new(2433767798173919658),
    );
    let c = ExtElement::new(
        Felt::new(4007042871203243940),
        Felt::new(3131225830454393212),
    );
    let d = ExtElement::new(Felt::new(9597810334906255130), Felt::new(0));
    let two = ExtElement::new(Felt::new(2), Felt::new(0));
    /*
    BaseElement(11306949585462557770), BaseElement(11306949585462557770), BaseElement(2433767798173919658), BaseElement(2433767798173919658), BaseElement(3131225830454393212), BaseElement(4007042871203243940), BaseElement(0), BaseElement(9597810334906255130) */
    let arr_a = vec![a];
    let arr_a = ExtElement::as_base_elements(&arr_a);
    let arr_b = vec![b];
    let arr_b = ExtElement::as_base_elements(&arr_b);
    let arr_c = vec![c];
    let arr_c = ExtElement::as_base_elements(&arr_c);
    let arr_d = vec![d];
    let arr_d = ExtElement::as_base_elements(&arr_d);

    let test = build_test!(
        source,
        &[
            arr_d[0].as_int(),
            arr_d[1].as_int(),
            arr_c[0].as_int(),
            arr_c[1].as_int(),
            arr_b[0].as_int(),
            arr_b[1].as_int(),
            arr_a[0].as_int(),
            arr_a[1].as_int()
        ]
    );
    let result = (a + b + ((a - b) * c / d)) / two;

    let arr_r = vec![result];
    let arr_r = ExtElement::as_base_elements(&arr_r);

    test.expect_stack(&[arr_r[1].as_int(), arr_r[0].as_int()]);

    // --- random values --------------------------------------------------------------------------
    let a = ExtElement::new(
        Felt::new(rand_value::<u64>()),
        Felt::new(rand_value::<u64>()),
    );
    let b = ExtElement::new(
        Felt::new(rand_value::<u64>()),
        Felt::new(rand_value::<u64>()),
    );
    let c = ExtElement::new(
        Felt::new(rand_value::<u64>()),
        Felt::new(rand_value::<u64>()),
    );
    let d = ExtElement::new(Felt::new(rand_value::<u64>()), Felt::new(0));

    let arr_a = vec![a];
    let arr_a = ExtElement::as_base_elements(&arr_a);
    let arr_b = vec![b];
    let arr_b = ExtElement::as_base_elements(&arr_b);
    let arr_c = vec![c];
    let arr_c = ExtElement::as_base_elements(&arr_c);
    let arr_d = vec![d];
    let arr_d = ExtElement::as_base_elements(&arr_d);

    let test = build_test!(
        source,
        &[
            arr_d[0].as_int(),
            arr_d[1].as_int(),
            arr_c[0].as_int(),
            arr_c[1].as_int(),
            arr_b[0].as_int(),
            arr_b[1].as_int(),
            arr_a[0].as_int(),
            arr_a[1].as_int()
        ]
    );
    let result = (a + b + ((a - b) * c / d)) / two;

    let arr_r = vec![result];
    let arr_r = ExtElement::as_base_elements(&arr_r);

    test.expect_stack(&[arr_r[1].as_int(), arr_r[0].as_int()]);
}

#[test]
fn next_pos_exp() {
    let source = "
        use.std::crypto::fri
        begin
            exec.fri::next_pos_exp
        end";

    let nor = Felt::new(18446744069414584320);
    let offset = Felt::new(7);
    // --- simple case 1----------------------------------------------------------------------------
    let poe = Felt::new(4);
    let b = Felt::new(0);

    let test = build_test!(source, &[poe.as_int(), poe.as_int(), b.as_int()]);

    test.expect_stack(&[(poe * poe).as_int(), (poe * offset).as_int()]);

    // --- simple case 2----------------------------------------------------------------------------
    let poe = Felt::new(4);
    let b = Felt::new(1);

    let test = build_test!(source, &[poe.as_int(), poe.as_int(), b.as_int()]);

    test.expect_stack(&[(poe * poe).as_int(), ((poe * offset) / nor).as_int()]);

    // --- random values 1--------------------------------------------------------------------------
    let poe = Felt::new(rand_value::<u64>());
    let b = Felt::new(0);

    let test = build_test!(source, &[poe.as_int(), poe.as_int(), b.as_int()]);

    test.expect_stack(&[(poe * poe).as_int(), (poe * offset).as_int()]);

    // --- random values 1--------------------------------------------------------------------------
    let poe = Felt::new(rand_value::<u64>());
    let b = Felt::new(1);

    let test = build_test!(source, &[poe.as_int(), poe.as_int(), b.as_int()]);

    test.expect_stack(&[(poe * poe).as_int(), ((poe * offset) / nor).as_int()]);
}

#[test]
fn prepare_next() {
    let source = "
        use.std::crypto::fri
        begin
            exec.fri::prepare_next
        end";

    // --- simple case 1----------------------------------------------------------------------------
    let d = Felt::new(1 << 12);
    let p = Felt::new(3874);
    let com = vec![Felt::new(0), Felt::new(0), Felt::new(0), Felt::new(0)];
    let t_d = 12;
    let e0 = Felt::new(0);
    let e1 = Felt::new(0);
    let a0 = Felt::new(0);
    let a1 = Felt::new(0);
    let poe = Felt::new(1);
    let add_p = Felt::new(1 << 32);

    let test = build_test!(
        source,
        &[
            add_p.as_int(),
            poe.as_int(),
            d.as_int(),
            p.as_int(),
            e0.as_int(),
            e1.as_int(),
            t_d,
        ],
        &[
            a0.as_int(),
            a1.as_int(),
            com[0].as_int(),
            com[1].as_int(),
            com[2].as_int(),
            com[3].as_int(),
        ],
        vec![]
    );

    test.expect_stack(&[
        d.as_int(),
        p.as_int(),
        com[0].as_int(),
        com[1].as_int(),
        com[2].as_int(),
        com[3].as_int(),
        t_d,
        e1.as_int(),
        e0.as_int(),
        poe.as_int(),
        a1.as_int(),
        a0.as_int(),
        (add_p - Felt::new(2)).as_int(),
    ]);
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

    let mut f =
        File::open("./tests/integration/stdlib/crypto/fri/set.txt").expect("Unable to open file");
    f.read_to_string(&mut info_str)
        .expect("Unable to read string");

    let map: BTreeMap<Vec<u64>, BTreeMap<u64, Vec<Vec<u64>>>> = json_to_iter(&info_str)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let mut mp_set_all = vec![];
    let mut depth = depth as u32;
    for c in COM[..10].iter() {
        let set = map.get(&c.to_vec()).unwrap();
        let mut indices = vec![];
        let mut paths = vec![];
        let mut values = vec![];

        set.iter().for_each(|(k, v)| {
            indices.push(*k);
            paths.push(to_path(&v[1..]));
            values.push(to_word(&v[0]));
        });
        let mp_set = AdviceSet::new_merkle_path_set(indices, values, paths, depth).unwrap();

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

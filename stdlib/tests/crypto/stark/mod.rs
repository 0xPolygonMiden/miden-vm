use assembly::Assembler;
use miden_air::{Felt, FieldElement, FieldExtension, HashFunction, PublicInputs};
use num::traits::sign;
use processor::{DefaultHost, Program, ProgramInfo, Word};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use signature::{generate_advice_inputs_signature, VerifierData};
use test_utils::{
    crypto::{rpo_falcon512::PublicKey, rpo_stark::{PublicInputs as SignaturePublicInputs, SecretKey}}, math::{polynom, ExtensionOf}, prove, rand::{rand_array, rand_value}, verify, AdviceInputs, MemAdviceProvider, ProvingOptions, QuadFelt, StackInputs, VerifierError
};


mod verifier_recursive;
use verifier_recursive::{generate_advice_inputs, to_int_vec, QuadExt, };

mod signature;



// Note: Changes to MidenVM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[test]
#[ignore]
fn signature_verification() {

    let VerifierData { initial_stack, tape, store, advice_map } =
        generate_signature_data().unwrap();

    // Verify inside Miden VM
    let source = "
        use.std::crypto::dsa::rpo_stark::verifier
        begin
            exec.verifier::verify
        end
        ";

    let test = build_test!(source, &initial_stack, &tape, store, advice_map);

    test.expect_stack(&[]);
}

fn generate_signature_data() ->  Result<VerifierData, VerifierError>  {
    let seed = [0_u8; 32];
        let mut rng = ChaCha20Rng::from_seed(seed);
        let sk = SecretKey::with_rng(&mut rng);
        let sk = SecretKey::from_word(Word::default()); 

        let message = Word::default();
        let signature = sk.sign(message);
//println!("signature {:?}", signature);
        let pk = sk.compute_public_key();
       // signature.verify(message, pk.inner());
        let proof = signature.inner();

        let pub_inputs = SignaturePublicInputs::new(pk.inner(), message);
        
    let res = generate_advice_inputs_signature(proof, pub_inputs);
       // println!("res is {:?}", res);
    Ok(res.unwrap())
}

// Note: Changes to MidenVM may cause this test to fail when some of the assumptions documented
// in `stdlib/asm/crypto/stark/verifier.masm` are violated.
#[test]
#[ignore]
fn stark_verifier_e2f4() {
    // An example MASM program to be verified inside Miden VM.
    // Note that output stack-overflow is not yet supported because of the way we handle public
    // inputs in the STARK verifier is not yet general enough. Thus the output stack should be
    // of size exactly 16.
    let example_source = "begin
            repeat.32
                swap dup.1 add
            end
        end";
    let mut stack_inputs = vec![0_u64; 16];
    stack_inputs[15] = 0;
    stack_inputs[14] = 1;

    let VerifierData {
        initial_stack,
        advice_stack: tape,
        store,
        advice_map,
    } = generate_recursive_verifier_data(example_source, stack_inputs).unwrap();

    // Verify inside Miden VM
    let source = "
        use.std::crypto::stark::verifier
        begin
            exec.verifier::verify
        end
        ";

    let test = build_test!(source, &initial_stack, &tape, store, advice_map);

    test.expect_stack(&[]);
}

// Helper function for recursive verification
pub fn generate_recursive_verifier_data(
    source: &str,
    stack_inputs: Vec<u64>,
) -> Result<VerifierData, VerifierError> {
    let program: Program = Assembler::default().assemble_program(source).unwrap();
    let stack_inputs = StackInputs::try_from_ints(stack_inputs).unwrap();
    let advice_inputs = AdviceInputs::default();
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    let mut host = DefaultHost::new(advice_provider);

    let options =
        ProvingOptions::new(11, 8, 12, FieldExtension::Quadratic, 4, 7, HashFunction::Rpo256);

    let (stack_outputs, proof) = prove(&program, stack_inputs.clone(), &mut host, options).unwrap();

    let program_info = ProgramInfo::from(program);
    let res =
        verify(program_info.clone(), stack_inputs.clone(), stack_outputs.clone(), proof.clone())
            .unwrap();
    println!("res of veri {:?}", res);
    // build public inputs and generate the advice data needed for recursive proof verification
    let pub_inputs = PublicInputs::new(program_info, stack_inputs, stack_outputs);

    let (_, proof) = proof.into_parts();
    Ok(generate_advice_inputs(proof, pub_inputs).unwrap())
}

/*
poly 0 at (14281334308168704527, 10495222116659147228) is (10060846794257653263, 546027550684353096)
poly 1 at (14281334308168704527, 10495222116659147228) is (10903134704496183277, 5579342745194126910)
poly 2 at (14281334308168704527, 10495222116659147228) is (12893269862983042775, 17647699874671928675)
poly 3 at (14281334308168704527, 10495222116659147228) is (11122676875213712094, 5263064823292274143)
poly 4 at (14281334308168704527, 10495222116659147228) is (41855652377006736, 9909668281301462903)
poly 5 at (14281334308168704527, 10495222116659147228) is (14932615313904988467, 7592103672217106309)
poly 6 at (14281334308168704527, 10495222116659147228) is (20740824113534935, 3818898257924990209)
poly 7 at (14281334308168704527, 10495222116659147228) is (15272117991568816033, 14472765353298605242)
poly 8 at (14281334308168704527, 10495222116659147228) is (1409366930799599557, 4944651295747950778)
poly 9 at (14281334308168704527, 10495222116659147228) is (15967857750552075868, 2857971283248518823)
poly 10 at (14281334308168704527, 10495222116659147228) is (11602683508887447121, 18097224529377507115)
poly 11 at (14281334308168704527, 10495222116659147228) is (6426931889180492583, 17042609443108091430)
poly 12 at (14281334308168704527, 10495222116659147228) is (3787620295263208906, 5581463112448029526)
poly 13 at (14281334308168704527, 10495222116659147228) is (14819709244247351689, 9116944519363158237)
poly 14 at (14281334308168704527, 10495222116659147228) is (18258588819309979526, 12167294900610607376)
poly 15 at (14281334308168704527, 10495222116659147228) is (13323880019629950849, 16070006070861920166)
poly 16 at (14281334308168704527, 10495222116659147228) is (15088898314701251493, 17537315582941049748)
poly 17 at (14281334308168704527, 10495222116659147228) is (601580842485586130, 9327490444933127718)
poly 18 at (14281334308168704527, 10495222116659147228) is (2822984643375099546, 7202787223205524049)
poly 19 at (14281334308168704527, 10495222116659147228) is (14598635898512929731, 4144988733395143778)
poly 20 at (14281334308168704527, 10495222116659147228) is (17626662679135368322, 1223116119813894249)
poly 21 at (14281334308168704527, 10495222116659147228) is (4860278503993160809, 7161649524312813954)
poly 22 at (14281334308168704527, 10495222116659147228) is (5189975042430731858, 9217744093123510922)
poly 23 at (14281334308168704527, 10495222116659147228) is (3964029779034283138, 11151025876422081715)

*/


#[test]
fn second_mds_multiply() {
    // Verify inside Miden VM
    let source = "
        use.std::crypto::dsa::rpo_stark::rpo_stark
        begin
            push.1000
            push.2.0.2.0
            repeat.3
                dup.4 add.1 swap.5
                mem_storew
            end

            dropw
            push.1.0.1.0
            repeat.3
                dup.4 add.1 swap.5
                mem_storew
            end

            dropw drop

            push.14281334308168704527 push.10495222116659147228 
            push.2000 push.1000
            exec.rpo_stark::multiply_mds_add_constant_apply_sbox_round

            add.2000 swap

            exec.rpo_stark::multiply_mds_add_constant

            push.3000
            padw
            repeat.6
                dup.4 add.1 swap.5
                mem_loadw
                movup.4
                padw
            end
            

        end
        ";


    let z = QuadExt::new(Felt::new(14281334308168704527), Felt::new(10495222116659147228));
    let mut ark = vec![];

    for poly in ark1_polys {
        let poly: Vec<Felt> = poly
            .iter()
            .map(|coef| {
                Felt::new(*coef)
                
            })
            .collect();
        let eval = polynom::eval(&poly, z);

        ark.push(eval);
    }
    //ark.reverse();
    let mut state: [QuadExt; 12] = [QuadExt::ONE; 12];
    state.iter_mut().take(6).for_each(|s| *s = s.double() );
    println!("state is   {:?}", state);
    //let mut initial_state = state.clone();
    apply_mds(&mut state);
    add_constants(&mut state, &ark);
    apply_sbox(&mut state);


    let mut ark = vec![];

    for poly in ark2_polys {
        let poly: Vec<Felt> = poly
            .iter()
            .map(|coef| {
                Felt::new(*coef)
                
            })
            .collect();
        let eval = polynom::eval(&poly, z);

        ark.push(eval);
    }

    apply_mds(&mut state);
    add_constants(&mut state, &ark);

    let mut input_stack = vec![];
    input_stack.push(14281334308168704527_u64);
    input_stack.push(10495222116659147228);

    //initial_state.reverse();
    //let tmp = to_int_vec( &initial_state) ;
   
   //let mut adv_stack = vec![];
   //adv_stack.extend_from_slice(&tmp);

    let test = build_test!(source, &[] );

    
    println!("res is   {:?}", ark);
    println!("state is   {:?}", state);
    test.expect_stack(&[]);
}
#[test]
fn multiply_mds_add_constant_apply_sbox_round() {
    // Verify inside Miden VM
    let source = "
        use.std::crypto::dsa::rpo_stark::rpo_stark
        begin
            push.1000
            push.2.0.2.0
            repeat.3
                dup.4 add.1 swap.5
                mem_storew
            end

            dropw
            push.1.0.1.0
            repeat.3
                dup.4 add.1 swap.5
                mem_storew
            end

            dropw drop

            push.14281334308168704527 push.10495222116659147228 
            push.2000 push.1000
            exec.rpo_stark::multiply_mds_add_constant_apply_sbox_round

            push.2000
            padw
            repeat.6
                dup.4 add.1 swap.5
                mem_loadw
                movup.4
                padw
            end
            

        end
        ";


    let z = QuadExt::new(Felt::new(14281334308168704527), Felt::new(10495222116659147228));
    let mut ark = vec![];

    for poly in ark1_polys {
        let poly: Vec<Felt> = poly
            .iter()
            .map(|coef| {
                Felt::new(*coef)
                
            })
            .collect();
        let eval = polynom::eval(&poly, z);

        ark.push(eval);
    }
    //ark.reverse();
    let mut state: [QuadExt; 12] = [QuadExt::ONE; 12];
    state.iter_mut().take(6).for_each(|s| *s = s.double() );
    println!("state is   {:?}", state);
    //let mut initial_state = state.clone();
    apply_mds(&mut state);
    add_constants(&mut state, &ark);
    apply_sbox(&mut state);

    let mut input_stack = vec![];
    input_stack.push(14281334308168704527_u64);
    input_stack.push(10495222116659147228);

    //initial_state.reverse();
    //let tmp = to_int_vec( &initial_state) ;
   
   //let mut adv_stack = vec![];
   //adv_stack.extend_from_slice(&tmp);

    let test = build_test!(source, &[] );

    
    println!("res is   {:?}", ark);
    println!("state is   {:?}", state);
    test.expect_stack(&[]);
}

#[test]
fn multiply_double_extension_by_base() {
    // Verify inside Miden VM
    let source = "
        use.std::crypto::dsa::rpo_stark::rpo_stark
        begin
            exec.rpo_stark::multiply_double_extension_by_base
        end
        ";

    let tau0 = rand_value::<QuadFelt>();
    let tau1 = rand_value::<QuadFelt>();
    let k0: Felt = rand_value();
    let k1: Felt = rand_value();
    let res0 = tau0.mul_base(k0);
    let res1 = tau1.mul_base(k1);
    let input_stack = vec![
        tau0.base_element(0).as_int(),
        tau0.base_element(1).as_int(),
        tau1.base_element(0).as_int(),
        tau1.base_element(1).as_int(),
        k0.as_int(),
        k1.as_int(),
    ];

    let res = res0 + res1;
    let output_stack = vec![res.base_element(1).as_int(), res.base_element(0).as_int()];

    let test = build_test!(source, &input_stack);
    test.expect_stack(&output_stack);
}

#[test]
fn sbox() {
    // Verify inside Miden VM
    let source = "
        use.std::crypto::dsa::rpo_stark::rpo_stark
        begin
            exec.rpo_stark::sbox
        end
        ";

    let tau = rand_value::<QuadFelt>();

    let input_stack = vec![tau.base_element(0).as_int(), tau.base_element(1).as_int()];

    let res = tau.exp((7_u32).into());
    let output_stack = vec![res.base_element(1).as_int(), res.base_element(0).as_int()];

    let test = build_test!(source, &input_stack);
    test.expect_stack(&output_stack);
}

/*
coef is 8329032129963563016
acc is (0, 0)
res is (8329032129963563016, 0)
coef is 8934619293596015107
acc is (8329032129963563016, 0)
res is (5357259674621098529, 0)
coef is 15061243612815629935
acc is (5357259674621098529, 0)
res is (18043538241885439730, 0)
coef is 17197876067442802066
acc is (18043538241885439730, 0)
res is (15585052757326223702, 0)
coef is 16438925404421745533
acc is (15585052757326223702, 0)
res is (4992160156068303057, 0)
coef is 3323980907020445132
acc is (4992160156068303057, 0)
res is (4845877461879073039, 0)
coef is 15627700026530187867
acc is (4845877461879073039, 0)
res is (16564465804631895702, 0)
coef is 2791148843003040487
acc is (16564465804631895702, 0)
res is (13708779853286870332, 0)
poly 0 at (5904545740857336763, 2641448861188835256) is (13708779853286870332, 0)

*/
#[test]
fn evaluate_ark1_index_0_at_z() {
    let source = "
        use.std::crypto::dsa::rpo_stark::round_constants
        begin
            # => [tau1, tau0, dest_ptr, dest_ptr, ...]

            exec.round_constants::evaluate_ark1_index_0_at_z
            # => [tau1, tau0, dest_ptr + 1, dest_ptr, ...]
            
            # Load the computed evaluation at tau
            padw movup.7 mem_loadw
            # => [0, 0, ev1, ev0, tau1, tau0, dest_ptr + 1, ...]

            # Clean up the stack
            drop drop
            # => [ev1, ev0, tau1, tau0, dest_ptr + 1, ...]

            # Fix overflow
            movup.5 drop
        end
        ";

    let tau = QuadFelt::new(Felt::new(2358509191725100597), Felt::new(12448696411509285448));
    let ptr = 1000;
    let ptr_nxt = 1001;
    let res = QuadFelt::new(Felt::new(9108108481109559168), Felt::new(1381914921145369315));
    let input_stack = vec![ptr, ptr, tau.base_element(0).as_int(), tau.base_element(1).as_int()];

    let output_stack = vec![
        res.base_element(1).as_int(),
        res.base_element(0).as_int(),
        tau.base_element(1).as_int(),
        tau.base_element(0).as_int(),
        ptr_nxt,
    ];

    let test = build_test!(source, &input_stack);
    test.expect_stack(&output_stack);
}

#[test]
fn evaluate_ark1_index_1_at_z() {
    let source = "
        use.std::crypto::dsa::rpo_stark::round_constants
        begin
            # => [tau1, tau0, dest_ptr, dest_ptr, ...]

            exec.round_constants::evaluate_ark1_index_1_at_z
            # => [tau1, tau0, dest_ptr + 1, dest_ptr, ...]
            
            # Load the computed evaluation at tau
            padw movup.7 mem_loadw
            # => [0, 0, ev1, ev0, tau1, tau0, dest_ptr + 1, ...]

            # Clean up the stack
            drop drop
            # => [ev1, ev0, tau1, tau0, dest_ptr + 1, ...]

            # Fix overflow
            movup.5 drop
        end
        ";

    let tau = QuadFelt::new(Felt::new(14281334308168704527), Felt::new(10495222116659147228));
    let ptr = 1000;
    let ptr_nxt = 1001;
    let res = QuadFelt::new(Felt::new(10903134704496183277), Felt::new(5579342745194126910));
    let input_stack = vec![ptr, ptr, tau.base_element(0).as_int(), tau.base_element(1).as_int()];

    let output_stack = vec![
        res.base_element(1).as_int(),
        res.base_element(0).as_int(),
        tau.base_element(1).as_int(),
        tau.base_element(0).as_int(),
        ptr_nxt,
    ];

    let test = build_test!(source, &input_stack);
    test.expect_stack(&output_stack);
}

const ark1_polys: [[u64; 8]; 12] = [
    [
        2791148843003040487,
        15627700026530187867,
        3323980907020445132,
        16438925404421745533,
        17197876067442802066,
        15061243612815629935,
        8934619293596015107,
        8329032129963563016,
    ],
    [
        12374840782518234980,
        13067814063463246108,
        16444767378767684843,
        14102478245486764913,
        5721113686559987364,
        5550835267666484285,
        1140255260049705235,
        2940403776379367555,
    ],
    [
        803239822289277331,
        92290315131409452,
        14504533961394498192,
        17449340622223708244,
        11641839262529699117,
        13830007435883778192,
        2171477269448177901,
        18024172012126180450,
    ],
    [
        17259550729090128080,
        1436126261944143983,
        6827772059756176056,
        13633102721671278493,
        438808730725699483,
        5407974288453498850,
        8312944094443125539,
        12083646389987382742,
    ],
    [
        16872030888212246909,
        10307413365916228519,
        5970696349452178882,
        3075676239487301499,
        6764475190363405683,
        12388465667776429451,
        5463940702410555170,
        4302341177141596798,
    ],
    [
        13010646010715444756,
        13220880625440862351,
        15043202536693295725,
        12450612311725651741,
        1975121089721419805,
        17751579298152306599,
        5865829689316994740,
        10135856292008745804,
    ],
    [
        11079243880688886569,
        13171475342452126305,
        16717118787075991203,
        12997762624630485971,
        2082764394649465048,
        2274009722649114980,
        11078166678912510243,
        14537022526074710084,
    ],
    [
        6412175863526823473,
        7693358564819205804,
        12953861830888836611,
        942265184773461437,
        1607941585775238966,
        4379845611455349528,
        58856250008039951,
        16473125605159627119,
    ],
    [
        5005263760224338834,
        11877952992388592984,
        13609269279460528668,
        12700182930600529552,
        13020259874513262126,
        1930782708828158469,
        5575776764248027070,
        12391274268940303373,
    ],
    [
        11718820523788741765,
        59883440729546652,
        6312723811819661455,
        5308884009005395250,
        6182287153329169016,
        13207315343261220442,
        4815800570707034136,
        8349688098368982860,
    ],
    [
        6043410183720093018,
        2126146309185433854,
        8378524768105627493,
        5544949132734068891,
        17681500018572116275,
        10699876530722034865,
        8011357321704081273,
        5725123156318075397,
    ],
    [
        6223946836251310965,
        16586492306509623392,
        6856802671405566180,
        16479930990938214663,
        13641986601858015391,
        6665166827227923662,
        9454375281307822385,
        8412441953430543726,
    ],
];


const ark2_polys: [[u64; 8]; 12] = [[4644899059340455409, 2588000450744117712, 13105238537181697838, 9145530290182703923, 8318881075359001860, 2260425315804771343, 13140762909627491530, 10125751776384352797],
[1076805373481731514, 731771438168089372, 1451573959488719120, 7220138601034011484, 14518803097042109937, 8441003397118172058, 16691081699485452106, 13848563765445705020],
[10296796646521415185, 5634304416804201670, 2436250166439517097, 175122579984732489, 17819414171265412835, 11568691945480450310, 4685303716418085212, 9212910395839541860],
[13748310607328698306, 3231680637487443842, 11897908975989921011, 14948979053368640201, 788938329072261875, 2424518612267165880, 14447959571237460393, 17817850486374744537],
[430095072692583909, 3538460359490439815, 4910259163119359388, 13172964728236666704, 13264785748607221492, 9148793266208888638, 12523400296259625004, 1323646892185216842],
[17837225074619059476, 6702081431642835557, 15726014635140220142, 14804677398279293696, 17716100798530125807, 12274533074791157450, 454935568758037748, 3171283654084997195],
[2011206516638815708, 12992430013458659144, 8696495089799708930, 1400178508260896298, 4445028994876124043, 15086337211977579054, 2579568427157492470, 3842347926541387017],
[14455558190054243213, 9476151068182324219, 4476065032800445994, 4772948326559040143, 9999073914541559972, 8857261784008983684, 15200374878312346200, 12519281252880051360],
[15872839284583424888, 9158419513185267937, 989723691436384985, 3480674361312569156, 7972171312030024281, 6916765091459382631, 5174110470653655799, 10891579218396689814],
[3270022819064790682, 17115077542742512990, 12665325866539717707, 2141009619237002231, 9832785799742116087, 10808339012888707871, 17545443011752642513, 12528974695128465622],
[8849418171382398211, 1150833344352386619, 6870113859747343930, 13474841269676445703, 12213798227721715260, 12131307142274672056, 11958348010538656609, 6096042974457287451],
[17284012187876944133, 10339865060086263172, 12560365515943446953, 8769164948183307270, 11644987068425776572, 16184522505189623125, 1188079598780402137, 16980889739006603967]];

/// RPO MDS matrix
pub const MDS: [[Felt; STATE_WIDTH]; STATE_WIDTH] = [
    [
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
    ],
    [
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
    ],
    [
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
    ],
    [
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
    ],
    [
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
    ],
    [
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
    ],
    [
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
    ],
    [
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
    ],
    [
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
    ],
    [
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
        Felt::new(8),
    ],
    [
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
        Felt::new(23),
    ],
    [
        Felt::new(23),
        Felt::new(8),
        Felt::new(26),
        Felt::new(13),
        Felt::new(10),
        Felt::new(9),
        Felt::new(7),
        Felt::new(6),
        Felt::new(22),
        Felt::new(21),
        Felt::new(8),
        Felt::new(7),
    ],
];

const STATE_WIDTH:usize = 12;
pub fn add_constants<E: FieldElement + From<Felt>>(state: &mut [E], ark: &[E])  {

 
    for i in 0..STATE_WIDTH {
        state[i] += ark[i];
    }
 
}



#[inline(always)]
fn apply_sbox<E: FieldElement + From<Felt>>(state: &mut [E; STATE_WIDTH]) {
    state.iter_mut().for_each(|v| {
        let t2 = v.square();
        let t4 = t2.square();
        *v *= t2 * t4;
    });
}

#[inline(always)]
fn apply_mds<E: FieldElement + From<Felt>>(state: &mut [E; STATE_WIDTH]) {
    let mut result = [E::ZERO; STATE_WIDTH];
    result.iter_mut().zip(MDS).for_each(|(r, mds_row)| {
        state.iter().zip(mds_row).for_each(|(&s, m)| {
            *r += E::from(m) * s;
        });
    });
    *state = result
}

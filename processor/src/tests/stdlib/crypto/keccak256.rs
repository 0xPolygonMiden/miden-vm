use super::build_test;
use crate::Felt;

/// Equivalent to https://github.com/itzmeanjan/merklize-sha/blob/1d35aae/include/test_bit_interleaving.hpp#L12-L34
#[test]
fn keccak256_bit_interleaving() {
    let source = "
    use.std::crypto::hashes::keccak256

    begin
        exec.keccak256::to_bit_interleaved
        exec.keccak256::from_bit_interleaved
    end
    ";

    let word = rand_utils::rand_value::<u64>();

    let high = (word >> 32) as u32 as u64;
    let low = word as u32 as u64;

    let test = build_test!(source, &[low, high]);
    let stack = test.get_last_stack_state();

    assert_eq!(stack[0], Felt::new(high));
    assert_eq!(stack[1], Felt::new(low));
}

#[test]
fn keccak256_2_to_1_hash() {
    let source = "
    use.std::crypto::hashes::keccak256

    begin
        exec.keccak256::keccak_p
    end
    ";

    let mut state = [
        1959948167, 4294028507, 3144626199, 1914386326, 2198120490, 1730042467, 1330963950,
        1668744720, 2095810235, 8798881, 2397528473, 2934427705, 635863215, 2589089733, 3937798160,
        252470082, 1721165114, 709384051, 3485882851, 1618038182, 3324831186, 3333493765,
        4226858030, 2288039584, 2371195348, 3478518845, 230568340, 154379440, 2248836976,
        2511544685, 2194117658, 212124028, 777448701, 3321501499, 2834939093, 1409224893,
        1314715970, 3759625198, 3624601661, 1724129065, 3938562828, 1240708426, 2918554202,
        1576977968, 1421943364, 2812408036, 3494754730, 1773978049, 196387936, 3800496843,
    ];
    keccak_p(&mut state);

    let test = build_test!(source);
    test.expect_stack(&state[0..16]);
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/utils.hpp#L31-L41
#[inline]
fn rotl(x: u32, n: usize) -> u32 {
    if n == 0 {
        x
    } else {
        (x << n) | (x >> (32 - n))
    }
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L55-L98
fn theta(state: &mut [u64]) {
    let mut c = [0u32; 10];
    let mut d = [0u32; 10];

    for i in 0..10 {
        c[i] = state[i] as u32
            ^ state[i + 10] as u32
            ^ state[i + 20] as u32
            ^ state[i + 30] as u32
            ^ state[i + 40] as u32;
    }

    for i in 0..5 {
        let p_idx = ((i + 4) % 5) << 1;
        let n_idx = ((i + 1) % 5) << 1;
        let c_idx = i << 1;

        d[c_idx + 0] = c[p_idx + 0] ^ rotl(c[n_idx + 1], 1);
        d[c_idx + 1] = c[p_idx + 1] ^ c[n_idx + 0];
    }

    for i in 0..50 {
        state[i] = (state[i] as u32 ^ d[i % 10]) as u64;
    }
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L115-L147
fn rho(state: &mut [u64]) {
    let rot = [
        1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
    ];

    for i in 1..25 {
        let c_idx = i << 1;
        let offset = rot[i - 1];
        // read section 2.1 of
        // https://keccak.team/files/Keccak-implementation-3.2.pdf
        //
        if (offset & 0b1) == 0 {
            // even
            state[c_idx + 0] = rotl(state[c_idx + 0] as u32, offset >> 1) as u64;
            state[c_idx + 1] = rotl(state[c_idx + 1] as u32, offset >> 1) as u64;
        } else {
            // odd
            let even = rotl(state[c_idx + 1] as u32, (offset >> 1) + 1) as u64;
            let odd = rotl(state[c_idx + 0] as u32, offset >> 1) as u64;

            state[c_idx + 0] = even;
            state[c_idx + 1] = odd;
        }
    }
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L169-L207
fn pi(state: &mut [u64]) {
    let mut tmp = [0u64; 50];
    for i in 0..50 {
        tmp[i] = state[i];
    }

    for y in 0..5 {
        for x in 0..5 {
            let to_idx = (y * 5 + x) << 1;
            let frm_idx = (5 * x + (x + 3 * y) % 5) << 1;

            state[to_idx + 0] = tmp[frm_idx + 0];
            state[to_idx + 1] = tmp[frm_idx + 1];
        }
    }
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L233-L271
fn chi(state: &mut [u64]) {
    let mut c = [0u32; 10];

    for y in 0..5 {
        for x in 0..5 {
            let x_0 = (y * 5 + (x + 1) % 5) << 1;
            let x_1 = (y * 5 + (x + 2) % 5) << 1;

            let rhs_0 = !(state[x_0 + 0] as u32) & state[x_1 + 0] as u32;
            let rhs_1 = !(state[x_0 + 1] as u32) & state[x_1 + 1] as u32;

            c[(x << 1) + 0] = rhs_0;
            c[(x << 1) + 1] = rhs_1;
        }

        for x in 0..5 {
            let idx = (y * 5 + x) << 1;

            state[idx + 0] = (state[idx + 0] as u32 ^ c[(x << 1) + 0]) as u64;
            state[idx + 1] = (state[idx + 1] as u32 ^ c[(x << 1) + 1]) as u64;
        }
    }
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L288-L306
#[inline]
fn iota<const EVEN: u32, const ODD: u32>(state: &mut [u64]) {
    state[0] = (state[0] as u32 ^ EVEN) as u64;
    state[1] = (state[1] as u32 ^ ODD) as u64;
}

/// https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L325-L342
#[inline]
fn rnd<const EVEN: u32, const ODD: u32>(state: &mut [u64]) {
    theta(state);
    rho(state);
    pi(state);
    chi(state);
    iota::<EVEN, ODD>(state);
}

/// See https://github.com/itzmeanjan/merklize-sha/blob/1d35aae9da7fed20127489f362b4bc93242a516c/include/sha3.hpp#L379-L427
fn keccak_p(state: &mut [u64]) {
    rnd::<1u32, 0u32>(state);
    rnd::<0u32, 137u32>(state);
    rnd::<0u32, 2147483787u32>(state);
    rnd::<0u32, 2147516544u32>(state);
    rnd::<1u32, 139u32>(state);
    rnd::<1u32, 32768u32>(state);
    rnd::<1u32, 2147516552u32>(state);
    rnd::<1u32, 2147483778u32>(state);
    rnd::<0u32, 11u32>(state);
    rnd::<0u32, 10u32>(state);
    rnd::<1u32, 32898u32>(state);
    rnd::<0u32, 32771u32>(state);
    rnd::<1u32, 32907u32>(state);
    rnd::<1u32, 2147483659u32>(state);
    rnd::<1u32, 2147483786u32>(state);
    rnd::<1u32, 2147483777u32>(state);
    rnd::<0u32, 2147483777u32>(state);
    rnd::<0u32, 2147483656u32>(state);
    rnd::<0u32, 131u32>(state);
    rnd::<0u32, 2147516419u32>(state);
    rnd::<1u32, 2147516552u32>(state);
    rnd::<0u32, 2147483784u32>(state);
    rnd::<1u32, 32768u32>(state);
    rnd::<0u32, 2147516546u32>(state);
}

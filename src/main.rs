// encode BCH(15, 11)
fn bch_encode(input: u16) -> u16 {
    assert!(input < 1 << 11);
    let divider = 0b10011;

    let mut dividend = input << 4;

    for i in (4..15).rev() {
        if (dividend & (1 << i)) != 0 {
            // calculate xor with 0b10011
            let mask = 0b11111 << (i - 4);
            let value = (dividend & mask) >> (i - 4);
            let result = value ^ divider;

            dividend &= !mask; // clear the area of interest
            dividend |= result << (i - 4); // overwrite with the result
        }
    }

    // merge code and check bits
    (input << 4) | dividend
}

// get BCH(15,11) syndrome
fn bch_decode(input: u16) -> u16 {
    assert!(input < 1 << 15);
    let mut r4 = 0;
    let mut r3 = 0;
    let mut r2 = 0;
    let mut r1 = 0;

    for i in (0..15).rev() {
        let in_bit = (input >> i) & 1;
        let r4_next = in_bit ^ r1;
        let r3_next = r1 ^ r4;
        let r2_next = r3;
        let r1_next = r2;
        r4 = r4_next;
        r3 = r3_next;
        r2 = r2_next;
        r1 = r1_next;
    }

    (r4 << 3) & (r3 << 2) & (r2 << 1) & r1
}

// get hamming distance between in1 and in2
fn hamming_dist(in1: u16, in2: u16) -> u8 {
    let mut ret = 0;
    for i in 0..16 {
        let mask = 1 << i;
        if (in1 & mask) != (in2 & mask) {
            ret += 1;
        }
    }
    ret
}

fn main() {
    // generate codes
    let codes: Vec<_> = (0b0000_0000_000..=0b1111_1111_111)
        .map(bch_encode)
        .collect();

    let mut dists = std::collections::BTreeMap::new();
    for i in 0..codes.len() {
        for j in (i + 1)..codes.len() {
            let dist = hamming_dist(codes[i], codes[j]);
            if dists.contains_key(&dist) {
                let handle = dists.get_mut(&dist).unwrap();
                *handle += 1;
            } else {
                dists.insert(dist, 1);
            }
        }
    }

    for (k, v) in dists {
        println!("{} {}", k, v);
    }
}

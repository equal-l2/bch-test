// 長さlenで全てのビットが立ったビット列を生成する
fn gen_mask(len: u8) -> u16 {
    let mut ret = 0;
    for _ in 0..len {
        ret = (ret << 1) + 1;
    }
    ret
}

// GF(2)上での剰余
fn gf2_mod(dividend: u16, divisor: u16, divisor_digits: u8) -> u16 {
    let digits = divisor_digits - 1;
    let mut dividend = dividend;
    let mask_arch = gen_mask(divisor_digits);

    for i in (digits..16).rev() {
        // 上から1ビットずつずらしながら見ていく
        if (dividend & (1 << i)) != 0 { // 最上位ビットが立っていれば
            let i_pad = i - digits;
            // 今見ている値と割る数のXORをとる
            let mask = mask_arch << i_pad;
            let value = (dividend & mask) >> i_pad;
            let result = value ^ divisor;

            dividend &= !mask; // 今見ているところをクリアする
            dividend |= result << i_pad; // 計算で得られた余りで上書き
        }
    }
    dividend
}

// 情報ビット列を受け取り、BCH(15, 11)にエンコードする
fn bch_encode(input: u16, gen: u16, gen_len: u8) -> u16 {
    let shifted = input << (gen_len - 1); // 生成多項式分シフトしておく
    let checksum = gf2_mod(shifted, gen, gen_len); // 生成多項式から得られるチェックサム

    shifted ^ checksum
}

// 符号ビット列からBCH(15, 11)のシンドロームを得る
fn bch_decode(input: u16, gen: u16, gen_len:u8) -> u16 {
    gf2_mod(input, gen, gen_len)
}

// in1とin2のハミング距離を計算する
fn hamming_dist(in1: u16, in2: u16) -> u8 {
    let mut ret = 0;
    for i in 0..16 { // 各ビットごとに比較
        let mask = 1 << i;
        if (in1 & mask) != (in2 & mask) {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let gen = 0b10011; // 生成多項式
    let gen_len = 5; // 生成多項式のビット数

    // 全情報ビットパターンについて符号を生成
    let mut codes = Vec::with_capacity(1<<11);
    for i in 0b00000_00000_0..=0b11111_11111_1 {
        let code = bch_encode(i, gen, gen_len); // 符号生成
        assert_eq!(bch_decode(code, gen, gen_len), 0); // シンドロームが0か確認
        codes.push(code); // codesに格納
    }

    // 相異なる符号間のハミング距離を計算
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

    // 計算結果を出力
    for (k, v) in dists {
        println!("{} : {}", k, v);
    }
}

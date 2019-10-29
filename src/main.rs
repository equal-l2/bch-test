use std::io::Write;

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
        if (dividend & (1 << i)) != 0 {
            // 最上位ビットが立っていれば
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

// 情報ビット列を符号化する
fn bch_encode(input: u16, gen: u16, gen_len: u8) -> u16 {
    let shifted = input << (gen_len - 1); // チェックサムの分シフトしておく
    let checksum = gf2_mod(shifted, gen, gen_len); // 生成多項式から得られるチェックサム

    shifted ^ checksum
}

// 符号ビット列からシンドロームを得る
fn bch_decode(input: u16, gen: u16, gen_len: u8) -> u16 {
    gf2_mod(input, gen, gen_len)
}

// in1とin2のハミング距離を計算する
fn hamming_dist(in1: u16, in2: u16) -> u8 {
    let mut ret = 0;
    for i in 0..16 {
        // 各ビットごとに比較
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
    let code_len = 15; // 符号長
    let errors = 1; // 誤り訂正可能数

    let data_len = code_len - gen_len + 1;

    // 全情報ビットパターンについて符号を生成
    let mut codes = Vec::with_capacity(1 << data_len);
    for i in 0..(1 << data_len) {
        let code = bch_encode(i, gen, gen_len); // 符号生成
        assert_eq!(bch_decode(code, gen, gen_len), 0); // シンドロームが0か確認
        codes.push(code); // codesに格納
    }

    // 符号をファイルに保存
    {
        let mut f = std::fs::File::create("codes.txt").unwrap();
        for i in codes.iter() {
            writeln!(f, "{:0w$b}", i, w = code_len as usize).unwrap();
        }
    }

    // 相異なる符号間のハミング距離の分布を調べる
    let mut dists = std::collections::BTreeMap::new();
    for i in 0..codes.len() {
        for j in (i + 1)..codes.len() {
            let dist = hamming_dist(codes[i], codes[j]);
            *dists.entry(dist).or_insert(0) += 1;
        }
    }

    // ハミング距離の分布を出力
    for (k, v) in dists.iter() {
        println!("{} : {}", k, v);
    }

    // シンドロームを生成
    let mut syndromes = Vec::new();
    for i in 0..(1 << code_len) {
        if hamming_dist(0, i) <= errors {
            syndromes.push((i, bch_decode(i, gen, gen_len)));
        }
    }

    // シンドロームをファイルに保存
    {
        let mut f = std::fs::File::create("syndromes.txt").unwrap();
        for (k, v) in syndromes {
            writeln!(
                f,
                "{:0w1$b} : {:0w2$b}",
                k,
                v,
                w1 = code_len as usize,
                w2 = gen_len as usize - 1
            )
            .unwrap();
        }
    }
}

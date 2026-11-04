//! Marginal cost of the work a protocol translator adds, with std only. These are the translator's own operations, run on
//! synthetic data: they say what a translator adds per packet, not how fast Horizon's own decode is.
use std::hint::black_box;
use std::time::Instant;

/// Packs `values` of `bits` bits into longs, as the protocol's paletted containers do (no value spans two longs).
fn pack(values: &[u32], bits: usize) -> Vec<u64> {
    let per = 64 / bits;
    let mut out = vec![0u64; values.len().div_ceil(per)];
    for (i, v) in values.iter().enumerate() {
        out[i / per] |= u64::from(*v) << ((i % per) * bits);
    }
    out
}

fn remap_packed(data: &mut [u64], bits: usize, count: usize, table: &[u32]) {
    let per = 64 / bits;
    let mask = (1u64 << bits) - 1;
    for i in 0..count {
        let (word, shift) = (i / per, (i % per) * bits);
        let old = ((data[word] >> shift) & mask) as usize;
        let new = u64::from(table[old]);
        data[word] = (data[word] & !(mask << shift)) | (new << shift);
    }
}

fn time<F: FnMut()>(label: &str, per: &str, iterations: u32, mut f: F) {
    for _ in 0..(iterations / 10).max(1) {
        f();
    }
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let ns = start.elapsed().as_nanos() as f64 / f64::from(iterations);
    println!("| {label} | {ns:.0} ns per {per} |");
}

fn main() {
    println!("| Operation | Cost |\n|---|---|");

    // 1. The common case: look a packet id up in a table and pass the frame on untouched.
    let table: Vec<i32> = (0..256).map(|i| (i * 7 + 3) % 256).collect();
    // Each lookup depends on the last, so the compiler cannot fold the loop away.
    let mut sink = 97usize;
    time("id lookup + forward unchanged (Frame::Same)", "packet", 20_000_000, || {
        sink = table[black_box(sink)] as usize;
    });

    // 2. A chunk section whose container is indirect: only the palette (at most 256 entries) is remapped.
    let block_table: Vec<u32> = (0..32_768u32).map(|i| if i > 120 { i + 1 } else { i }).collect();
    let mut palette: Vec<u32> = (0..64u32).map(|i| i * 400).collect();
    time("indirect section: remap a 64-entry palette", "section", 5_000_000, || {
        for p in palette.iter_mut() {
            *p = block_table[(*p as usize) % block_table.len()];
        }
        black_box(&palette);
    });

    // 3. A section whose container is direct (global ids, 15 bits): every one of the 4096 entries is remapped.
    let values: Vec<u32> = (0..4096u32).map(|i| (i * 31) % 29_873).collect();
    let mut packed = pack(&values, 15);
    let direct = block_table.clone();
    time("direct section: remap 4096 packed 15-bit entries", "section", 200_000, || {
        remap_packed(&mut packed, 15, 4096, &direct);
        black_box(&packed);
    });
    time("direct chunk: 24 sections of 4096 entries", "chunk", 20_000, || {
        for _ in 0..24 {
            remap_packed(&mut packed, 15, 4096, &direct);
        }
        black_box(&packed);
    });

    // 4. Items in a container click: a handful of ids per packet.
    let item_table: Vec<u32> = (0..2000u32).map(|i| if i > 400 { i + 1 } else { i }).collect();
    let mut stacks: Vec<u32> = (0..46u32).map(|i| i * 40).collect();
    time("window items: remap 46 stacks", "packet", 5_000_000, || {
        for s in stacks.iter_mut() {
            *s = item_table[(*s as usize) % item_table.len()];
        }
        black_box(&stacks);
    });
    black_box(sink);
}

use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn adjust_ms(ms: u32) -> u32 {
    // the calculator's clock is slightly behind,
    // making notes near the end of the song much
    // later than they should be
    
    let calc_ms_per_real_ms: f64 = 0.995;  // hours upon gruelling hours
    (ms as f64 * calc_ms_per_real_ms) as u32
}

fn get_field(metadata_line: &str) -> &str {
    metadata_line.splitn(3, ' ').nth(2).expect("empty metadata field")
}

fn bpm_to_mspb(bpm: f64) -> f64 {
    1.0 / (bpm / 60_000.0)
}

fn to_ms(time: &str, uses_beats: bool, mspb: f64) -> u32 {
    if uses_beats {
        let mut parts = time.split(':');

        // oops, all 4/4
        let measure: u32 = parts.next().expect("no measure").parse().expect("invalid measure");
        let beat: f64 = parts.next().expect("no beat").parse().expect("invalid beat");
        let beats_into: f64 = (measure * 4) as f64 + beat - 1.0;

        let ms = (beats_into * mspb) as u32;
        return adjust_ms(ms);
    } else {
        return adjust_ms(time.parse().expect("invalid ms"))
    }
}

fn pad_str(s: &str) -> Vec<u8> {
    let mut buf = vec![b' '; 64];

    let bytes = s.as_bytes();
    let length = bytes.len().min(64-1);

    buf[..length].copy_from_slice(&bytes[..length]);
    buf[length] = 0;

    buf
}


// r0, g0, b0 = (int(c) for c in input("r0 g0 b0: ").split())
// r1, g1, b1 = (int(c) for c in input("r1 g1 b1: ").split())
// 
// n = int(input("n: "))
// 
// dr, dg, db = r1-r0, g1-g0, b1-b0
// 
// for i in range(n+1):
//     t = i/n
//     print(f"{r0 + t*dr} {g0 + t*dg} {b0 + t*db}")
// 
fn interpolate_colour(n: u16, c0: (u16, u16, u16), c1: (u16, u16, u16)) -> Vec<(u16, u16, u16)> {
    let r0 = c0.0 as f32;
    let g0 = c0.1 as f32;
    let b0 = c0.2 as f32;

    let dr = c1.0 as f32 - r0;
    let dg = c1.1 as f32 - g0;
    let db = c1.2 as f32 - b0;

    let mut colours = Vec::new();
    for i in 1..n+1 {
        let t = i as f32 / n as f32;
        colours.push((
            (r0 + t * dr) as u16,
            (g0 + t * dg) as u16,
            (b0 + t * db) as u16
        ));
    }

    colours
}

pub fn compile(input: PathBuf, output: PathBuf) {
    let txt = fs::read_to_string(input).unwrap();
    let mut bin = fs::File::create(output).unwrap();

    let lines: Vec<&str> = txt.lines().collect();

    let title = get_field(lines[0]);
    let artist = get_field(lines[1]);
    let id = get_field(lines[2]);
    let bpm_field = get_field(lines[3]);

    bin.write_all(&pad_str(title)).unwrap();
    bin.write_all(&pad_str(artist)).unwrap();
    bin.write_all(&pad_str(id)).unwrap();

    let uses_beats = bpm_field != "ms";
    let mut mspb: f64 = 0.0;
    if uses_beats {
        let bpm: f64 = bpm_field.parse().expect("invalid bpm");
        mspb = bpm_to_mspb(bpm);
    }
    
    let mut i = 4;
    for raw_line in &lines[4..] {
        let line = raw_line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue
        }

        let mut parts = line.split_whitespace();
        let class = parts.next().expect("no item class");
        let ms = to_ms(parts.next().expect("no item time"), uses_beats, mspb);
        
        match class {
            "t" => {
                let x: f32 = parts.next().expect("no note x").parse().expect("invalid note x");

                bin.write_all(b"t").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&x.to_le_bytes()).unwrap();
                bin.write_all(b"    ").unwrap();    // padding?

            },
            "h" => {
                let x: f32 = parts.next().expect("no note x").parse().expect("invalid note x");
                let ms_end = to_ms(parts.next().expect("no hold note end time"), uses_beats, mspb);

                bin.write_all(b"h").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&x.to_le_bytes()).unwrap();
                bin.write_all(&ms_end.to_le_bytes()).unwrap();
            },
            "e" => {
                let r: u16 = parts.next().expect("no bg color r").parse().expect("invalid bg color r");
                let g: u16 = parts.next().expect("no bg color g").parse().expect("invalid bg color g");
                let b: u16 = parts.next().expect("no bg color b").parse().expect("invalid bg color b");

                bin.write_all(b"e").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&r.to_le_bytes()).unwrap();
                bin.write_all(&g.to_le_bytes()).unwrap();
                bin.write_all(&b.to_le_bytes()).unwrap();
                bin.write_all(b"  ").unwrap();      // padding
            },
            "e-fade" => {
                let r0: u16 = parts.next().expect("no fade color r0").parse().expect("invalid fade color r0");
                let g0: u16 = parts.next().expect("no fade color g0").parse().expect("invalid fade color g0");
                let b0: u16 = parts.next().expect("no fade color b0").parse().expect("invalid fade color b0");

                let r1: u16 = parts.next().expect("no fade color r1").parse().expect("invalid fade color r1");
                let g1: u16 = parts.next().expect("no fade color g1").parse().expect("invalid fade color g1");
                let b1: u16 = parts.next().expect("no fade color b1").parse().expect("invalid fade color b1");

                let n: u16 = parts.next().expect("no fade n").parse().expect("invalid fade n");
                let ms_end = to_ms(parts.next().expect("no fade end time"), uses_beats, mspb);

                let dms = ((ms_end - ms) as f32 / n as f32) as u32;
                let mut ms_i = ms + dms;
                for (r, g, b) in interpolate_colour(n, (r0, g0, b0), (r1, g1, b1)) {
                    bin.write_all(b"e").unwrap();
                    bin.write_all(&ms_i.to_le_bytes()).unwrap();
                    bin.write_all(&r.to_le_bytes()).unwrap();
                    bin.write_all(&g.to_le_bytes()).unwrap();
                    bin.write_all(&b.to_le_bytes()).unwrap();
                    bin.write_all(b"  ").unwrap();      // padding
                    
                    ms_i += dms;
                }
            },
            other => panic!("/!\\ line {}: unsupported note type '{}'", i, other)
        };

        i += 1;
    }
}


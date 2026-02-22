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

pub fn convert_map(input: PathBuf, output: PathBuf) {
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
                bin.write_all(b"mtn ").unwrap();    // padding?

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
            }
            _ => panic!()
        };
    }
}


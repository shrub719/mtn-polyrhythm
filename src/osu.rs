use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn get_field(metadata_line: &str) -> &str {
    metadata_line.split(':').nth(1).unwrap()
}

pub fn osu(input: PathBuf, output: PathBuf) {
    let osu = fs::read_to_string(input).unwrap();
    let mut txt = fs::File::create(output).unwrap();

    let lines: Vec<&str> = osu.lines().collect();

    let mut i = match lines.iter().position(|l| l == &"[Metadata]") {
        Some(index) => index + 1,
        None => panic!("no Metadata section in file")
    };

    let title = get_field(lines[i+1]);
    let artist = get_field(lines[i+3]);
    let id = format!("osu_{}", get_field(lines[i+8]));

    write!(
        &mut txt, "// title: {}\n// artist: {}\n// id: {}\n// bpm: ms\n",
        title, artist, id
    ).unwrap();

    i = match lines.iter().position(|l| l == &"[HitObjects]") {
        Some(index) => index + 1,
        None => panic!("no HitObjects section in file")
    };

    while i < lines.len() {
        let line = lines[i].trim();
        let mut parts = line.split(',');
        
        let osu_x: f32 = parts.next().unwrap().parse().unwrap();
        let _osu_y = parts.next().unwrap();
        let ms: u32 = parts.next().unwrap().parse().unwrap();
        let type_flags: u16 = parts.next().unwrap().parse().unwrap();
        let _ = parts.next().unwrap();
        let ms_end: u32 = parts.next().unwrap().split(':').nth(0).unwrap().parse().unwrap();

        let x: f32 = osu_x / 512.0;
        assert!(x < 1.0);

        match type_flags {
            1 | 5 => write!(&mut txt, "t {} {}\n", ms, x),
            128 | 132 => write!(&mut txt, "h {} {} {}\n", ms, x, ms_end),
            _ => panic!("/!\\ line {}: unsupported note type with flags {}", i, type_flags)
        }.unwrap();

        i += 1;
    }
}

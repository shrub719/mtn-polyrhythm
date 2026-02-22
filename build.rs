fn adjust_ms(ms: u32) -> u32 {
    // the calculator's clock is slightly behind,
    // making notes near the end of the song much
    // later than they should be
    
    let calc_ms_per_real_ms: f64 = 0.995;  // hours upon gruelling hours
    (ms as f64 * calc_ms_per_real_ms) as u32
}

fn convert_map(input: &str, output: &str) {
    let txt = fs::read_to_string(input).unwrap();
    let mut bin = fs::File::create(output).unwrap();
    
    for line in txt.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue
        }

        let mut parts = line.split_whitespace();
        let class = parts.next().unwrap();
        let ms: u32 = adjust_ms(parts.next().unwrap().parse().unwrap());
        let x: f32 = parts.next().unwrap().parse().unwrap();
        
        match class {
            "t" => {
                bin.write_all(b"t").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&x.to_le_bytes()).unwrap();
                bin.write_all(b"pibi").unwrap();

            },
            "h" => {
                let ms_end: u32 = adjust_ms(parts.next().unwrap().parse().unwrap());

                bin.write_all(b"h").unwrap();
                bin.write_all(&ms.to_le_bytes()).unwrap();
                bin.write_all(&x.to_le_bytes()).unwrap();
                bin.write_all(&ms_end.to_le_bytes()).unwrap();
            },
            _ => ()
        };
    }
}

fn convert_maps() {
    let out_dir = "target/maps";
    let in_dir = "assets/maps";
    
    fs::create_dir_all(out_dir).unwrap();

    for entry in fs::read_dir(in_dir).unwrap() {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) == Some("mtn") {
            println!("cargo:rerun-if-changed={}", path.display());

            let filename = path.file_stem().unwrap().to_str().unwrap();
            let output = Path::new(&out_dir).join(format!("{filename}.mtb"));

            convert_map(path.to_str().unwrap(), output.to_str().unwrap());
        }
    }
}


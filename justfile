current_target := `rustc -Vv | grep host | awk '{print $2}'`

[default]
dev:
    cargo build

build:
    cargo build --release

test path:
    cargo run -- compile {{path}} -o ./test/output.mtb
    ./test/epsilon.bin --nwb ./test/libmetronome.so --nwb-external-data ./test/output.mtb


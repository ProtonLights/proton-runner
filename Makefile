all:
	@cargo build
copy:
	@cp ../proton-cli/target/debug/proton ./proton_cli
update:
	@cp ../proton-cli/target/debug/proton ./proton_cli
	@cargo run update-data GreatNorthern
run:
	@cargo run run-show GreatNorthern "/dev/ttyUSB0"
run1:
	@cargo run run-show GreatNorthern "/dev/ttyUSB1"
test:
	@cargo test
on:
	@cargo run allon "/dev/ttyUSB0"
off:
	@cargo run alloff "/dev/ttyUSB0"


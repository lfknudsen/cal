all:
	cargo build -r
	mv target/release/cal.exe .
	install
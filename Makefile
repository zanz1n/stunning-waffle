upload:
	~/.platformio/penv/bin/platformio run --target upload

build:
	pnpm tauri build || true
	cp ./src-tauri/target/release/stunning-waffle ./bin/app.bin

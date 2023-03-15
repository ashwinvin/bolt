.PHONY: setup all build-yew build-tauri watch-yew watch-tauri

setup:
	cargo install tauri-cli
	cargo install trunk

all: build-yew build-tauri
	cp -r ./tauri/target/release/bundle ./target

build-yew:
	cd yew && trunk build -d ../tauri/dist

build-tauri:
	cd tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

watch-yew:
	cd yew && trunk watch -d ../tauri/dist
	
web:
	cd yew && trunk serve -d ../tauri/dist --port 3000 --open
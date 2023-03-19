.PHONY: run setup all build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean

setup:
	cargo install tauri-cli
	cargo install trunk

all: build-yew build-tauri
	cp -r ./tauri/target/release/bundle ./target

run: build-yew watch-tauri

build-yew:
	cd yew && trunk build -d ../tauri/dist
	cd yew && cp ./script.js ../tauri/dist 

build-tauri:
	cd tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

watch-yew:
	cd yew && trunk watch -d ../tauri/dist
	
web: build-yew
	cd ./tauri/dist && http-server -p 3000

clean-yew:
	cd yew && cargo clean

clean-tauri:
	cd tauri && cargo clean

clean: clean-yew clean-tauri
	
run: generate bootstrap
	flutter run

build-api:
	cd native && cargo build

bootstrap:
	flutter pub get

generate:
	CPATH=/usr/lib/clang/15.0.7/include flutter_rust_bridge_codegen --class-name Native --no-build-runner --dart-output lib/bridge_generated.dart --rust-input native/src/api.rs -c ios/Runner/bridge_generated.h --rust-crate-dir native
	flutter pub run build_runner build --delete-conflicting-outputs

setup_codegen:
	cargo install cbindgen
	cargo install flutter_rust_bridge_codegen
	cargo install cargo-ndk
	dart pub global activate ffigen

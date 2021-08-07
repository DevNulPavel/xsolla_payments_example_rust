.SILENT:
.PHONY:

START_TUNNEL_TO_LOCALHOST:
	ngrok http localhost:8080

START_SERVER:
	cargo clippy && \
	cargo build && \
	./target/debug/xsolla_payments_example_rust
.SILENT:
.PHONY:

ENCRYPT_TEST_ENV:
	gpg -a -r 0x0BD10E4E6E578FB6 -o test_env/config.env.asc -e test_env/config.env

DECRYPT_TEST_ENV:
	rm -rf test_env/config.env
	gpg -a -r 0x0BD10E4E6E578FB6 -o test_env/config.env -d test_env/config.env.asc

START_TUNNEL_TO_LOCALHOST_1:
	ngrok http localhost:8080

START_TUNNEL_TO_LOCALHOST_2:
	tunnelto --port 8080

START_SERVER:
	cargo clippy && \
	cargo build && \
	./target/debug/xsolla_payments_example_rust
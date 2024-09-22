.PHONY: all clean run-% dev-setup

all:
	@echo "Available targets:"
	@echo "stress 🌩  --- run the stress test"
	@echo "dev-setup ❄ --- setup the tools needed"
	@echo "simple_server.bin | non_blocking_server.bin | multiplexed_server  🖥  --- compile one of the servers"
	@echo "run-simple_server.bin | run-non_blocking_server.bin | run-multiplexed_server  🖥  --- run one of the servers"
	@echo "clean 🧹 --- clean build artifacts"

dev-setup:
	nix develop

stress:
	elixir test.exs

%: %.bin

%.bin: target/debug/%
	cp $< $@

target/debug/%:
	cargo build --bin $*

run-%: %.bin
	./$

clean:
	cargo clean
	rm -f *.bin

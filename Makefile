.PHONY: all clean run-%

all:
	@echo "Please specify a target to build, e.g., 'make simple_server'"

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

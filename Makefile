clippy:
	cargo clippy

TARGET = target/debug/jlox
.PHONY: $(TARGET)
$(TARGET):
	cargo build

single_test:
	cargo run ../../clone/craftinginterpreters/test/$(ARGS)

INTERPRETER = ../../Projects/lox/target/debug/jlox
test: target/debug/jlox
	cd ../../clone/craftinginterpreters; \
	dart tool/bin/test.dart \
		chap10_functions --interpreter $(INTERPRETER) \
		| sed -e 's/\x1b\[[0-9;]*m//g'

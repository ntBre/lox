clippy:
	cargo clippy

TARGET = target/debug/jlox
.PHONY: $(TARGET)
$(TARGET):
	cargo build

TEST_DIR = ../../clone/craftinginterpreters/test
single_test:
	RUSTFLAGS='--cfg testing' cargo run  $(TEST_DIR)/$(ARGS)

CHAPTER=
CH = 10
ifeq ($(CH),10)
	CHAPTER=chap10_functions
else ifeq ($(CH),11)
	CHAPTER=chap11_resolving
else ifeq ($(CH),17)
	CHAPTER=chap17_compiling
endif

INTERPRETER = ../../Projects/lox/target/debug/jlox
test: target/debug/jlox
	cd ../../clone/craftinginterpreters; \
	dart tool/bin/test.dart \
		$(CHAPTER) --interpreter $(INTERPRETER) \
		| sed -e 's/\x1b\[[0-9;]*m//g'

.PHONY: clox
clox:
	cargo run --bin clox

INTERPRETER = ../../Projects/lox/target/debug/clox
ctest: target/debug/clox
	cd ../../clone/craftinginterpreters; \
	dart tool/bin/test.dart \
		$(CHAPTER) --interpreter $(INTERPRETER) \
		| sed -e 's/\x1b\[[0-9;]*m//g'

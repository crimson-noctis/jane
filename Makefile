all: main
	
jane.h: src/ffi.rs
	cbindgen --config cbindgen.toml --crate jane --output jane.h

main: main.c jane.h
	clang -Wall -Wextra main.c -L./target/debug -ljane -lpthread -ldl -o main 

use std::io::{Result, Write};

use parity_wasm::{deserialize_file, elements::Module};

fn load_module(name: &str) -> Module {
	deserialize_file(name)
		.expect("Failed to parse WebAssembly file")
		.parse_names()
		.unwrap_or_else(|v| v.1)
}

fn do_runtime(lock: &mut dyn Write) -> Result<()> {
	let runtime = codegen_luau::RUNTIME;
	let numeric = codegen_luau::NUMERIC;

	writeln!(lock, "local rt = (function()")?;
	writeln!(lock, "local I64 = (function()")?;
	writeln!(lock, "{numeric}")?;
	writeln!(lock, "end)()")?;
	writeln!(lock, "{runtime}")?;
	writeln!(lock, "end)()")
}

fn main() -> Result<()> {
	let wasm = match std::env::args().nth(1) {
		Some(name) => load_module(&name),
		None => {
			eprintln!("usage: wasm2luau <file>");

			return Ok(());
		}
	};

	let lock = &mut std::io::stdout().lock();

	do_runtime(lock)?;
	codegen_luau::from_module_untyped(&wasm, lock)
}

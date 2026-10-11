use std::path::PathBuf;

use nassau::codegen::{Codegen, CodegenOptions};
use nassau::{infer, lower, prelude::Basis};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let target = arguments.next().expect("target triple");
    let output = PathBuf::from(arguments.next().expect("output directory"));
    let basis = Basis::check(&mut infer::Session::new());
    let module = basis
        .lower(&mut lower::Session::new_repl(), "nassau_basis")
        .expect("lower the Basis");
    let (object, bindings) = Codegen::new(CodegenOptions::default())
        .compile_basis(&module, &target)
        .expect("compile the Basis");
    std::fs::write(output.join("basis.o"), object).expect("write the Basis object");
    std::fs::write(output.join("basis_symbols.rs"), bindings).expect("write Basis bindings");
}

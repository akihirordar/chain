#[cfg(target_env = "sgx")]
mod sgx_module;

#[cfg(target_env = "sgx")]
fn main() -> std::io::Result<()> {
    sgx_module::entry()
}

#[cfg(not(target_env = "sgx"))]
fn main() {
    println!("`tx-query-next` cannot be compiled for non-sgx environment!");
}

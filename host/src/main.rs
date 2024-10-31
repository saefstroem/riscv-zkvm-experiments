use std::fs::File;
use std::io::Read;
use risc0_zkvm::{compute_image_id, default_prover, ExecutorEnv, Receipt};
use sha2::{Sha256, Digest};

fn load_elf(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open ELF file");
    let mut elf_data = Vec::new();
    file.read_to_end(&mut elf_data).expect("Failed to read ELF file");
    elf_data
}


fn prove_custom_elf(
    elf_path: &str, 
    input: Option<Vec<u8>>
) -> Result<Receipt, Box<dyn std::error::Error>> {
    // Load the ELF file
    let elf_data = load_elf(elf_path);
    
    // Compute the image ID
    let image_id =compute_image_id(&elf_data).unwrap();

    // Create the executor environment
    let env = ExecutorEnv::builder()
        .build()?;

    // Get the default prover
    let prover = default_prover();
    
    // Prove the execution
    let prove_info = prover.prove(env, &elf_data)?;
    
    // Verify the receipt with computed image ID
    prove_info.receipt.verify(image_id)?;
    
    Ok(prove_info.receipt)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Example usage
    let elf_path = "./riscv-guest";
    
    // Optional: Prepare your input data
    let input_data = vec![1, 2, 3, 4]; // Example input
    
    // Prove the custom ELF
    let receipt = prove_custom_elf(elf_path, Some(input_data))?;
    
    // Access the journal output if needed
    let output: Vec<u8> = receipt.journal.decode()?;
    println!("Proof verification successful! Output: {:?}", output);
    
    Ok(())
}
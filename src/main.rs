use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os().skip(1);
    let (Some(input), Some(output), None) = (args.next(), args.next(), args.next()) else {
        eprintln!("Usage: ngram_token_lemma_tokenizer <input-corpus-dir> <output-corpus-dir>");
        std::process::exit(2);
    };
    ngram_token_lemma_tokenizer::corpus::process_corpus(
        &PathBuf::from(input), &PathBuf::from(output),
    )?;
    Ok(())
}

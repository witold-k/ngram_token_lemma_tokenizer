//! Two-pass corpus transformation: document-local bigrams, then global IDs.
use crate::{bigram, error::{Error, Result}};
use std::{fs, path::{Path, PathBuf}};
use token_db::{TokenDb, TokenId};

pub fn load_token_stream(path: &Path) -> Result<Vec<TokenId>> {
    Ok(postcard::from_bytes(&fs::read(path)?)?)
}
pub fn save_token_stream(path: &Path, tokens: &[TokenId]) -> Result<()> {
    fs::write(path, postcard::to_allocvec(tokens)?)?;
    Ok(())
}

fn global_output(local: &Path) -> Result<PathBuf> {
    let stem = local.file_stem().and_then(|s| s.to_str())
        .ok_or_else(|| Error::InvalidPath(local.to_path_buf()))?;
    Ok(local.with_file_name(format!("{stem}_glob.tok")))
}

/// Transform all documents in `input` with `token_db.tdb` and `*_glob.tok`.
/// Output preserves relative document paths. Each run rebuilds the global DB,
/// so removed documents cannot leave obsolete tokens behind.
pub fn process_corpus(input: &Path, output: &Path) -> Result<()> {
    if !input.is_dir() || !input.join("token_db.tdb").is_file() {
        return Err(Error::InvalidPath(input.to_path_buf()));
    }
    if output.starts_with(input) || input.starts_with(output) {
        return Err(Error::InvalidPath(output.to_path_buf()));
    }
    let source_db = TokenDb::load(input.join("token_db.tdb"))?;
    let mut inputs = Vec::new();
    fsscanner::fsscanner_base::collect_files_fast(input, "tok", &mut inputs);
    inputs.retain(|path| path.file_stem().and_then(|s| s.to_str())
        .is_some_and(|stem| stem.ends_with("_glob")));
    inputs.sort();
    if inputs.is_empty() { return Err(Error::EmptyCorpus); }

    // First pass: transform each source document using the source's global IDs.
    // Each document gets its own bigram DB and local stream.
    let mut global_db = TokenDb::new();
    let mut outputs = Vec::new();
    for source in inputs {
        let relative = source.strip_prefix(input)
            .map_err(|_| Error::InvalidPath(source.clone()))?;
        let stem = relative.file_stem().and_then(|s| s.to_str())
            .ok_or_else(|| Error::InvalidPath(source.clone()))?;
        let base = stem.strip_suffix("_glob")
            .ok_or_else(|| Error::InvalidPath(source.clone()))?;
        let local_path = output.join(relative).with_file_name(format!("{base}.tok"));
        if let Some(parent) = local_path.parent() { fs::create_dir_all(parent)?; }
        let tokens = load_token_stream(&source)?;
        let (local_db, local_tokens) = bigram::transform(&source_db, &tokens)?;
        save_token_stream(&local_path, &local_tokens)?;
        local_db.save(local_path.with_extension("tdb"))?;
        global_db.merge(&local_db)?;
        outputs.push(local_path);
    }
    global_db.save(output.join("token_db.tdb"))?;

    // Second pass: translate each document's local IDs into the global vocabulary.
    for local_path in outputs {
        let local_db = TokenDb::load(local_path.with_extension("tdb"))?;
        let mapping = local_db.iter().map(|(_, entry)| {
            global_db.id(entry.text())
                .ok_or_else(|| Error::MissingGlobalToken(entry.text().to_owned()))
        }).collect::<Result<Vec<_>>>()?;
        let translated = load_token_stream(&local_path)?.into_iter().map(|id| {
            mapping.get(id.get() as usize).copied().ok_or(Error::InvalidTokenId(id))
        }).collect::<Result<Vec<_>>>()?;
        save_token_stream(&global_output(&local_path)?, &translated)?;
    }
    Ok(())
}

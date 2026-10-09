//! Naive overlapping adjacent bigrams; no linguistic filtering or sentence boundaries.
use crate::error::{Error, Result};
use token_db::{TokenDb, TokenId};

/// Encode a pair of source tokens unambiguously using length-prefixed UTF-8.
/// This prevents collisions when tokens contain spaces or punctuation.
pub fn encode_pair(first: &str, second: &str) -> String {
    format!("{}:{first}{}:{second}", first.len(), second.len())
}

/// Create a new local vocabulary and an ordered bigram stream.
/// Input IDs must refer to the supplied source database.
pub fn transform(source: &TokenDb, tokens: &[TokenId]) -> Result<(TokenDb, Vec<TokenId>)> {
    let mut output_db = TokenDb::new();
    let mut output_tokens = Vec::with_capacity(tokens.len().saturating_sub(1));
    for pair in tokens.windows(2) {
        let first = source.get(pair[0]).ok_or(Error::InvalidTokenId(pair[0]))?;
        let second = source.get(pair[1]).ok_or(Error::InvalidTokenId(pair[1]))?;
        let encoded = encode_pair(first.text(), second.text());
        output_tokens.push(output_db.insert(&encoded)?);
    }
    Ok((output_db, output_tokens))
}

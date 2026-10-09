use ngram_token_lemma_tokenizer::bigram::{encode_pair, transform};
use token_db::TokenDb;

#[test]
fn overlapping_pairs_preserve_order_and_frequency() {
    let mut source = TokenDb::new();
    let a = source.insert("a").unwrap();
    let b = source.insert("b").unwrap();
    let (db, tokens) = transform(&source, &[a, b, a, b]).unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], tokens[2]);
    assert_ne!(tokens[0], tokens[1]);
    assert_eq!(db.get(tokens[0]).unwrap().count(), 2);
    assert_eq!(db.len(), 2);
}
#[test]
fn length_prefix_prevents_ambiguous_pairs() {
    assert_ne!(encode_pair("a b", "c"), encode_pair("a", "b c"));
    assert_ne!(encode_pair("a:b", "c"), encode_pair("a", "b:c"));
}
#[test]
fn empty_and_singleton_inputs_yield_no_bigrams() {
    let mut db = TokenDb::new();
    let a = db.insert("a").unwrap();
    assert!(transform(&db, &[]).unwrap().1.is_empty());
    assert!(transform(&db, &[a]).unwrap().1.is_empty());
}

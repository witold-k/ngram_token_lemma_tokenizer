# ngram_token_lemma_tokenizer

First algorithm: **naive overlapping bigrams** over an existing ordered token-ID stream.
No lemmatization happens here; tokens can already represent words or lemmas.

Input directory is a corpus created by `lemmatizer_wrapper` or `pdf_to_text_wrapper`:
`token_db.tdb` plus document-level `*_glob.tok` files. The input global
vocabulary is used to decode the source token IDs.

Run:

```sh
cargo run -- <input-corpus-dir> <output-corpus-dir>
```

Output preserves the relative directory structure:
- `document.tdb` and `document.tok`: document-local bigram vocabulary and IDs.
- `document_glob.tok`: global bigram IDs.
- `token_db.tdb`: merged global bigram vocabulary.

The output `.tok` files use the same postcard serialization of `TokenId`
as the upstream tools. Bigram vocabulary keys are collision-free
length-prefixed UTF-8 pairs, e.g. `3:cat3:dog`.

Two passes: transform documents and merge local databases, then remap local
streams to global IDs. Document boundaries are respected. Rebuilding is
currently unconditional; incremental processing and additional algorithms
(trigrams, POS-aware combinations) are future work.

The input and output directory trees must not contain one another.

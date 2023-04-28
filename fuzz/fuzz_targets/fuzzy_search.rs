#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use simsearch::SimSearch;

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    id: u32,
    text: String,
    query: String,
}

fuzz_target!(|fuzz_input: FuzzInput| {
    let mut engine: SimSearch<u32> = SimSearch::new();

    engine.insert(fuzz_input.id, &fuzz_input.text);

    let _ = engine.search(&fuzz_input.query);
});

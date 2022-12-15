use fasttext::FastText;

const MODEL: &[u8] = include_bytes!("fixtures/cooking.model.bin");

#[test]
fn test_fasttext_load_model() {
    let mut fasttext = FastText::new();
    assert!(fasttext
        .load_model("tests/fixtures/cooking.model.bin")
        .is_ok());
}

#[test]
fn test_fasttext_load_invalid_model() {
    let mut fasttext = FastText::new();
    assert!(fasttext
        .load_model("tests/fixtures/invalid.model.bin")
        .is_err());
}

#[test]
fn test_fasttext_is_quant() {
    let mut fasttext = FastText::new();
    fasttext
        .load_model("tests/fixtures/cooking.model.bin")
        .unwrap();
    assert!(!fasttext.is_quant());
}

#[test]
fn test_fasttext_predict() {
    let mut fasttext = FastText::new();
    fasttext
        .load_model("tests/fixtures/cooking.model.bin")
        .unwrap();
    let preds = fasttext
        .predict("Which baking dish is best to bake a banana bread ?", 2, 0.0)
        .unwrap();
    assert_eq!(2, preds.len());
    assert_eq!("__label__baking", &preds[0].label);
    assert_eq!("__label__bread", &preds[1].label);
}

#[test]
fn test_fasttext_predict_from_bytes() {
    let mut fasttext = FastText::new();
    fasttext
        .load_model_bytes(MODEL)
        .unwrap();
    let preds = fasttext
        .predict("Which baking dish is best to bake a banana bread ?", 2, 0.0)
        .unwrap();
    assert_eq!(2, preds.len());
    assert_eq!("__label__baking", &preds[0].label);
    assert_eq!("__label__bread", &preds[1].label);
}

#[test]
fn test_fasttext_predict_on_words() {
    let mut fasttext = FastText::new();
    fasttext
        .load_model("tests/fixtures/cooking.model.bin")
        .unwrap();
    let words = vec![
        "Which", "baking", "dish", "is", "best", "to", "bake", "a", "banana", "bread", "?",
    ];
    let words_ids: Vec<i32> = words
        .iter()
        .map(|&x| fasttext.get_word_id(x).unwrap() as i32)
        .collect();
    let preds = fasttext.predict_on_words(&words_ids, 2, 0.0).unwrap();
    assert_eq!(2, preds.len());
    assert_eq!("__label__baking", &preds[0].label);
    assert_eq!("__label__bread", &preds[1].label);
}

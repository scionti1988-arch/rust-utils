use std::io::Write;
use tempfile::NamedTempFile;

use table_qa::generate_qa_from_csv;

#[test]
fn generates_qa_for_numeric_columns() {
    let mut temp = NamedTempFile::new().expect("failed to create temp file");

    writeln!(
        temp,
        "month,revenue,visits\nJan,100.0,10\nFeb,200.0,20\nMar,300.0,30"
    )
    .expect("failed to write temp csv");

    let path = temp.path();

    let qa_pairs = generate_qa_from_csv(path).expect("expected Q&A pairs");

    // We expect at least three questions per numeric column (total, average, count)
    // Here we have two numeric columns: revenue and visits.
    assert!(qa_pairs.len() >= 6);

    let questions: Vec<_> = qa_pairs.iter().map(|qa| qa.question.as_str()).collect();

    assert!(questions.iter().any(|q| q.contains("revenue")));
    assert!(questions.iter().any(|q| q.contains("visits")));
}

use csv::StringRecord;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Simple question and answer pair generated from a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionAnswer {
    pub question: String,
    pub answer: String,
}

/// Generate Q&A pairs from a CSV file path.
///
/// The function inspects numeric columns and produces questions like:
/// "What is the total of column X?" and "What is the average of column X?"
pub fn generate_qa_from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<QuestionAnswer>, String> {
    let file = File::open(&path)
        .map_err(|e| format!("Failed to open {}: {e}", path.as_ref().display()))?;

    let mut reader = csv::Reader::from_reader(file);

    let headers = reader
        .headers()
        .map_err(|e| format!("Failed to read headers: {e}"))?
        .clone();

    // We will track sum and count for numeric columns by index.
    #[derive(Debug, Default)]
    struct Stats {
        sum: f64,
        count: u64,
    }

    let mut stats: HashMap<usize, Stats> = HashMap::new();

    for result in reader.records() {
        let record = result.map_err(|e| format!("Failed to read record: {e}"))?;
        accumulate_numeric_fields(&record, &mut stats);
    }

    let mut qa_pairs = Vec::new();

    for (idx, stat) in stats {
        if stat.count == 0 {
            continue;
        }

        let header = headers
            .get(idx)
            .unwrap_or("value")
            .trim()
            .to_string();

        let total = stat.sum;
        let avg = stat.sum / stat.count as f64;

        qa_pairs.push(QuestionAnswer {
            question: format!("What is the total of column \"{}\"?", header),
            answer: format!("{:.2}", total),
        });

        qa_pairs.push(QuestionAnswer {
            question: format!("What is the average of column \"{}\"?", header),
            answer: format!("{:.2}", avg),
        });

        qa_pairs.push(QuestionAnswer {
            question: format!(
                "How many rows contain a numeric value in column \"{}\"?",
                header
            ),
            answer: stat.count.to_string(),
        });
    }

    if qa_pairs.is_empty() {
        return Err("No numeric columns found in the CSV file".to_string());
    }

    Ok(qa_pairs)
}

fn accumulate_numeric_fields(record: &StringRecord, stats: &mut HashMap<usize, Stats>) {
    for (idx, field) in record.iter().enumerate() {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Try to parse as f64. Non numeric values are just ignored.
        if let Ok(value) = trimmed.parse::<f64>() {
            let entry = stats.entry(idx).or_insert_with(Stats::default);
            entry.sum += value;
            entry.count += 1;
        }
    }

    #[derive(Debug, Default)]
    struct Stats {
        sum: f64,
        count: u64,
    }
}

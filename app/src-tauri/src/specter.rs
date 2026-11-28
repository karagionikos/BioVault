//! Spike 0: SPECTER2 (int8 ONNX) fingerprints on ONNX Runtime via `ort` (docs/08-ml.md §1).
//! No Tauri in here: in P1 this moves into biovault-core's `ml` module.
//! tract was tried first: correct (cosine 0.998) but 33 s per abstract on the desktop,
//! because it runs the int8 MatMulInteger ops without optimized kernels.

use std::{path::Path, sync::OnceLock, time::Instant};

use ort::{environment::Environment, session::Session, value::Tensor};
use tokenizers::{Tokenizer, TruncationParams};

pub const MODEL_FILE: &str = "specter2-proximity-int8.onnx";
const MAX_TOKENS: usize = 512;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// ONNX Runtime allows one environment per process.
/// shortcut: two first calls racing would make the second fail; fine for one button, not for P1.
fn environment() -> Result<&'static Environment> {
    static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();
    if let Some(env) = ENVIRONMENT.get() {
        return Ok(env);
    }
    let env = ort::init().build()?;
    Ok(ENVIRONMENT.get_or_init(|| env))
}

pub struct Specter {
    tokenizer: Tokenizer,
    session: Session,
}

impl Specter {
    /// Loads `tokenizer.json` and the proximity model from `dir`.
    pub fn load(dir: &Path) -> Result<Self> {
        let mut tokenizer = Tokenizer::from_file(dir.join("tokenizer.json"))?;
        tokenizer.with_truncation(Some(TruncationParams {
            max_length: MAX_TOKENS,
            ..Default::default()
        }))?;
        let session = Session::builder(environment()?)?.commit_from_file(dir.join(MODEL_FILE))?;
        Ok(Self { tokenizer, session })
    }

    /// The text's fingerprint (768 floats, unit length) and how many tokens it used.
    pub fn fingerprint(&mut self, text: &str) -> Result<(Vec<f32>, usize)> {
        let encoding = self.tokenizer.encode(text, true)?;
        let n = encoding.len();
        let ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id.into()).collect();
        let mask: Vec<i64> = encoding
            .get_attention_mask()
            .iter()
            .map(|&m| m.into())
            .collect();
        let outputs = self.session.run(ort::inputs![
            "input_ids" => Tensor::from_array(([1, n], ids))?,
            "attention_mask" => Tensor::from_array(([1, n], mask))?,
        ])?;
        let mut v = outputs["embedding"].try_extract_tensor::<f32>()?.1.to_vec();
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        v.iter_mut().for_each(|x| *x /= norm);
        Ok((v, n))
    }
}

/// The sample abstract and its fp32 fingerprint, written by ml/src/export_specter2.py.
fn sample() -> Result<(String, Vec<f32>)> {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/sample_abstract.json"))?;
    let text = fixture["text"]
        .as_str()
        .ok_or("fixture: no text")?
        .to_owned();
    let reference = serde_json::from_value::<Vec<f32>>(fixture["fingerprint_fp32"].clone())?;
    let norm = reference.iter().map(|x| x * x).sum::<f32>().sqrt();
    Ok((text, reference.iter().map(|x| x / norm).collect()))
}

/// Spike 0 measurement: load the model, fingerprint the sample abstract three times,
/// then one text at the 512-token limit. Returns a plain-text report.
pub fn spike_report(dir: &Path) -> Result<String> {
    let model_path = dir.join(MODEL_FILE);
    let model_mb = std::fs::metadata(&model_path)
        .map_err(|e| format!("{}: {e}", model_path.display()))?
        .len() as f64
        / 1e6;
    let (text, reference) = sample()?;

    let start = Instant::now();
    let mut specter = Specter::load(dir)?;
    let load_ms = start.elapsed().as_millis();

    let mut runs_ms = Vec::new();
    let (mut fingerprint, mut tokens) = (Vec::new(), 0);
    for _ in 0..3 {
        let start = Instant::now();
        (fingerprint, tokens) = specter.fingerprint(&text)?;
        runs_ms.push(start.elapsed().as_millis());
    }
    let start = Instant::now();
    let (_, long_tokens) = specter.fingerprint(&text.repeat(3))?;
    let long_ms = start.elapsed().as_millis();
    let cosine: f32 = fingerprint.iter().zip(&reference).map(|(a, b)| a * b).sum();

    Ok(format!(
        "engine: ONNX Runtime (ort)\nmodel: {MODEL_FILE}, {model_mb:.1} MB\nload: {load_ms} ms\n\
         sample abstract, {tokens} tokens: {runs_ms:?} ms\n{long_tokens} tokens: {long_ms} ms\n\
         dims: {}, cosine vs fp32 reference: {cosine:.4}",
        fingerprint.len()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs in CI: a placeholder fixture would otherwise only show up as a wrong number on the phone.
    #[test]
    fn fixture_holds_a_text_and_a_full_fingerprint() {
        let (text, reference) = sample().unwrap();
        assert!(text.contains("[SEP]"));
        assert_eq!(reference.len(), 768);
    }

    #[test]
    #[ignore = "needs the exported model in models/ (run ml/src/export_specter2.py)"]
    fn int8_matches_fp32_reference() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models");
        let report = spike_report(&dir).unwrap();
        println!("{report}");

        let (text, reference) = sample().unwrap();
        let (fingerprint, _) = Specter::load(&dir).unwrap().fingerprint(&text).unwrap();
        let cosine: f32 = fingerprint.iter().zip(&reference).map(|(a, b)| a * b).sum();
        assert_eq!(fingerprint.len(), 768);
        assert!(cosine >= 0.99, "cosine {cosine}");
    }
}

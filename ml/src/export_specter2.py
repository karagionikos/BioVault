"""Spike 0: export SPECTER2 to 8-bit ONNX for the Rust core (docs/08-ml.md section 1).

Run from the repository root:
    ml/.venv/Scripts/python ml/src/export_specter2.py

Writes into models/ (git-ignored): specter2-<variant>-int8.onnx, tokenizer.json, SHA256SUMS.
Writes app/src-tauri/fixtures/sample_abstract.json: the sample text and its fp32 fingerprint,
which the app and its test compare the int8 result against.
"""

import hashlib
import json
import pathlib

import numpy as np
import onnxruntime as ort
import torch
from adapters import AutoAdapterModel
from onnxruntime.quantization import QuantType, quantize_dynamic
from transformers import AutoTokenizer

ROOT = pathlib.Path(__file__).resolve().parents[2]
MODELS = ROOT / "models"
WORK = ROOT / "ml" / "artifacts"
FIXTURE = ROOT / "app" / "src-tauri" / "fixtures" / "sample_abstract.json"

BASE = "allenai/specter2_base"
VARIANTS = {"proximity": "allenai/specter2", "adhoc_query": "allenai/specter2_adhoc_query"}
MIN_COSINE = 0.99
# Plain int8 everywhere reached only cosine 0.988 (Spike 0). The feed-forward output projections of
# layers 8-10 carry most of that error; keeping just those three in fp32 costs about 21 MB.
KEEP_FP32 = [f"/model/bert/encoder/layer.{i}/output/dense/MatMul" for i in (8, 9, 10)]

# Synthetic texts written for this check (not real papers). The first is the sample abstract
# the phone times: about the length of a typical title + abstract.
SAMPLE_TITLE = (
    "Voluntary wheel running restores hippocampal BDNF signalling and spatial memory in aged mice"
)
SAMPLE_ABSTRACT = (
    "Brain-derived neurotrophic factor (BDNF) supports synaptic plasticity, and its expression in "
    "the hippocampus declines with age. Whether aerobic exercise late in life can reverse this "
    "decline, and whether any recovery depends on TrkB signalling, remains unclear. We housed "
    "20-month-old male and female C57BL/6 mice with or without access to a running wheel for six "
    "weeks (n = 24 per group) and compared them with 4-month-old controls. Running increased "
    "hippocampal BDNF protein by 38% relative to sedentary aged mice and restored levels of "
    "phosphorylated TrkB to those of young animals. In the Morris water maze, running mice located "
    "the hidden platform faster from day three of training and spent more time in the target "
    "quadrant during the probe trial. Dendritic spine density on CA1 pyramidal neurons rose by 21%, "
    "and long-term potentiation at Schaffer collateral synapses was enhanced. Blocking TrkB with "
    "ANA-12 during the final two weeks abolished the gains in spine density and memory without "
    "changing running distance, indicating that the benefits require BDNF-TrkB signalling rather "
    "than general activity. Effects were similar in males and females. Serum BDNF did not correlate "
    "with hippocampal levels, which cautions against using circulating BDNF as a proxy for brain "
    "changes in exercise trials. These results suggest that the aged hippocampus retains the "
    "capacity for exercise-induced plasticity and identify TrkB activation as a necessary step. "
    "Because the intervention began at an age equivalent to roughly 65 human years, the findings "
    "support testing structured aerobic training in older adults with mild cognitive complaints, "
    "with outcome measures that do not rely on blood biomarkers alone."
)
OTHER_SAMPLES = [
    "CRISPR base editing corrects a pathogenic PCSK9 variant in primary human hepatocytes[SEP]"
    "An adenine base editor delivered by lipid nanoparticles corrected the variant in 62% of "
    "alleles and lowered secreted PCSK9 by half, with no off-target edits detected.",
    "Intermittent fasting versus daily calorie restriction for weight loss in adults with obesity"
    "[SEP]In a 12-month randomized trial of 240 adults, both diets produced similar weight loss; "
    "adherence was lower with intermittent fasting after six months.",
    "Gut microbiota composition predicts response to anti-PD-1 therapy in metastatic melanoma"
    "[SEP]Responders had higher faecal abundance of Ruminococcaceae before treatment.",
    "Sleep deprivation impairs amyloid-beta clearance through the glymphatic system",
    "BDNF exercise memory aging",
]


class ClsOnly(torch.nn.Module):
    """The fingerprint is the first-token ([CLS]) output: 768 floats (normalized in Rust)."""

    def __init__(self, model):
        super().__init__()
        self.model = model

    def forward(self, input_ids, attention_mask):
        out = self.model(input_ids=input_ids, attention_mask=attention_mask)
        return out.last_hidden_state[:, 0, :]


def unit(rows):
    return rows / np.linalg.norm(rows, axis=1, keepdims=True)


def main():
    MODELS.mkdir(exist_ok=True)
    WORK.mkdir(parents=True, exist_ok=True)
    tokenizer = AutoTokenizer.from_pretrained(BASE)
    tokenizer.backend_tokenizer.save(str(MODELS / "tokenizer.json"))

    sample = SAMPLE_TITLE + tokenizer.sep_token + SAMPLE_ABSTRACT
    texts = [sample, *OTHER_SAMPLES]
    enc = tokenizer(texts, padding=True, truncation=True, max_length=512, return_tensors="pt")
    feeds = {k: enc[k].numpy() for k in ("input_ids", "attention_mask")}

    for variant, adapter in VARIANTS.items():
        model = AutoAdapterModel.from_pretrained(BASE)
        model.load_adapter(adapter, source="hf", load_as=variant, set_active=True)
        wrapped = ClsOnly(model).eval()
        with torch.no_grad():
            reference = wrapped(enc["input_ids"], enc["attention_mask"]).numpy()

        fp32 = WORK / f"specter2-{variant}-fp32.onnx"
        torch.onnx.export(
            wrapped,
            (enc["input_ids"], enc["attention_mask"]),
            str(fp32),
            input_names=["input_ids", "attention_mask"],
            output_names=["embedding"],
            dynamic_axes={
                "input_ids": {0: "batch", 1: "seq"},
                "attention_mask": {0: "batch", 1: "seq"},
                "embedding": {0: "batch"},
            },
            opset_version=17,
            dynamo=False,
        )
        # Quantize in the work folder; only a model that passes the check reaches models/.
        int8 = WORK / f"specter2-{variant}-int8.onnx"
        quantize_dynamic(str(fp32), str(int8), weight_type=QuantType.QInt8, nodes_to_exclude=KEEP_FP32)
        fp32.unlink()

        quantized = ort.InferenceSession(str(int8)).run(None, feeds)[0]
        cosines = (unit(reference) * unit(quantized)).sum(axis=1)
        print(f"{variant}: {int8.stat().st_size / 1e6:.1f} MB, cosine int8 vs fp32 min {cosines.min():.4f}")
        if cosines.min() < MIN_COSINE:
            raise SystemExit(f"{variant}: quantization hurt too much ({cosines}); {int8} kept for inspection")
        int8.replace(MODELS / int8.name)

        if variant == "proximity":
            FIXTURE.parent.mkdir(exist_ok=True)
            fixture = {"text": sample, "fingerprint_fp32": reference[0].tolist()}
            FIXTURE.write_text(json.dumps(fixture) + "\n", encoding="utf-8", newline="\n")

    files = [MODELS / f"specter2-{v}-int8.onnx" for v in VARIANTS] + [MODELS / "tokenizer.json"]
    sums = "".join(f"{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n" for p in files)
    (MODELS / "SHA256SUMS").write_text(sums, encoding="utf-8", newline="\n")
    print(sums, end="")


if __name__ == "__main__":
    main()

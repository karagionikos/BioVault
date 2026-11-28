import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function App() {
  const [platform, setPlatform] = useState("…");
  const [report, setReport] = useState("");
  const [running, setRunning] = useState(false);

  useEffect(() => {
    invoke<string>("platform").then(setPlatform, (err) => setPlatform(`error: ${String(err)}`));
  }, []);

  function fingerprint() {
    setRunning(true);
    setReport("Loading the model and fingerprinting…");
    invoke<string>("fingerprint_sample")
      .then(setReport, (err) => setReport(`error: ${String(err)}`))
      .finally(() => setRunning(false));
  }

  return (
    <main>
      <h1>BioVault</h1>
      <p>Spike 0 · Rust core is running on: {platform}</p>
      <button type="button" onClick={fingerprint} disabled={running}>
        Fingerprint the sample abstract
      </button>
      <pre style={{ whiteSpace: "pre-wrap" }}>{report}</pre>
    </main>
  );
}

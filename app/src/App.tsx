import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function App() {
  const [platform, setPlatform] = useState("…");

  useEffect(() => {
    invoke<string>("platform").then(setPlatform, (err) => setPlatform(`error: ${String(err)}`));
  }, []);

  return (
    <main>
      <h1>BioVault</h1>
      <p>Spike 0 · Rust core is running on: {platform}</p>
    </main>
  );
}

import os, time, json, hashlib
from datetime import datetime

BASE = os.getcwd()
OUT = os.path.join(BASE, "out")
LEDGER = os.path.join(BASE, "ledger")

print("🧠 TITAN PYTHON CORTEX ONLINE (WINDOWS)")

if not os.path.exists(LEDGER):
    os.makedirs(LEDGER)

VOCAB = {"nouns": ["money", "code", "titan", "user"], "verbs": ["build", "run", "deploy"]}

while True:
    try:
        for f in os.listdir(OUT):
            if f.endswith(".bus") and not f.endswith(".done"):
                p = os.path.join(OUT, f)
                with open(p, "r", encoding="utf-8") as rf:
                    text = rf.read().lower()
                
                concepts = []
                for n in VOCAB["nouns"]:
                    if n in text: concepts.append({"type": "noun", "val": n})
                for v in VOCAB["verbs"]:
                    if v in text: concepts.append({"type": "verb", "val": v})

                result = {
                    "hash": hashlib.sha256(text.encode()).hexdigest(),
                    "timestamp": datetime.utcnow().isoformat(),
                    "semantics": concepts
                }

                l_path = os.path.join(LEDGER, f + ".json")
                with open(l_path, "w") as lf:
                    json.dump(result, lf, indent=2)
                
                os.rename(p, p + ".done")
                print(f"   Analyzed: {f}")
    except Exception as e:
        print(f"Error: {e}")
    time.sleep(1)

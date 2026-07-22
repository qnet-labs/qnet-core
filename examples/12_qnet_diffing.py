#!/usr/bin/env python3
"""
Example 12 — Diffing Two Topology Versions

Use diff() to show what changed between a v1 and v2 network design.
Demonstrates version management of quantum network configurations.

Run:
    python examples/12_qnet_diffing.py
"""

import os

# ── 1. Use the existing v1/v2 files from the repo root ─────────────────
v1_path = "network_v1.qnet"
v2_path = "network_v2.qnet"

# If they don't exist at root, check examples/
for base in (os.path.dirname(v1_path) or ".", "."):
    if not os.path.exists(v1_path):
        candidates = [
            f"{base}/{v1_path}",
            f"{base}/network_v1.qnet",
        ]
        for c in candidates:
            if os.path.exists(c):
                v1_path = c
                break

for base in (os.path.dirname(v2_path) or ".", "."):
    if not os.path.exists(v2_path):
        candidates = [
            f"{base}/{v2_path}",
            f"{base}/network_v2.qnet",
        ]
        for c in candidates:
            if os.path.exists(c):
                v2_path = c
                break

# ── 2. Diff the two topology files ─────────────────────────────────────
from qnet_core import diff

print(f"=== Comparing {v1_path} vs {v2_path} ===")

result = diff(v1_path, v2_path)

if "error" in result:
    print(f"  Error: {result['error']}")
else:
    print(f"  Summary: {result.get('summary', 'N/A')}")
    print()

    if result.get("nodes_added"):
        print("  Nodes ADDED:")
        for n in result["nodes_added"]:
            print(f"    + {n}")

    if result.get("nodes_removed"):
        print("  Nodes REMOVED:")
        for n in result["nodes_removed"]:
            print(f"    - {n}")

    if result.get("nodes_modified"):
        print("  Nodes MODIFIED:")
        for n in result["nodes_modified"]:
            print(f"    ~ {n}")

    if result.get("links_added"):
        print("  Links ADDED:")
        for l in result["links_added"]:
            print(f"    + {l}")

    if result.get("links_removed"):
        print("  Links REMOVED:")
        for l in result["links_removed"]:
            print(f"    - {l}")

    if result.get("links_modified"):
        print("  Links MODIFIED:")
        for l in result["links_modified"]:
            print(f"    ~ {l}")

# ── 3. Now create two custom versions and diff them ────────────────────
print("\n=== Custom version diff demo ===")
from qnet_core import QNetFile, save_qnet_file_wrapper

v_a = QNetFile(name="version_a")
v_a.add_node("X", memory_lifetime_ms=500.0)
v_a.add_node("Y", memory_lifetime_ms=1000.0)
v_a.add_link("l1", src="X", to="Y", distance_km=10.0, base_fidelity=0.95,
             generation_rate_hz=1_000.0)

v_b = QNetFile(name="version_b")
v_b.add_node("X", memory_lifetime_ms=800.0)     # changed T2
v_b.add_node("Y", memory_lifetime_ms=1000.0)
v_b.add_node("Z", memory_lifetime_ms=600.0)      # new node
v_b.add_link("l1", src="X", to="Y", distance_km=20.0, base_fidelity=0.90,  # changed params
             generation_rate_hz=800.0)
v_b.add_link("l2", src="Y", to="Z", distance_km=15.0, base_fidelity=0.88,  # new link
             generation_rate_hz=700.0)

save_qnet_file_wrapper(v_a, "examples/v_a.qnet")
save_qnet_file_wrapper(v_b, "examples/v_b.qnet")

diff_result = diff("examples/v_a.qnet", "examples/v_b.qnet")
print(f"  Summary: {diff_result.get('summary', 'N/A')}")
for key in ("nodes_added", "nodes_removed", "nodes_modified",
            "links_added", "links_removed", "links_modified"):
    items = diff_result.get(key, [])
    if items:
        print(f"  {key}: {items}")

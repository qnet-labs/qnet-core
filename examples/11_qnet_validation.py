#!/usr/bin/env python3
"""
Example 11 — Validating a .qnet File

Intentionally introduce errors in a QNetFile to show how validate() catches them
before simulation: self-loops, duplicate node IDs, and other structural issues.

Run:
    python examples/11_qnet_validation.py
"""

from qnet_core import validate
from qnet_core.qnet_core import QNetFile, save_qnet_file_wrapper

# ── 1. Create a valid file for baseline comparison ─────────────────────
qf_valid = QNetFile(name="valid_network")
qf_valid.add_node("A", memory_lifetime_ms=1000.0)
qf_valid.add_node("B", memory_lifetime_ms=1000.0)
qf_valid.add_link("link_1", src="A", to="B", distance_km=10.0, base_fidelity=0.95,
                  generation_rate_hz=1000.0)

valid_path = "examples/valid_network.qnet"
save_qnet_file_wrapper(qf_valid, valid_path)

print("=== Validating a well-formed .qnet file ===")
result = validate(valid_path)
print(f"  Valid: {result['valid']}")
if result.get("warnings"):
    print(f"  Warnings ({len(result['warnings'])}):")
    for w in result["warnings"]:
        print(f"    - {w['type']}: {w['message']}")

# ── 2. Create files with intentional errors ───────────────────────────
print("\n=== Validating with error: self-loop link ===")
self_loop = QNetFile(name="self_loop_test")
self_loop.add_node("NodeX", memory_lifetime_ms=1000.0)
# Self-loop: both src and to are the same node
self_loop.add_link("bad_link", src="NodeX", to="NodeX", distance_km=10.0,
                   base_fidelity=0.95, generation_rate_hz=1000.0)

selfloop_path = "examples/self_loop.qnet"
save_qnet_file_wrapper(self_loop, selfloop_path)

result = validate(selfloop_path)
print(f"  Valid: {result['valid']}")
for err in result.get("errors", []):
    print(f"  Error [{err['type']}]: {err['message']}")

# ── 3. Create a file with duplicate node IDs ───────────────────────────
print("\n=== Validating with error: duplicate node IDs ===")
dup_nodes = QNetFile(name="duplicate_test")
dup_nodes.add_node("SameId", memory_lifetime_ms=1000.0)
# Note: add_node overwrites in practice, so create via save + manual edit
# Instead, use the existing invalid_network.qnet which has known issues
invalid_path = "invalid_network.qnet"

if __import__("os").path.exists(invalid_path):
    result = validate(invalid_path)
    print(f"  Valid: {result['valid']}")
    for err in result.get("errors", []):
        print(f"  Error [{err['type']}]: {err['message']}")

# ── 4. Summary ─────────────────────────────────────────────────────────
print("\n=> validate() never raises — it returns a dict with 'valid', 'errors', and 'warnings'.")
print("   Always call validate() before attempting simulation on untrusted .qnet files.")

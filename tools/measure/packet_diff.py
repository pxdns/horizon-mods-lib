#!/usr/bin/env python3
"""Measures how far the 1.21.11 (774) wire format and id tables are from 26.1 (775) and Horizon's 26.3 (777).

usage: packet_diff.py DATA_DIR HORIZON_IDS_RS > report.md

DATA_DIR holds <version>-protocol.json, -blocks.json, -items.json, -entities.json for 1.21.11 and 26.1 (the
PrismarineJS/minecraft-data files; tools/measure/fetch.py downloads them). HORIZON_IDS_RS is Horizon's
crates/horizon-protocol/src/ids.rs, which lists packet names by id for 26.3. Only packet *names and ids* exist for 26.3 in
machine-readable form, so field layouts are compared for 774 -> 775 only.
"""
import json
import re
import sys
from pathlib import Path

data, ids_rs = Path(sys.argv[1]), Path(sys.argv[2])


def load(version, kind):
    return json.loads((data / f"{version}-{kind}.json").read_text())


def packet_tables(protocol):
    """{(state, direction): {id: name}} from a minecraft-data protocol.json."""
    out = {}
    for state in ("handshaking", "status", "login", "configuration", "play"):
        for direction in ("toClient", "toServer"):
            block = protocol.get(state, {}).get(direction)
            if not block:
                continue
            mapping = None
            for t in block["types"].get("packet", [None, []])[1]:
                if isinstance(t, dict) and t.get("name") == "name":
                    mapping = t["type"][1]["mappings"]
            if mapping is None:
                continue
            out[(state, direction)] = {int(k, 16): v for k, v in mapping.items()}
    return out


def horizon_tables(text):
    """The same shape from ids.rs: NAMES arrays in module order."""
    out = {}
    state_names = {"handshake": "handshaking", "status": "status", "login": "login", "configuration": "configuration", "play": "play"}
    for state, directions in re.findall(r"pub mod (\w+) \{(.*?)\n\}", text, re.S):
        if state not in state_names:
            continue
        for direction, body in re.findall(r"pub mod (clientbound|serverbound) \{(.*?)\n    \}", directions, re.S):
            m = re.search(r"NAMES: &\[&str\] = &\[(.*?)\];", body, re.S)
            if not m:
                continue
            names = re.findall(r'"([^"]+)"', m.group(1))
            key = (state_names[state], "toClient" if direction == "clientbound" else "toServer")
            out[key] = {i: n for i, n in enumerate(names)}
    return out


def normalise(name):
    return name.replace("_", "").lower()


def compare_tables(a, b):
    rows = []
    for key in sorted(set(a) | set(b)):
        ta, tb = a.get(key, {}), b.get(key, {})
        na, nb = {normalise(v): (k, v) for k, v in ta.items()}, {normalise(v): (k, v) for k, v in tb.items()}
        common = set(na) & set(nb)
        moved = sorted(na[n][1] for n in common if na[n][0] != nb[n][0])
        removed = sorted(na[n][1] for n in set(na) - set(nb))
        added = sorted(nb[n][1] for n in set(nb) - set(na))
        rows.append((key, len(ta), len(tb), len(common), moved, removed, added))
    return rows


def layout(protocol, state, direction, name):
    block = protocol.get(state, {}).get(direction, {}).get("types", {})
    return json.dumps(block.get("packet_" + name), sort_keys=True)


def table_md(title, rows):
    out = [f"### {title}", "", "| State / direction | Packets | Packets | In both | Id differs | Removed | Added |", "|---|---:|---:|---:|---:|---:|---:|"]
    for (state, direction), n1, n2, common, moved, removed, added in rows:
        out.append(f"| {state} {direction} | {n1} | {n2} | {common} | {len(moved)} | {len(removed)} | {len(added)} |")
    out.append("")
    return out


def listing(rows, which):
    out = []
    for (state, direction), _, _, _, moved, removed, added in rows:
        items = {"removed": removed, "added": added}[which]
        if items:
            out.append(f"- {state} {direction}: " + ", ".join(f"`{n}`" for n in items))
    return out


def registry_shift(a, b, field="name"):
    ia = {x[field]: x["id"] for x in a}
    ib = {x[field]: x["id"] for x in b}
    common = set(ia) & set(ib)
    moved = sum(1 for n in common if ia[n] != ib[n])
    return len(ia), len(ib), len(common), moved, sorted(set(ia) - set(ib)), sorted(set(ib) - set(ia))


def block_state_shift(a, b):
    sa = {x["name"]: (x["minStateId"], x["maxStateId"]) for x in a}
    sb = {x["name"]: (x["minStateId"], x["maxStateId"]) for x in b}
    common = set(sa) & set(sb)
    moved = sum(1 for n in common if sa[n][0] != sb[n][0])
    resized = sum(1 for n in common if sa[n][1] - sa[n][0] != sb[n][1] - sb[n][0])
    return (max(v[1] for v in sa.values()) + 1, max(v[1] for v in sb.values()) + 1, moved, resized)


p774, p775 = load("1.21.11", "protocol"), load("26.1", "protocol")
t774, t775 = packet_tables(p774), packet_tables(p775)
t777 = horizon_tables(ids_rs.read_text())

out = ["# Measurement: how far 1.21.11 is from Horizon's 26.3", "",
       "Protocol numbers: 1.21.11 = 774, 26.1 = 775, 26.2 = 776 (no data source found), 26.3 = 777 (Horizon).", "",
       "Generated by `tools/measure/packet_diff.py`; the inputs are PrismarineJS/minecraft-data for 774 and 775 and Horizon's `ids.rs` for 777.", "",
       "## Packet ids and names", ""]
r1 = compare_tables(t774, t775)
out += table_md("1.21.11 (774) -> 26.1 (775)", r1)
# Horizon's names are Mojang's; minecraft-data's are not, so names and ids cannot be compared across the two sources.
# Packet *counts* per state and direction can.
out += ["### Packet counts, 774 -> 775 -> Horizon 777", "",
        "Horizon's `ids.rs` uses Mojang's packet names and minecraft-data uses its own (`add_resource_pack` against `resource_pack_push`), "
        "so names and ids are **not** compared between the two sources; only how many packets each state has is.", "",
        "| State / direction | 774 | 775 | 777 (Horizon) | Net change 774 to 777 |", "|---|---:|---:|---:|---:|"]
total_change = 0
for key in sorted(set(t774) | set(t775) | set(t777)):
    n774, n775, n777 = len(t774.get(key, {})), len(t775.get(key, {})), len(t777.get(key, {}))
    total_change += abs(n777 - n774)
    out.append(f"| {key[0]} {key[1]} | {n774} | {n775} | {n777} | {n777 - n774:+d} |")
out += ["", f"The counts differ by {total_change} packets in total between 774 and 777. That is a lower bound on the packets added or removed: "
        "additions and removals can cancel, and a packet can change layout without changing the count.", "",
        "An id-level comparison of 774 with 777 needs 1.21.11's packet report in Mojang's own names, which Mojang's data generator "
        "writes (`java -DbundlerMainClass=net.minecraft.data.Main -jar server.jar --reports`, Java 21 for 1.21.11). "
        "That report was not produced for this measurement; see the next steps in the proposal.", ""]

# layouts 774 -> 775
changed, same, listing_changed = 0, 0, []
for (state, direction), names in t774.items():
    other = {normalise(v): v for v in t775.get((state, direction), {}).values()}
    for name in names.values():
        n2 = other.get(normalise(name))
        if not n2:
            continue
        if layout(p774, state, direction, name) == layout(p775, state, direction, n2):
            same += 1
        else:
            changed += 1
            listing_changed.append(f"{state} {direction} `{name}`")
out += ["## Packet field layouts, 774 -> 775", "",
        f"Of {same + changed} packets present in both, {same} have an identical layout in minecraft-data and {changed} differ.", ""]
out += ["Layouts that differ:", ""] + [f"- {x}" for x in listing_changed] + [""]
out += ["Layouts for 26.2 and 26.3 are not machine-readable in these sources: the 775 -> 777 layout changes are not measured here and "
        "are the first thing to read from Horizon's decoders (`crates/horizon-protocol/src/packets/play.rs`, 3269 lines) when the translator is written.", ""]

# registries
out += ["## Registry id tables, 774 -> 775", "", "| Table | Entries 774 | Entries 775 | In both | Id differs | Only in 774 | Only in 775 |", "|---|---:|---:|---:|---:|---:|---:|"]
details = []
for kind in ("items", "entities"):
    n1, n2, c, m, only1, only2 = registry_shift(load("1.21.11", kind), load("26.1", kind))
    out.append(f"| {kind} | {n1} | {n2} | {c} | {m} | {len(only1)} | {len(only2)} |")
    if only1 or only2:
        details.append(f"- {kind}: only in 774: {', '.join(only1) or 'none'}; only in 775: {', '.join(only2) or 'none'}")
b1, b2 = load("1.21.11", "blocks"), load("26.1", "blocks")
n1, n2, c, m, only1, only2 = registry_shift(b1, b2)
out.append(f"| blocks | {n1} | {n2} | {c} | {m} | {len(only1)} | {len(only2)} |")
details.append(f"- blocks: only in 774: {', '.join(only1) or 'none'}; only in 775: {', '.join(only2) or 'none'}")
s1, s2, moved_states, resized = block_state_shift(b1, b2)
out += ["", f"Block states: {s1} in 774, {s2} in 775; {moved_states} blocks start at a different state id and {resized} blocks have a different number of states.", ""]
out += details + [""]
out += ["Block, item and entity tables for 26.2 and 26.3 were not available in a machine-readable form here; Horizon generates its own "
        "(`tools/gen_block_state_data.py`, `gen_flat_tables.py`) and that is the source for the last two steps.", ""]
print("\n".join(out))

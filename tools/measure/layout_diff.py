#!/usr/bin/env python3
"""Compares packet field layouts: Horizon's 26.3 decoders against minecraft-data's 26.1 layouts, matched through Mojang's packet ids.

usage: layout_diff.py REPORTS_DIR MINECRAFT_DATA_DIR HORIZON_PACKETS_DIR > layouts.md

HORIZON_PACKETS_DIR is crates/horizon-protocol/src/packets. Horizon declares packets with `packet!(Name { field: rust_type => Codec, ... })`
and lists them per state in `packet_enum!`; both are read as text. A layout is reduced to the kinds of its top-level fields
(varint, int, string, uuid, position, nbt, option, array, nested ...). Equal kind sequences can still hide a changed meaning, and
different ones can be a harmless difference in how the two sources describe a type, so the output is a list to review, not a verdict.
"""
import json
import re
import sys
from pathlib import Path

reports, mdata, packets_dir = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])


# ---------------------------------------------------------------- Horizon's decoders
def balanced(text, start):
    """The text between the brace at `start` and its match, and the index after it."""
    depth = 0
    for i in range(start, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[start + 1:i], i + 1
    raise ValueError("unbalanced")


def strip_comments(text):
    return re.sub(r"//[^\n]*", "", text)


def split_fields(body):
    """Top-level `name: type => Codec` entries of a packet! body."""
    parts, depth, cur = [], 0, ""
    body = body.replace("=>", "\u2192")  # the arrow's > is not a bracket
    for ch in body:
        if ch in "<([{":
            depth += 1
        elif ch in ">)]}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    out = []
    for p in parts:
        m = re.match(r"\s*(?:#\[[^\]]*\]\s*)*(\w+)\s*:\s*(.*?)\s*\u2192\s*(.*?)\s*$", p, re.S)
        if m:
            out.append((m.group(1), re.sub(r"\s+", " ", m.group(3))))
    return out


def horizon_structs(directory):
    """{struct name: {file stem: fields}}; the same name is used in several states (Ping, Disconnect), so the file matters."""
    structs, hand = {}, set()
    for path in directory.glob("*.rs"):
        text = strip_comments(path.read_text())
        for m in re.finditer(r"packet!\s*\(\s*(?:#\[[^\]]*\]\s*)*(\w+)\s*\{", text):
            body, _ = balanced(text, m.end() - 1)
            structs.setdefault(m.group(1), {})[path.stem] = split_fields(body)
        for m in re.finditer(r"\bimpl\s+Codec<(\w+)>\s+for", text):
            hand.add(m.group(1))
    return structs, hand


def horizon_enum(directory):
    """{(state, direction): {mojang_name: struct}} from the packet_enum! blocks."""
    out = {}
    for path in directory.glob("*.rs"):
        state = path.stem
        text = strip_comments(path.read_text())
        for m in re.finditer(r"packet_enum!\s*\(\s*(?:#\[[^\]]*\]\s*)*(Clientbound|Serverbound)\s*,\s*[\w:]+\s*\{", text):
            body, _ = balanced(text, m.end() - 1)
            table = {}
            for v in re.finditer(r"\w+\((\w+)\)\s*=\s*(\w+)", body):
                table[v.group(2).lower()] = v.group(1)
            out[(state, m.group(1).lower())] = table
    return out


KIND = [
    (r"^(VarInt|VarLong)$", "varint"), (r"^Int$", "i32"), (r"^Long$", "i64"), (r"^Short$", "i16"), (r"^(Byte|UByte)$", "i8"),
    (r"^Bool$", "bool"), (r"^Float$", "f32"), (r"^Double$", "f64"), (r"^(Str|StrN|Ident)", "string"), (r"^Uuid$", "uuid"),
    (r"^(Pos|SectionPos|ChunkPos)$", "position"), (r"^(UShort)$", "i16"), (r"^(Rest|Bytes)$", "buffer"), (r"^Vec3d$", "vec3d"), (r"^LpVec3$", "lpvec3"), (r"^Nbt$", "nbt"), (r"^OptVarInt$", "option"), (r"^Opt<", "option"),
    (r"^List<", "array"), (r"^BytesN", "buffer"), (r"^(Slot|HashedSlot|DelimitedSlot|Stack)$", "slot"),
]


def horizon_kind(codec):
    for pattern, kind in KIND:
        if re.match(pattern, codec):
            return kind
    return "nested:" + codec


# ---------------------------------------------------------------- minecraft-data
PRIMITIVE = {"varint", "varlong", "i32", "i64", "i16", "u16", "i8", "u8", "bool", "f32", "f64", "string", "UUID", "position", "nbt",
             "anonymousNbt", "anonOptionalNbt", "slot", "restBuffer", "vec3f", "vec3f64", "lpVec3", "void"}
ALIASES = {}


def resolve(t, depth=0):
    """Follows a named type through minecraft-data's alias table to what it is made of, one level of containers kept."""
    if isinstance(t, str) and t not in PRIMITIVE and t in ALIASES and depth < 6:
        return resolve(ALIASES[t], depth + 1)
    if isinstance(t, list) and t and t[0] == "mapper" and isinstance(t[1], dict):
        return resolve(t[1].get("type"), depth + 1)
    return t


def md_kind(t):
    t = resolve(t)
    if isinstance(t, str):
        return {"varint": "varint", "varlong": "varint", "i32": "i32", "i64": "i64", "i16": "i16", "u16": "i16", "i8": "i8", "u8": "i8",
                "bool": "bool", "f32": "f32", "f64": "f64", "string": "string", "UUID": "uuid", "position": "position",
                "nbt": "nbt", "anonymousNbt": "nbt", "anonOptionalNbt": "nbt", "slot": "slot", "restBuffer": "buffer",
                "vec3f": "vec3f", "vec3f64": "vec3d", "lpVec3": "lpvec3"}.get(t, "nested:" + t)
    head = t[0]
    return {"option": "option", "array": "array", "buffer": "buffer", "container": "nested:container", "switch": "switch",
            "bitfield": "bitfield", "topBitSetTerminatedArray": "array", "pstring": "string"}.get(head, "nested:" + head)


def md_layout(protocol, state, direction, name):
    types = protocol.get(state, {}).get(direction, {}).get("types", {})
    t = types.get("packet_" + name)
    if t is None:
        return None
    if isinstance(t, list) and t[0] == "container":
        return [(f.get("name") or "_", md_kind(f["type"])) for f in t[1]]
    return [("_", md_kind(t))]


def merge_hint(a, b):
    """True when two kinds are the same thing described by the two sources (nested containers match any nested type)."""
    if a == b:
        return True
    if a.startswith("nested:") and b.startswith("nested:"):
        return True
    return {a, b} <= {"vec3", "nested:container"} or {a, b} <= {"position", "i64"}


# ---------------------------------------------------------------- compare
structs, hand = horizon_structs(packets_dir)
enums = horizon_enum(packets_dir)
p775 = json.loads((mdata / "26.1-protocol.json").read_text())
for _state in p775.values():
    if isinstance(_state, dict):
        for _dir in _state.values():
            if isinstance(_dir, dict):
                for _k, _v in _dir.get("types", {}).items():
                    if not _k.startswith("packet") and _k != "packet":
                        ALIASES.setdefault(_k, _v)
for _k, _v in p775.get("types", {}).items():
    ALIASES.setdefault(_k, _v)
r775 = json.loads((reports / "26.1" / "reports" / "packets.json").read_text())
STATE = {"handshake": "handshaking"}
DIR = {"clientbound": "toClient", "serverbound": "toServer"}

same, differ, unclear, unmodelled, hand_written, nomd = [], [], [], [], [], []
for (state, direction), table in sorted(enums.items()):
    mstate = STATE.get(state, state)
    mid = {int(k, 16): v for k, v in next(
        (t["type"][1]["mappings"] for t in p775[mstate][DIR[direction]]["types"]["packet"][1] if isinstance(t, dict) and t.get("name") == "name"), {}).items()}
    by_mojang_id = {v["protocol_id"]: n.removeprefix("minecraft:") for n, v in r775.get(state, {}).get(direction, {}).items()}
    mojang_to_md = {by_mojang_id[i]: mid[i] for i in by_mojang_id if i in mid}
    for mojang, struct in sorted(table.items()):
        label = f"{state} {direction} `{mojang}`"
        if struct == "Empty":
            fields = []
        elif struct in hand and struct not in structs:
            hand_written.append(label)
            continue
        elif struct not in structs:
            unmodelled.append(label)
            continue
        else:
            # The enum's own file first, then the files it imports shared packets from.
            by_file = structs[struct]
            fields = by_file.get(state) or next(by_file[f] for f in ("configuration", "play", "login", "status", "handshake", *by_file) if f in by_file)
        md_name = mojang_to_md.get(mojang)
        layout = md_layout(p775, mstate, DIR[direction], md_name) if md_name else None
        if layout is None:
            nomd.append(label + " (no 26.1 packet of this name)")
            continue
        h = []
        for _, c in fields:
            k = horizon_kind(c)
            h += ["f64"] * 3 if k == "vec3d" else [k]
        m = [k for _, k in layout]
        row = (label, struct, h, m, [n for n, _ in fields], [n for n, _ in layout])
        if len(h) != len(m):
            differ.append(row)
        elif all(merge_hint(a, b) for a, b in zip(h, m)):
            same.append(row)
        elif all(merge_hint(a, b) or a.startswith("nested:") or b.startswith("nested:") for a, b in zip(h, m)):
            unclear.append(row)  # a nested type against a plain one: the sources describe it differently
        else:
            differ.append(row)

out = ["# Packet layouts: Horizon's 26.3 decoders against 26.1", "",
       "Generated by `tools/measure/layout_diff.py`. Horizon's decoders (`crates/horizon-protocol/src/packets`, read as text) are compared "
       "with minecraft-data's 26.1 (protocol 775) layouts, matched packet by packet through Mojang's packet ids for 26.1. A layout is "
       "reduced to the kinds of its top-level fields; equal kinds can still hide a changed meaning, and a difference can be only the "
       "two sources describing a type differently. Treat the list as the packets to read by hand, not as a verdict.", "",
       f"- Compared: {len(same) + len(differ)} packets modelled by Horizon with a 26.1 packet of the same name.",
       f"- Same field kinds: {len(same)}.", f"- Different number or kinds of fields (review): {len(differ)}.",
       f"- Only a nested type against a plain one (the sources describe it differently; read by hand): {len(unclear)}.",
       f"- Written by hand in Horizon (not read here): {len(hand_written)}.",
       f"- Not modelled by Horizon (decoded as `Unknown`): {len(unmodelled)}.",
       f"- Modelled by Horizon, no packet of that name in 26.1 (new in 26.2 or 26.3, or renamed): {len(nomd)}.", "",
       "## Packets whose field kinds differ", ""]
for label, struct, h, m, hn, mn in differ:
    out += [f"### {label} (`{struct}`)", "", f"- Horizon 26.3: `{', '.join(f'{n}: {k}' for n, k in zip(hn, h)) or 'no fields'}`",
            f"- minecraft-data 26.1: `{', '.join(f'{n}: {k}' for n, k in zip(mn, m)) or 'no fields'}`", ""]
out += ["## Nested against plain (probably the same on the wire)", ""] + [f"- {r[0]} (`{r[1]}`)" for r in unclear] + [""]
out += ["## New or renamed since 26.1", ""] + [f"- {x}" for x in nomd] + [""]
out += ["## Written by hand in Horizon (compare these by reading `packets/*.rs`)", ""] + [f"- {x}" for x in hand_written] + [""]
out += [f"## Not modelled by Horizon ({len(unmodelled)})", "", "A translator passes these through unchanged or needs its own layout from elsewhere.", ""]
out += [f"- {x}" for x in unmodelled] + [""]
print("\n".join(out))

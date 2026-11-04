#!/usr/bin/env python3
"""Downloads the minecraft-data files packet_diff.py reads, into the folder given: python3 fetch.py DATA_DIR"""
import json
import sys
import urllib.request
from pathlib import Path

out = Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
base = "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/"
paths = json.load(urllib.request.urlopen(base + "dataPaths.json", timeout=30))["pc"]
for version in ("1.21.11", "26.1"):
    for kind, file in (("protocol", "protocol.json"), ("blocks", "blocks.json"), ("items", "items.json"), ("entities", "entities.json")):
        text = urllib.request.urlopen(f"{base}{paths[version][kind]}/{file}", timeout=60).read()
        (out / f"{version}-{kind}.json").write_bytes(text)
        print(version, kind, len(text))

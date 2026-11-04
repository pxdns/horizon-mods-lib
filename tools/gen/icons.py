#!/usr/bin/env python3
"""Downloads each catalogued mod's real icon from Modrinth (its project icon), resized to 96px PNG, with author + licence."""
import io, json, sys, urllib.request, urllib.parse
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
UA = "horizon-mods-lib-icon-fetch (https://github.com/pxdns/horizon-mods-lib)"
data = json.load(open(ROOT / "site/data/mods.json"))
mods = data["mods"]
ids = [m["id"] for m in mods]
info = {}
for i in range(0, len(ids), 50):
    chunk = ids[i:i + 50]
    url = "https://api.modrinth.com/v2/projects?ids=" + urllib.parse.quote(json.dumps(chunk))
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    for p in json.load(urllib.request.urlopen(req, timeout=60)):
        info[p["id"]] = p
print("projects", len(info), "of", len(ids))
# author per project: team members
teams = {}
for p in info.values():
    teams[p["team"]] = p["id"]
tids = list(teams)
authors = {}
for i in range(0, len(tids), 50):
    chunk = tids[i:i + 50]
    url = "https://api.modrinth.com/v2/teams?ids=" + urllib.parse.quote(json.dumps(chunk))
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    for members in json.load(urllib.request.urlopen(req, timeout=60)):
        if members:
            authors[members[0]["team_id"]] = [m["user"]["username"] for m in members][:3]
out = ROOT / "assets/icons/catalogue"
credits = {}
ok = 0
for m in mods:
    p = info.get(m["id"])
    if not p:
        continue
    credits[m["id"]] = dict(title=p["title"], slug=p["slug"], authors=authors.get(p["team"], []), licence=(p.get("license") or {}).get("id") or "",
                            icon=bool(p.get("icon_url")))
    if not p.get("icon_url"):
        continue
    try:
        req = urllib.request.Request(p["icon_url"], headers={"User-Agent": UA})
        raw = urllib.request.urlopen(req, timeout=60).read()
        im = Image.open(io.BytesIO(raw)).convert("RGBA")
        im.thumbnail((96, 96), Image.LANCZOS)
        im.save(out / f"{m['page']}.png", optimize=True)
        (out / f"{m['page']}.svg").unlink(missing_ok=True)
        credits[m["id"]]["file"] = f"{m['page']}.png"
        ok += 1
    except Exception as e:
        print("skip", m["page"], e, file=sys.stderr)
json.dump(credits, open(ROOT / "site/data/icon-credits.json", "w"), indent=1)
print("icons", ok)

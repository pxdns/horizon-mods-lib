#!/bin/sh
# Runs Mojang's data generator for each server jar and leaves <out>/<version>/reports/*.json.
# usage: run_generator.sh OUT_DIR   (needs Java 21 and Java 25: set JAVA21 and JAVA25 if they are not on the usual paths)
# It downloads the server jars from Mojang's own servers (checked against the SHA-1 in the version manifest) and runs them, so run it
# where running Mojang's jar is acceptable.
set -eu
out=${1:?out dir}
JAVA21=${JAVA21:-/usr/lib/jvm/java-21-openjdk-amd64/bin/java}
JAVA25=${JAVA25:-/usr/lib/jvm/java-25-openjdk-amd64/bin/java}
mkdir -p "$out"
manifest=$(curl -fsS https://piston-meta.mojang.com/mc/game/version_manifest_v2.json)
for v in 1.21.11 26.1 26.2 26.3; do
  [ -d "$out/$v/reports" ] && continue
  url=$(printf '%s' "$manifest" | python3 -c "import json,sys;print(next(x['url'] for x in json.load(sys.stdin)['versions'] if x['id']=='$v'))")
  info=$(curl -fsS "$url" | python3 -c "import json,sys;d=json.load(sys.stdin)['downloads']['server'];print(d['url'],d['sha1'])")
  mkdir -p "$out/$v" && cd "$out/$v"
  curl -fsS -o server.jar "${info% *}"
  echo "${info#* }  server.jar" | sha1sum -c -
  if [ "$v" = 1.21.11 ]; then java=$JAVA21; else java=$JAVA25; fi
  "$java" -DbundlerMainClass=net.minecraft.data.Main -jar server.jar --reports --output . >/dev/null
  rm -f server.jar; cd - >/dev/null
done

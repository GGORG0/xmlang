#!/usr/bin/env bash

cd "$(dirname "$0")" || exit 1

playground_url="https://xmlang.ggorg.xyz/"
playground_prefix="${playground_url}?owner=GGORG0&repo=xmlang&branch=main&file=examples/"

echo "# XMLang examples" > README.md
echo "" >> README.md
echo "Click the ⚙️ emoji next to each example to run it in the [XMLang Playground]($playground_url)." >> README.md
echo "" >> README.md

echo "| Example | Description |" >> README.md
echo "| ------- | ----------- |" >> README.md

for file in *.xml; do
    header=$(head -n 1 "$file" | sed 's/^<!-- //; s/ -->$//')
    title="${header%% - *}"
    description="${header#* - }"

    echo "| [⚙](${playground_prefix}${file}) [$title](./$file) | $description |" >> README.md
done

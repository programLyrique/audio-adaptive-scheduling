find ./target -wholename */new/raw.csv -print0 |   xargs -0 xsv cat rows > benchmark-data.csv

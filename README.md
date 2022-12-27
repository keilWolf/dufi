# DUFI - (Du)plicate (Fi)nder

This Project is aimed to find duplicated files by content
and provide and easy way to remove/thrash them.

## Getting Started

```bash
cargo build --release

./target/release/dufi ./tmp ~/Downloads
```

## Naive Approach - Shell

```bash
# remove bad charackters due to processing problems with native md5
for f in *; do mv "$f" $(echo $f | tr ' ' '_'); done

# create hashes and save them to file
for p in ./*;do md5 $p; done >> md5.txt

# search md5's for duplicates and create a new file with filenames
cat md5.txt | sort -k4n | uniq -f3 -D | awk -F '=' '{print $1}' >> duplicates.txt

# remove filenames you like to keep with an editor of your choice

# iterate and move them to a directory of your choice
while read -r line; do mv $line ../trash; done < duplicates.txt
```

## Authors

Wolle

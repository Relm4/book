#!/bin/bash

# This script checks all links in the markdown files 

RED='\033[0;31m'
NC='\033[0m' # No Color

function search_file() {
    link_lines=`cat $1 | grep https://`
    for line in $link_lines; do
        if [[ $line == *"https://"* ]]; then
            line=`echo $line | sed -e 's/.*https:/https:/g'`
            line=`echo $line | sed -e 's/[\)| |\`|"].*//g'`

            if [[ $line == *"https://crates.io"* ]]; then
                echo " ? Unkown:  $line (crates.io blocks curl)"
                continue
            fi

            code=`curl -s -o /dev/null -w "%{http_code}" "$line"`
            if [[ "$code" == "200" ]]; then
                echo " - Success: $line"
            elif [[ "$code" == "301" ]]; then
                echo " + MOVED:   $line"
            else
                echo -e " ! ${RED}FAILURE:${NC} $line (code: $code)"
            fi
        fi
    done
}

for FILE in `find src/ -name "*.md"`; do
    search_file $FILE
done
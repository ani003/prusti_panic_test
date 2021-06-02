#!/bin/bash
dir=${1:-'.'}
dir=${dir%/}

tmpfile="/tmp/null"
:> $tmpfile

for dirname in $dir/*; do
    test -d "$dirname" || continue
    dname=${dirname##*/}
    echo -n "${dname} : " >> $tmpfile

    infile="${dirname}/backtrace"
    ret=$(sed -n '/^thread/p' $infile | sed -n "s/.*', \(.*$\)/\1/p")
    if [[ -z $ret ]]
    then
        ret=$(sed -n '/^ right:/p' $infile | sed -n "s/.*', \(.*$\)/\1/p")
    fi
    echo "${ret}" >> $tmpfile
done

outfile="panic_sources.md"
echo $'# Panic sources\n' > $outfile

declare -i count=1
while read -r line; do
    prog=$(sed -n "s/^\(.*\) : .*$/\1/p" <<< $line)
    src=$(sed -n "s/.* : \(.*\)/\1/p" <<< $line)
    if grep -Fq $src $outfile
    then
        #echo "s/$src : /$src : $prog /"
        sed -i "s%^\(.*$src\)\(.*\)%\1\2\n  - ${prog}%" $outfile
    else
        echo -e "- [ ] ${count}. \`${src}\` :\n  - ${prog}" >> $outfile
        count=$(( count+1 ))
    fi
done < $tmpfile


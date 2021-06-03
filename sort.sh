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

tmpfile2="/tmp/null2"
:> $tmpfile2

while read -r line; do
    prog=$(sed -n "s/^\(.*\) : .*$/\1/p" <<< $line)
    src=$(sed -n "s/.* : \(.*\)/\1/p" <<< $line)
    if grep -Fq $src $tmpfile2
    then
        #echo "s/$src : /$src : $prog /"
        sed -i "s%^\(.*$src\)\(.*\)%\1\2\n${prog}%" $tmpfile2
    else
        echo -e "${src}\n${prog}" >> $tmpfile2
    fi
done < $tmpfile

declare -i count1=1
declare -i count2=0
while read -r line; do
    if grep -Fq ':' <<< $line
    then
        if [[ -d panics/$count1 ]]
        then
            rm -rf panics/$count1
        fi
        mkdir -p panics/$count1
        echo "Panic source: \`${line}\`" >> panics/$count1/README.md
        count1=$(( count1+1 ))
        count2=$(( count2+1 ))
    else
        cp -r $dir/$line panics/$count2/$line 
    fi
done < $tmpfile2

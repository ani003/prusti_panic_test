#!/bin/bash
prusti='/home/anirudh/Desktop/intern/prusti-dev/target/release/prusti-rustc'
dir='.'
files='true'

while test $# -gt 0; do
    case "$1" in
    -f|--files)
        files='true'
        shift
        ;;
    
    -d|--dirs)
        files='false'
        shift
        ;;
    
    *)
        if [[ $1 == "-"* ]]
        then
            echo "Invalid flag"
            exit 1
        else
            if [[ -d $1 ]]
            then
                dir=${1%/}
                shift
            else
                echo "Input directory '${1}' does not exist"
                exit 1
            fi
        fi
        ;;
    esac
done

if $files
then
    for filename in $dir/*.rs; do
        test -f "$filename" || continue
        fname=${filename##*/}
        echo $fname
        result=$($prusti $filename 2> >(grep -c "RUST_BACKTRACE=1"))
        
        if [[ $result -eq 0 ]]
        then
            rm $filename
        else
            fout=${fname%.*}
            mkdir -p panic_files/$fout
            mv $filename panic_files/$fout/$fname
            RUST_BACKTRACE=1 $prusti panic_files/$fout/$fname 2> panic_files/$fout/backtrace
            echo "Link to the problem : [Rosetta code](https://www.rosettacode.org/wiki/)" >| panic_files/$fout/README.md
        fi
    done
else
    for dirname in $dir/*; do
        test -d "$dirname" || continue
        dname=${dirname##*/}
        echo $dname

        if [[ -f $dirname/$dname.rs ]]
        then
            result=$($prusti $dirname/$dname.rs 2> >(grep -c "RUST_BACKTRACE=1"))
            if [[ $result -eq 0 ]]
            then
                echo "${dname}.rs not panicking anymore. Shifting to 'no_panic' directory"
                mkdir -p no_panic
                if [[ -d no_panic/$dname ]]
                then
                    rm -rf no_panic/$dname
                fi
                mv $dirname no_panic/$dname
            else
                RUST_BACKTRACE=1 $prusti $dirname/$dname.rs 2> $dirname/backtrace
            fi
        fi
    done
fi
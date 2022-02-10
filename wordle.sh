#!/bin/bash

atoz="abcedfghijklmnopqrstuvwxyzABCDEFGHIJKLLMNOPQRSTUVWXYZ"
atoZ="abcedfghijklmnopqrstuvwxyzABCDEFGHIJKLLMNOPQRSTUVWXYZ"
words=$(cat /usr/share/dict/wordle-words.txt)
# words=$(cat /usr/share/dict/wordle-words-ans.txt)
# w=$(cat /usr/share/dict/words)
# words=$(echo "${w,,}" | grep -e "^[$atoZ]\{5\}$")
# i=0
# for j in $(echo $words | sed -e "s: :\n:g")
# do
#     i=$(echo $i+1 | bc)
#     echo $i
# done
# echo $i
# ans=pombe
ans=$(echo $words | sed -e "s: :\n:g" | shuf | tail -n 1 )
check_word () {
    i=0
    hint="<< "
    for c in $(echo $1 | sed -e "s:\(.\):\1 :g");
    do
        j=$(echo $ans | grep -o -e "^.\{$i\}$c")
        k=$(echo $ans | grep -o -e "$c")
        if [ "$j" != "" ];
        then
            hint=$hint"!"
        elif [ "$k" != "" ];
        then
            hint=$hint"?"
        else
            hint=$hint"-"
        fi
        i=$(echo $i + 1 | bc)
    done
    echo $hint
    if [ "$hint" == "<< !!!!!" ]; 
    then
        exit 0
    fi
}

unchecked_char () {
    unchecked=$atoz
    for c in $(echo $1 | sed -e "s:\(.\):\1 :g");
    do 
        a="-"
        if [ "$(echo $ans | grep -o $c)" != "" ];
        then
            a="!"
        fi
        unchecked=$(echo $unchecked | sed -e "s:$c:$a:g")
    done
    echo $unchecked
}

check_chars=""
for try in `seq 1 6`;
do
    while true; do
        read -p $try\>\  word
        if [ "$(echo -n $word|wc -m)" != "5" ];
        then
            echo bad word
            echo $(unchecked_char $check_chars)
            continue
        fi
        if [ "$word" != "$(echo $words | grep -o $word )" ] ;
        then
            echo bad word
            continue
        fi
        check_chars=$check_chars$word
        check_word $word
        break
    done
done
echo $ans

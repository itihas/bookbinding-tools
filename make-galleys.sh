#/usr/bin/env bash

if [[ ${1: -4} == ".pdf" ]]; then
    echo "pdf fmt"
    pdf2ps $1 bare.ps
elif [[ ${1: -5} == ".djvu" ]]; then
    echo "djvu fmt; cleaning first."
    mkdir ./pages ./tiffs
    djvmcvt -i "$1" ./pages output-index.djvu
    ls pages/ | xargs -I {} ddjvu --format=tiff ./pages/{} ./tiffs/{}.tiff

    ddjvu --format=pdf "$1" bare.pdf
    pdf2ps bare.pdf bare.ps
fi

pstops -p a4 "16:7@.7(0w,0.5h)+8@.7(0.6w,0.5h)+5@.7(0,0)+10@.7(0.6w,0),9@.7(0w,0.5h)+6@.7(0.6w,0.5h)+11@.7(0w,0)+4@.7(0.6w,0),3@.7(0w,0.5h)+12@.7(0.6w,0.5h)+1@.7(0w,0)+14@.7(0.6w,0),13@.7(0w,0.5h)+2@.7(0.6w,0.5h)+15@.7(0w,0)+0@.7(0.6w,0)" \
       bare.ps galley_a6.ps

psselect -o galley_a6.ps galley_a6_odds.ps
psselect -e galley_a6.ps galley_a6_evens.ps

pstops -p a4 "32:15@.7(0w,0.5h)+16@.7(0.6w,0.5h)+13@.7(0,0)+18@.7(0.6w,0),11@.7(0w,0.5h)+20@.7(0.6w,0.5h)+9@.7(0w,0)+22@.7(0.6w,0),7@.7(0w,0.5h)+24@.7(0.6w,0.5h)+5@.7(0w,0)+26@.7(0.6w,0),3@.7(0w,0.5h)+28@.7(0.6w,0.5h)+1@.7(0w,0)+30@.7(0.6w,0)" \
       bare.ps | psnup -2 > galley_a7_odds.ps

pstops -p a4 "32:21@.7(0w,0.5h)+10@.7(0.6w,0.5h)+23@.7(0w,0)+8@.7(0.6w,0),17@.7(0w,0.5h)+14@.7(0.6w,0.5h)+19@.7(0w,0)+12@.7(0.6w,0),21@.7(0w,0.5h)+10@.7(0.6w,0.5h)+23@.7(0w,0)+8@.7(0.6w,0),17@.7(0w,0.5h)+14@.7(0.6w,0.5h)+19@.7(0w,0)+12@.7(0.6w,0)" \
       bare.ps | psnup -2 > galley_a7_evens.ps

ls galley_*.ps | xargs -I {} ps2pdf -sPAPERSIZE=a4 {}

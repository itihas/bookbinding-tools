#/usr/bin/env bash

if [[ ${1: -4} == ".pdf" ]]; then
    echo "pdf fmt"
    pdftops -paper A4 -expand $1 bare.ps
elif [[ ${1: -5} == ".djvu" ]]; then
    echo "djvu fmt; cleaning first."
    mkdir ./pages ./tiffs
    djvmcvt -i "$1" ./pages output-index.djvu
    ls pages/ | xargs -I {} ddjvu --format=tiff ./pages/{} ./tiffs/{}.tiff

    ddjvu --format=pdf "$1" bare.pdf
    pdftops -paper A4 -expand bare.pdf bare.ps
fi

WOFFSET="0w"			# horizontal offset of even pages.
HOFFSET="0h"			# vertical offset of ALL pages.
SCALE="1"			# scale pages up or down. this plays oddly with original page sizes - particularly badly when the source text is letter-size.

psbook -s32 bare.ps | psnup -2 > galley_32.ps

ps2pdfwr -sPAPERSIZE=a4 galley_32.ps

# pstops -pa5 "32:15@$SCALE(0,$HOFFSET),16@$SCALE($WOFFSET,$HOFFSET),17@$SCALE(0,$HOFFSET),14@$SCALE($WOFFSET,$HOFFSET),13@$SCALE(0,$HOFFSET),18@$SCALE($WOFFSET,$HOFFSET),19@$SCALE(0,$HOFFSET),12@$SCALE($WOFFSET,$HOFFSET),11@$SCALE(0,$HOFFSET),20@$SCALE($WOFFSET,$HOFFSET),21@$SCALE(0,$HOFFSET),10@$SCALE($WOFFSET,$HOFFSET),9@$SCALE(0,$HOFFSET),22@$SCALE($WOFFSET,$HOFFSET),23@$SCALE(0,$HOFFSET),8@$SCALE($WOFFSET,$HOFFSET),7@$SCALE(0,$HOFFSET),24@$SCALE($WOFFSET,$HOFFSET),25@$SCALE(0,$HOFFSET),6@$SCALE($WOFFSET,$HOFFSET),5@$SCALE(0,$HOFFSET),26@$SCALE($WOFFSET,$HOFFSET),27@$SCALE(0,$HOFFSET),4@$SCALE($WOFFSET,$HOFFSET),3@$SCALE(0,$HOFFSET),28@$SCALE($WOFFSET,$HOFFSET),29@$SCALE(0,$HOFFSET),2@$SCALE($WOFFSET,$HOFFSET),1@$SCALE(0,$HOFFSET),30@$SCALE($WOFFSET,$HOFFSET),31@$SCALE(0,$HOFFSET),0@$SCALE($WOFFSET,0)" \
#        bare.ps   > galley_2_to_1.ps
# psresize -p a4 galley_2_to_1.ps

# psselect -o galley_a6.ps galley_a6_odds.ps
# psselect -e galley_a6.ps galley_a6_evens.ps


# pstops -pa4 "16:7@$SCALE(0w,$HOFFSET)+8@$SCALE($WOFFSET,$HOFFSET)+5@$SCALE(0w,0h)+10@$SCALE($WOFFSET,0h),9@$SCALE(0w,$HOFFSET)+6@$SCALE($WOFFSET,$HOFFSET)+11@$SCALE(0w,0h)+4@$SCALE($WOFFSET,0h),3@$SCALE(0w,$HOFFSET)+12@$SCALE($WOFFSET,$HOFFSET)+1@$SCALE(0w,0)+14@$SCALE($WOFFSET,0h),13@$SCALE(0w,$HOFFSET)+2@$SCALE($WOFFSET,$HOFFSET)+15@$SCALE(0w,0h)+0@$SCALE($WOFFSET,0h)" \
#        bare.ps galley_a6.ps

# psselect -o galley_a6.ps galley_a6_odds.ps
# psselect -e galley_a6.ps galley_a6_evens.ps

# pstops -pa4 "32:15@$SCALE(0w,$HOFFSET)+16@$SCALE($WOFFSET,$HOFFSET)+13@$SCALE(0w,0h)+18@$SCALE($WOFFSET,0h),11@$SCALE(0w,$HOFFSET)+20@$SCALE($WOFFSET,$HOFFSET)+9@$SCALE(0w,0h)+22@$SCALE($WOFFSET,0h),7@$SCALE(0w,$HOFFSET)+24@$SCALE($WOFFSET,$HOFFSET)+5@$SCALE(0w,0)+26@$SCALE($WOFFSET,0h),3@$SCALE(0w,$HOFFSET)+28@$SCALE($WOFFSET,$HOFFSET)+1@$SCALE(0w,0h)+30@$SCALE($WOFFSET,0h)" \
#        bare.ps | psnup -2 > galley_a7_odds.ps

# pstops -pa4  "32:21@$SCALE(0w,$HOFFSET)+10@$SCALE($WOFFSET,$HOFFSET)+23@$SCALE(0w,0h)+8@$SCALE($WOFFSET,0h),17@$SCALE(0w,$HOFFSET)+14@$SCALE($WOFFSET,$HOFFSET)+19@$SCALE(0w,0h)+12@$SCALE($WOFFSET,0h),29@$SCALE(0w,$HOFFSET)+2@$SCALE($WOFFSET,$HOFFSET)+31@$SCALE(0w,0h)+0@$SCALE($WOFFSET,0h),25@$SCALE(0w,$HOFFSET)+26@$SCALE($WOFFSET,$HOFFSET)+27@$SCALE(0w,0h)+4@$SCALE($WOFFSET,0h)" #  bare.ps | psnup -2 > galley_a7_evens.ps
#        # TODO fix this mess, unsure if it renders properly. shuddering to think of what happened to the galley I actually bound off of this.

# ls galley_*.ps | xargs -I {} ps2pdf -sPAPERSIZE=a4 {}

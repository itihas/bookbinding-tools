#/usr/bin/env bash

pdf2ps Downloads/Baudelaire,\ Charles_\ Baudelaire,\ Charles\ -\ Les\ fleurs\ du\ mal\ _\ the\ complete\ text\ of\ The\ flowers\ of\ evil-David\ R.\ Godine\ \(1983\).pdf

pstops -p a4 "16:7@.6(0w,0.5h)+8@.6(0.5w,0.5h)+5@.6(0,0)+10@.6(0.5w,0),9@.6(0w,0.5h)+6@.6(0.5w,0.5h)+11@.6(0w,0)+4@.6(0.5w,0),3@.6(0w,0.5h)+12@.6(0.5w,0.5h)+1@.6(0w,0)+14@.6(0.5w,0),13@.6(0w,0.5h)+2@.6(0.5w,0.5h)+15@.6(0w,0)+0@.6(0.5w,0)" \
       Baudelaire,\ Charles_\ Baudelaire,\ Charles\ -\ Les\ fleurs\ du\ mal\ _\ the\ complete\ text\ of\ The\ flowers\ of\ evil-David\ R.\ Godine\ \(1983\).ps fleurs_du_mal_a6_tops.ps

pstops -p a4 "32:15@.6(0w,0.5h)+16@.6(0.5w,0.5h)+13@.6(0,0)+18@.6(0.5w,0),11@.6(0w,0.5h)+20@.6(0.5w,0.5h)+9@.6(0w,0)+22@.6(0.5w,0),7@.6(0w,0.5h)+24@.6(0.5w,0.5h)+5@.6(0w,0)+26@.6(0.5w,0),3@.6(0w,0.5h)+28@.6(0.5w,0.5h)+1@.6(0w,0)+30@.6(0.5w,0)" \
       Baudelaire,\ Charles_\ Baudelaire,\ Charles\ -\ Les\ fleurs\ du\ mal\ _\ the\ complete\ text\ of\ The\ flowers\ of\ evil-David\ R.\ Godine\ \(1983\).ps | psnup -2 > fleurs_du_mal_a7_tops_odds.ps

pstops -p a4 "32:21@.6(0w,0.5h)+10@.6(0.5w,0.5h)+23@.6(0w,0)+8@.6(0.5w,0),17@.6(0w,0.5h)+14@.6(0.5w,0.5h)+19@.6(0w,0)+12@.6(0.5w,0),21@.6(0w,0.5h)+10@.6(0.5w,0.5h)+23@.6(0w,0)+8@.6(0.5w,0),17@.6(0w,0.5h)+14@.6(0.5w,0.5h)+19@.6(0w,0)+12@.6(0.5w,0)" \
       Baudelaire,\ Charles_\ Baudelaire,\ Charles\ -\ Les\ fleurs\ du\ mal\ _\ the\ complete\ text\ of\ The\ flowers\ of\ evil-David\ R.\ Godine\ \(1983\).ps | psnup -2 > fleurs_du_mal_a7_tops_evens.ps

ps2pdf fleurs_du_mal_a7_tops_evens.ps

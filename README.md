# Advanced Ascii Renderer
(abbreviated as 'aar')

This is a command line tool that will turn an image or video into ascii or other text.

With edge detection and gpu acceleration as "advanced" features.

For a significant part inspired by [this video by acerola](https://www.youtube.com/watch?v=gg40RWiaHRY)

Usecases could include:
- Needing to get an idea of what an image contains when in an enviorment where you only have the terminal like when you are ssh'd into a machine.
- Sending low resolution images in Discord servers where you dont have image permisions.
- Fun.

## example
The primeagens profile picture
```
$ cargo run ~/Pictures/prime.jpeg --set ascii --height 40

"""""""""""""""oooooooooooooooooooooooooooooooooooooooooooooooooooooo+""""""""""""""""""
"""""""""""""""|oooooooooooooooooooooooooooooooooooooooooooooooooooo/"""""""""""""""""""
"""""""""""""""|oooooooooooooooooooooooooooooooooooooooooooooooooooo/"""""""""""""""""""
""""""""""""""""ooooooooooooooooooooooooooooooooooooooooooooooooooo/""""""""""""""""""""
""""""""""""""""|oooooooooooooooooooo----------------+------oooooo/"""""""""""""""""""""
""""""""""""""""|oooooooooooooooo---"".-"-"+ooo+++++""--+--+--\ooo/"""""""""""""""""""""
""""""""""""""""|oooooooooooooo//--....-\----"------+----o+o-""/--""""""""""""""""""""""
""""""""""""""""|ooooooooooooo//.. -....-."---.--.""+oooo-oo+\--\-""""""""""""""""""""""
"""""""""""""""""+oooooooooooo..  ........... ......\-\+o+\--\"+-o""-"""""""""""""""""""
"""""""""""""""""|oooooooooo-/.......    ....... ..."-|o\"-++"----++"-""""""""""""""""""
""""""""""""""""""ooooooooo/.......   ..  ............-\\o++++-"..\-+/""""""""""""""""""
""""""""""""""""""|oooooooo...          .... ...-.....-.\-++-"\"-./"+-""""""""""""""""""
""""""""""""""""""|ooooooo/....      .---..........-""-------+--"-"+o+""""""""""""""""""
""""""""""""""""""|ooooooo".    .  ...."""""........|----o-/?-oo++"---""""""""""""""""""
""""""""""""""""""|ooooooo/.     ......"""""""-------+/-o??oo--\-"+"""""""""""""""""""""
""""""""""""""""""|oooooo+..    .....""""""""--------???-??#####--++""""""""""""""""""""
"""""""""""""""""""ooooo//-.  ...."""""""""-""--\????????#######??""""""""""""""""""""""
"""""""""""""""""""|ooo"/""\... |""""""""-----""+\?/\o\?????####?/""""""""""""""""""""""
"""""""""""""""""""\ooo+\""-..  /"""+""/......-----"|?+--\???????.""""""""""""""""""""""
"""""""""""""""""""+oooo"..""\.|""""""""""--....."""|+----------o-""""""""""""""""""""""
""""""""""""""""""""|oooo"-"++""++""""""""""......|-?+--------/?+-""""""""""""""""""""""
""""""""""""""""""""|oooo\"-++""++++""""""""""""""|?#?------????"/""""""""""""""""""""""
""""""""""""""""""""|ooooo"""""""++++"""""""""""""|?#????????#??."""""""""""""""""""""""
""""""""""""""""""""|oooooo"""""++++++""""""""""""||#???????#?//""""""""""""""""""""""""
"""""""""""""""""""""ooooo/""""++++++++"""""."+--"||???o??????/"""""""""""""""""""""""""
"""""""""""""""""""""|----"""""+++"++++//.......--++o??o?????/""""""""""""""""""""""""""
"""""""""""""""""-""-""\--"""""+++++++"..-------+-|-\---????/"""""""""""""""""""""""""""
""""""""""""""""/o-.|"+"\..""""++"+++""-.--"+""""---+-o-\???/"""""""""""""""""""""""""""
"""""""""""""""/ooo+--""+\-.""""+++++""""+----------.-\//?-/""""""""""""""""""""""""""""
""""""""""""""|++++++o+-"++---.--"+++""""""+"+----???--?--"""""""""""""""""""""""""""--o
"""""""""""""-ooo+++++o+o\"+++\\.."-+"""""""------/????/""""""""""""""""""""""""""""/ooo
""""""""--+++++++oooooooooo\""++\-..\-+++-""\oo-????--"""""""""""""""""""""""""""--ooooo
"""""--+++++++++++++oooooooo-\"+++"-."""++++++++--/-"""""""""""""""""""""""""""--ooooooo
""--+++++++++++++++ooooooooooo\"++---"++++--------?-"""""""""""""""""""""""""--ooooooooo
++++------------o++oooooooooooo--++oo--+-ooo??/o????\"""""""""""""""""""""--oooooooooooo
?-??##?#########---oooooooooo???--\+ooo\?\o???oooo??/o""""""""""""""""""/-oooooooooooooo
?#????#############---------oo????#-+\o?o-#??ooo--??#"""""""""""""""""--oooooooooooooooo
?#??######################?????????#\++\o?o--\ooo?\###-\""""""""""""--oooooooooooooooooo
???#######################???????????\\+oo///|?o-o\\###\\"""""""""/ooooooooooooooooooooo
???########################??????????##--/-----#--#-\####\""""""-ooooooooooooooooooooooo
```

## note
for larger resolution videos the non-release build is too slow, just use `cargo run --release` and then it can keep up easily

## current features
- character sets, current options braile, numbers and ascii
- changing the width and height, if one is given the other is calculated to match aspect ratio of image, if none is given it tries the biggest that will fit your terminal
- edge detection using the sobel operator
- ansi rgb color (use the 'rgb' color set)
- play video (with audio)
- show your webcam (on linux with `cargo run /dev/video0 --format v4l2 --media-mode video`)

## todo (in rough order, not all equally plausible)
- improve line detection to be able to place `>`, `<`, `(` and `)` when that fits the detected edge
- make edge detection only work on edges you cant already see, its useless to show edges you can see by contrast already, use it for the teeny tiny edges
- adjustable line 'thickness' and other parameters
- upper half block unicode character, with background character color
- add color quantization [acerola will save us again](https://www.youtube.com/watch?v=fv-wlo8yVhk)
- loop video
- on full cpu backup mode for if the user doenst have a GPU
- resizing on terminal resize
- interactive mode, seeing changes in
- recognizing text and making that visible

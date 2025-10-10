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
$ imageToText ~/Pictures/prime.jpeg --set ascii --height 80

"""""""""""""|oooooooooooooooooooooooooooooooooooooooooooooooo|"""""""""""""""""
""""""""""""""ooooooooooooooooooooooooooooooooooooooooooooooo|""""""""""""""""""
""""""""""""""ooooooooooooooooooooooooooooooooooooooooooooooo"""""""""""""""""""
""""""""""""""|ooooooooooooooooooooooooooooooooooooooooooooo|"""""""""""""""""""
""""""""""""""|ooooooooooooooooo---"""""""++++++o+++"-oooooo""""""""""""""""""""
""""""""""""""|oooooooooooooo/."...""""+oo++oo++"++++-+.\oo|""""""""""""""""""""
"""""""""""""""ooooooooooooo/. ........."""""""+oooo++\""+""""""""""""""""""""""
"""""""""""""""|oooooooooo||.. " ............."+oooo-ooooo\"""""""""""""""""""""
"""""""""""""""|oooooooooo|..  . ..............."++o+oo+"+o+"+""""""""""""""""""
"""""""""""""""|ooooooooo|.. ...        .. ....""\o+"--oo+--+++"""""""""""""""""
""""""""""""""""ooooooooo......   .. ........"...."o+""""...|+|"""""""""""""""""
""""""""""""""""|ooooooo|..    .    .............".|"++"""./+++"""""""""""""""""
""""""""""""""""|oooooo|....  . ..............""+++++oooo+++"-+"""""""""""""""""
""""""""""""""""|oooooo|..    ....."""""......"+oooo???oo""+""""""""""""""""""""
"""""""""""""""""|ooooo|..    ....""""""""""+++o???????##?\++"""""""""""""""""""
"""""""""""""""""|ooooo|.    ...""""""""++\o???#????#####??|""""""""""""""""""""
"""""""""""""""""|ooo/""| .. ."""""""""oo""+\?o?o??######?||""""""""""""""""""""
"""""""""""""""""|ooo"."+". ."""+""""....."++o"|||?????###|"""""""""""""""""""""
""""""""""""""""""ooo|""."...""""""."......."""||++--o????""""""""""""""""""""""
""""""""""""""""""|ooo|".+"""+"""""""""""...."/?|....".+o|/"""""""""""""""""""""
""""""""""""""""""|oooo++""""++""""""""""""""|??\++"+????|."""""""""""""""""""""
""""""""""""""""""|oooo|"""""++++""""""""""""|??##????#?#|""""""""""""""""""""""
""""""""""""""""""|ooooo\""""+++++"""""""""""|??????????|"""""""""""""""""""""""
"""""""""""""""""""ooooo."""++++++"""""".""""|??????????|"""""""""""""""""""""""
"""""""""""""""""""|oo-|""""+++++++""".....-++\-??o????|""""""""""""""""""""""""
"""""""""""""""""""-" ."""""++++++"....... /"|+???????|"""""""""""""""""""""""""
"""""""""""""""/\"."+"\.."""++++++"..\+""".".\"+ooo??||"""""""""""""""""""""""""
""""""""""""""ooo+o""+"".."""+++++"""++"..."+"..\"|??""""""""""""""""""""""""""/
"""""""""""""++++++o+""++\..""+"+"""""+""+-oo????o?/"""""""""""""""""""""""""/oo
""""""""""""|+o++++++o+"++"\..."++""""""""""+?????/"""""""""""""""""""""""""oooo
""""""""+++++++ooooooooo\"+++\...\++"""+ooo????/"""""""""""""""""""""""""/oooooo
""""./++++++++++++oooooooo""++""."""+++++++oo-"""""""""""""""""""""""""/oooooooo
""+++++++++++++++oooooooooo\"+++""+++++|??????\""""""""""""""""""""""/oooooooooo
+++o++oo??oo+oo++oooooooooooo+++oo++oooo??oo???|"""""""""""""""""""/oooooooooooo
???????#######?oo+oooooooo????\+ooo?#oo??oooo??-|"""""""""""""""""/ooooooooooooo
##???#############ooo???#?o???#\++o???#?|ooo???|"""""""""""""""/oooooooooooooooo
##?####################????????##|+o???/?ooo?\##\\"""""""""""/oooooooooooooooooo
???####################??????????#++o?|.|?o/o||###\"""""""""oooooooooooooooooooo
??######################?????????##\o|--\###?#o####||""""/oooooooooooooooooooooo
#####################??##??????#?###?\\...\\?#?o#####\"/oooooooooooooooooooooooo
```

## current features
- character sets, current options braile, numbers and ascii
- changing the width and height, if one is not given the other is calculated to try and match aspect ratio of image
- edge detection using the sobel operator
- ansi rgb color (use the 'rgb' color set)
- play video (with audio)
- show your webcam (on linux with `aar /dev/video0 --format v4l2 --media-mode stream`)

## todo (in rough order)
- re add sobel edge directions as well
- improve line detection to be able to place `>`, `<`, `(` and `)` when that fits the detected edge
- make edge detection only work on edges you cant already see, its useless to show edges you can see by contrast already, use it for the teeny tiny edges
- improve video file intake performance (probably just downscaling the frame)
- upper half block unicode character, with background character color
- improve color quantization [acerola will save us again](https://www.youtube.com/watch?v=fv-wlo8yVhk)
- adjustable line 'thickness' and other parameters
- loop video
- on full cpu backup mode for if the user doenst have a GPU
- recognizing text and making that visible
- resizing on terminal resize
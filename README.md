# ImageToText

This is a simple command line tool that will turn an image into text.

Usecases could include:
- Needing to get a gist of what an image contains when in an enviorment where you only have the terminal like when you are ssh'd into a machine.
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
- line detection using the sobel operator

## todo (in order)
- getting colors
- match aspect ratio better by getting data from the terminal
- improve the line detection [like done in this cool video](https://www.youtube.com/watch?v=gg40RWiaHRY)
- take in videos or streams to display video

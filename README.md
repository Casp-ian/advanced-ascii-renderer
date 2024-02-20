# ImageToText

This is a simple tool that will turn a image into text.

## example 
steam boat mickey
```
imageToText ~/Pictures/micky.jpg --set braile --width 120
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⢕⡈⠀⠀⠀⣿⢕⣿⣿⣿⣫⣿⣿⣫⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⡈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣿⢕⣫⣫⠀⢕⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⣿⣿⣿⣿⣫⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⢕⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣫⣫⣿⣫⣿⣿⣫⣿⣫⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⢕⡈⣫⣫⢕⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣫⡈⢕⣿⣿⣿⢕⠀⠀⠀⣿⣿⣿⣫⠀⠀⠀⠀⠀⠀⣿⢕⣫⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⡈⣫⣿⣿⠀⣿⣿⣿⣫⠀⣿⣿⣿⣿⣿⠀⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⡈⡈⠀⠀⠀⠀⡈⢕⣿⢕⣿⣫⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⡈⣫⣿⣫⣫⢕⢕⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⢕⢕⣿⣿⣿⣿⢕⢕⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⠀⣿⣿⢕⢕⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣿⣿⣫⢕⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⡈⡈⠀⠀⠀⠀⠀⡈⢕⣫⣫⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⠀⣿⣿⠀⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⢕⢕⡈⢕⣫⡈⣫⠀⣿⢕⢕⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣫⢕⣫⣫⣿⣿⡈⡈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣫⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⠀⠀⠀⠀⣫⡈⣫⣿⢕⡈⣫⣿⣿⣫⣫⣿⣫⢕⣫⣫⣿⢕
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⣿⢕⣫⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⡈⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⡈⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⠀⣿⣿⠀⣿⣿⣿⣿⣿⣿⡈⣿⢕⠀⣿⣿⣿⢕⢕⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⡈⠀⠀⢕⣿⣿⣿⢕⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣫⣿⣿⣫⣿⣿⣿⣫⣫⣿⢕⣿⣿⣿⣿⣿⣫⣫⣿⣿⣿⠀⣿⣿⣿⢕⣿⡈⣿⣿⣿⠀⣫⣿⣫⣫⠀⢕⡈⡈⡈⠀⠀⠀⠀⡈⢕⡈⣫⣿⡈⣿⢕⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣫⢕⣿⣿⣿⣿⣿⣫⣫⣿⣫⣿⣿⣿⣫⣿⣿⣿⣿⣿
⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣫⣿⣿⣿⢕⣿⣿⣿⣿⣿⠀⣫⣫⡈⡈⣫⣫⣫⣫⠀⠀⠀⠀⠀⠀⡈⡈⣫⢕⢕⠀⠀⢕⣿⣿⢕⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⡈⡈⠀⣿⣿⢕⣿⣿⣫⡈⣫⢕⡈
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⡈⣿⣿⣿⢕⣿⣿⣿⠀⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⣿⢕⠀⠀⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣫⢕⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣫⣫⣿⢕⣫⠀⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⠀⣿⣿⣿⠀⣿⣿⣿⣫⣿⣿⣿⠀⣿⣿⣿⣿⣫⣫⣫⣿⣿⣿⢕⠀⠀⠀⣿⣿⣿⣿⣿⣫⣿⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣫⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⡈⣿⣿⣿⣿⠀⣿⣿⣿⠀⣿⣿⣿⣿⣫⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣫⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⠀⣫⣿⣿⣿⣿⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣫⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⡈⣿⣿⣿⡈⣿⠀⣿⣿⣿⢕⣿⣿⣿⣿⣿⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⠀⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⠀⢕⣿⣿⣿⡈⣫⢕⡈⠀⢕⡈⣫⠀⢕⡈⣫⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⡈⠀⠀⠀⠀⠀⠀⠀⠀⣿⣫⣫⣫⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⡈⣿⣿⡈⣿⣫⣿⣿⣿⣿⠀⣿⣿⣿⢕⣫⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⡈⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣫⣿⣫⣿⢕⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⡈⣿⣿⣿⢕⠀⣿⢕⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣫⣫⠀⢕⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⢕⣿⣫⣫⣿⣿⣿⣿⣫⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⣿⣿⣿⡈⣿⣿⡈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀
⣿⣿⣿⣿⣿⣿⣿⢕⣫⣿⣿⣿⣿⣫⣿⢕⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣫⠀⠀⡈⣿⣫⣿⣿⣿⣫⡈⢕
⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⡈⡈⣫⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣫⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣫⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⢕⣿⣿⣿⣿⢕⣿⠀⣫⣿⣿⠀⣫⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣫⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⢕⠀⡈⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣫⣿⣿⢕⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⡈⣫⣿⠀⣿⣿⣿⣿⣫⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⡈⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣫⡈⠀⢕⣿⠀⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⠀⠀⣿⣿⣿⠀⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⡈⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⡈⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡈⣫⡈⡈⡈⢕⡈⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⠀⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫
⣿⣿⣿⣿⣿⣿⣿⣿⢕⡈⡈⠀⠀⢕⢕⢕⢕⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⡈⣿⣿⡈⣿⣿⣿⣿⣿⣿⠀⢕⢕⡈⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡈⡈⡈⡈⡈⡈⡈⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣫⣿⣿⣿⣿⣿⡈⣿⣿⣿⢕⣿⣿⣿⠀⡈⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⢕⣿⣿⣿⣿⣿⣫⢕⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⡈⢕⡈⡈⠀⢕⡈⡈⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⢕⣿⣿⣿⣿⣿⣿⢕⡈⣫⡈⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⡈⠀⠀⠀⠀⢕⡈⣫⠀⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⠀⣿⣿⣿⠀⣿⣿⣿⢕⣿⣿⣿⣿⠀⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⢕⢕⠀⠀⠀⡈⣿⠀⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⡈⡈⠀⠀⠀⠀⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⠀⣫⣿⣫⠀⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⠀⢕⠀⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣿⣫⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢕⠀⣫⣿⠀⣫⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⠀⠀⣫⡈⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⠀⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⢕⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⠀⣿⣿⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⣫⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⡈⠀⢕⠀⣿⣿⠀⠀⠀⠀⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⠀⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⡈⠀⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⣿⣿⣿⢕⠀⠀⠀⠀⠀⡈⣫⣫⢕⣿⣿⣿⣿⣿⣿⣿⣿⡈⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣫⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⢕⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⠀⠀⣿⣫⣫⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣫⣿⣿⢕⣿⣫⣿⠀⣫⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⠀⣿⣿⣿⣿⢕⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣫⡈⠀⢕⣫⣿⣿⣿⣿⣿⣿⣫⡈⠀⠀⣿⣿⣿⣿⣿⣿⠀⣿⣿⠀⣿⣿⡈⠀⣿⣿⣿⣿⣿⣿⣿⣿⣫⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⢕⣿⣫⣿⣿⣫⣫⣿⣿⣫⣫⣫⣿
⣿⣿⣿⣿⣿⣿⣿⢕⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡈⣿⣿⣫⢕⢕⠀⢕⣫⣿⣫⣿⡈⠀⠀⠀⠀⣫⣿⣫⣿⣿⣿⣫⠀⡈⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⣿⣫⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣫⠀⣿⣿⠀⣫⣿⡈⡈⣫⢕⠀⣫⠀
⣿⣿⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣫⣿⣿⣫⣿⣿⣿⣿⢕⣿⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⣿⣫⠀⡈⣿⢕⡈⣿⣿⢕⣿⣿⣫⣿⣿⣿⣿⣿⢕⣿⣿⣿⣫⣫⠀⡈⡈⢕⣿⠀⠀⣿⣿⣿⣫⢕⠀⠀⢕⢕⢕⣫⣫⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢕⣿⠀⢕⣿⣿⣿⡈⣿⣿⣫⠀⡈⠀⢕⠀
```

## current features
- character sets, current options braile and filled (using '/' and '#')
- colors, currently only 2 green-blue and blue-purple
- changing the width, height is calculated to try and match aspect ratio of image

## todo
- make the matching of aspect ratio more accurate or add possibilty to manually set height anyways
- better and more colors
- some simple image processing (convolution maybe) to get clearer results
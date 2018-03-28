# md5-bmp
A simple tool to convert text into a bitmap representation of its md5 checksum.

## Example:
The following image has been generated with the string "ElectricCoffee's MD5-BMP Tool",
which generated the following checksum: `914329dd9233d2974886fe02ebc73dab`.

This checksum has then been fed into the algorithm to produce this image:

![img.png](https://github.com/ElectricCoffee/md5-bmp/blob/master/img.png)

## How to Use
The command structure looks like this: `md5-bmp <height> <text> <filename>`
* Height is the image's height (and width) in pixels
* Text is the text you want to convert to an image
* Filename is the file you want it to save to.
  * Note that since the tool outputs bitmaps, it's recommended to give the file a .bmp extension.
  
For example, typing in `md5-bmp 512 "hello world" hello-world.bmp` will generate this image:

![hello-world.png](https://github.com/ElectricCoffee/md5-bmp/blob/master/hello-world.png)

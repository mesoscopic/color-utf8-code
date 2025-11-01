This is a simple Rust program that encodes and decodes UTF-8 messages into images.

It was made in response to a friend posting messages encoded in this format.

The encoder works by drawing square "blocks", left to right, top to bottom, each of which contains three bytes from a message, stored in the Red, Green, and Blue channels.

The decoder works by interpreting those blocks as UTF-8 text.

## btoc

Binary TO Color (named like the JS function btoa)

Takes a message from an argument (if `-m` or `--message` is passed) or from stdin, and encodes it into a provided file.

The file format is determined by the extension given, but you should probably choose a lossless format.

```sh
cat message.txt | ./btoc -s 10 message.png
```

### Options

`-s` or `--size`: The size of each block in pixels. `1` by default.

`-w` or `--width`: The width of the resulting image in **blocks**. `64` by default. If the width is 64 and the block size is 10, for example, the image will be 640 pixels wide.

`-m` or `--message`: A string to use for the message instead of reading stdin.

## ctob

Color TO Binary (named like the JS function atob)

Takes a provided file (or URL if `-w` or `--web` is passed), and decodes it into stdout.

```sh
./ctob -s 10 message.png > received.txt
```

```sh
./ctob -ws 10 https://raw.githubusercontent.com/mesoscopic/color-ascii-code/master/example.png
```

### Options

`-s` or `--size`: The size of each block in pixels. `1` by default.

`-w` or `--web`: Treats the path given as a URL instead of a local file, downloading the image from there.

A minimalistic lib that reduces text size by replacing specific character sequences with single Unicode characters.
On average, this compresses text by ~5%, with a variation ~5%.

One neat feature of this lib: * and ° look pretty similar yet usually only °C would normally be convertable into ℃. Using aliases, this limitation is no more as *C, ˚C, ᴼC, and ºC will all be replaceable by ℃
A neat side effect is that AI currently struggles with understanding the insertion of "random" unicodes, even when they are visually equivalent to the characters they replace
# Perlin Builder

Generates 2d terrain and heightmap jpeg using Perlin noise.

Probably a good place to look in for anyone starting with Nannou.

I wrote this project to learn about Nannou and Rust. Constructive criticism is more than welcome.

## Controls

- 'O' && ( '+' || '-' ) : Adjust the octaves.
- 'Z' && ( '+' || '-' ) : Zoom in and out
- 'W' && ( '+' || '-' )  - 'H' && ( '+' || '-' ) : Adjust the image dimensions.
- 'S' Save the image to the current directory.
- 'R' Get a new random seed.
- 'M' Toggle between the two modes, colored terrain or heightmap.

## Some examples

#### A typical view when the program starts:
![Example 6](/assets/perlin_s5.410421842378751_o16_163.jpeg)

#### The same view with 4 octaves:
![Example 5](/assets/perlin_s5.410421842378751_o4_165.jpeg)

#### And with 1 octave:
![Example 4](/assets/perlin_s5.410421842378751_o1_194.jpeg)

#### An heightmap with 12 octaves:
![Heightmap](/assets/perlin_s1.6384_o12_86.jpeg)

#### Zoomed out:
![Example 2](/assets/perlin_s1.6384_o12_159.jpeg)

#### Zoomed out with 1 octave:
![Example 3](/assets/perlin_s1.134647698758828_o1_145.jpeg)




# Designing a (candlestick) chart

## Requirements:

- a grid system with 
  - x as a timeline
  - y as a price

[SVG](https://developer.mozilla.org/en-US/docs/Web/SVG)
[Window](https://www.geeksforgeeks.org/window-to-viewport-transformation-in-computer-graphics-with-implementation/)

## Grid system

Grid system description

```
        Q  |
        u  |  ymin                     ymax
        o  |     _|___________________|_
        t  |      |                   |
        e  |      |                   |   Window is 
           |      |                   |   - resizable
           |      |                   |   - translatable
      ...  |      |                   |   by the user
  0.98023  |     _|___________________|_
  0.98020  |  xmin                     xmax
           |            Window
    f32  __|__________________________________
           |  1 2 3 4 ....
                        Timeline
          u32
                        World/Chart
```

Window transform to viewport

```
ymin                     ymax
   _|___________________|_ 
    |                   |           .________________________.
    |                   |         h |                        |
    |                   |         e |                        |
    |                   |         i |                        |
    |                   |         g |                        | 
    |                   |         h |                        | 
    |                   |         t |                        | 
    |                   |           |________________________|
    |                   |                     Width
   _|___________________|_                  Viewport
xmin                     xmax
            Window
```

## Reponsability - Draft

App spawn 
- a chart.
  - chart is the "world"
  - chart provides information as wanted

App spwan
- a layer
  - layer have a context (where to draw)

App spawn
- a window.
  - window can convert coordinate to viewport

App ask graphics from chart given a window (clipping issue?)
- Line
- Bar
- Candle

Chart hold 
objects
Indicators

App ask layer to draw graphics
Chart is the world and data holder
It should have the begining timestamp + the timeframe
It should provide information only
much like a db
Maybe it is not the good naming
DataStore?
History?

Other part should build Graphics based on information

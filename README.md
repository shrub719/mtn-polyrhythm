# polyrhythm

A multipurpose command line conversion tool for metronome.

## Installation

```cargo install mtn-poly```

## Usage

- `mtn-poly compile {input}`: compile editable `.mtn` file to binary `.mtb` file
    - `-o {output}`: pass in location of output
    - `-s {offset}`: start time in ms. Used for testing
- `mtn-poly osu {input}`: convert a `.osu` file (from inside a compressed `.osz` archive) to a `.mtn` map file
    - `-o {output}`: pass in location of output
    - `-s {offset}`: start time in ms. Used for testing

## Making maps

`.mtn` files can be directly written to using a text editor. All `.mtn` files have to start with
a header, for example:

```
// title: My Beautiful Song
// artist: shrub719
// id: shrub719_my_beautiful_song
// bpm: 120
```

The **title** and **artist** show up in the game menu, and the **ID** should be as unique as
possible as it is used to save the high scores for your map.
The **BPM** is either a float value or `ms`, which decides the **timing mode** of your map file.

If the **timing mode** is `ms`, all time values in your file will be treated as milliseconds after
the start of the map.  
If the **timing mode** is a number, all time values should be in the format
`[measure]:[beat]`. For example, `4:3` represents beat 3 in measure 4,
and `0:2.5` represents beat 2 and a half in the opening measure.

The rest of the file consists of **items** in the format `[code] [time] [arguments]`.

The following **items** are accepted by polyrhythm:

- `t [time] [x]`: tap note.  
  `[x]`: the x position of the note from 0.0 to 1.0.

- `h [time] [x] [end time]`: hold note.  
  `[x]`: the x position of the note from 0.0 to 1.0.  
  `[end time]`: the time value of the end of the hold note.

- `e [time] [colour]`: background colour change.  
  `[colour]`: a triplet of RGB values, e.g. `255 0 255` for magenta.

- `e-fade [time] [start colour] [end colour] [n] [end time]`: background colour fade.  
  `[start colour]`: a triplet of RGB values; the colour to start the fade from.  
  `[end colour]`: a triplet of RGB values; the colour to fade to.  
  `[n]`: the number of colour changes to use when fading. A higher `n` results in a
  smoother fade, but possibly worse performance. Around 16 changs per second are recommended.  
  `[end time]`: the time when the fade should end.  

Comments can be added by preceding a line with a `#`.

Alternatively, you can create the map in OSU:mania and convert it using polyrhythm, only
editing the `.mtn` file if you want more precise x-coordinate control than Osu lanes
allow, or you want to use visual effects.


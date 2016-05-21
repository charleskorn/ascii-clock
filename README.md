# ascii-clock

The May Shokunin challenge:

> In this challenge you must draw an analogue clock face.
>
> - The time is supplied on stdin in the format hh:mm, followed by a single newline. Both hh and mm are padded with a leading 0 (zero) if they are less > than 10.
> - The analogue clock face representing that time, subject to the following rules, should be written on stdout.
> - Each hour mark should be drawn with 'o' (Lower-case O).
> - The hour mark representing the hour given in the input should be drawn with an 'h'.
> - The hour mark representing the minute given in the input should be drawn with an 'm'.
> - If the hour and the minute both fall on the same mark, then you should draw it with an 'x'.
> - You should round down the minutes past the hour to the nearest 5 for the purposes of marking it on the clock (so 23 becomes 20, 39 becomes 35).
>
> Examples:
>
> 21:35
>
> ```
>         o
>     o       o
> 
>  o             o
>
> h               o
>
>  o             o
>
>     m       o
>         o
> ```
>
> 04:59
>
> ```
>         o
>     m       o
>
>  o             o
>
> o               o
>
>  o             h
>
>     o       o
>         o
> ```

## Prerequisites

* Rust 1.6.0

Run `./go.sh check` to test that everything looks OK.

## Running

* Run `./go.sh run` to run the application (as per the requirements, you'll be prompted to enter the time you'd like to display)

## Testing

* Run `./go.sh test` to run the tests

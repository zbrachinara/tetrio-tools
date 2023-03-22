# Viewtris

Viewtris aims to be an alternative replayer for block stacking games such as tetrio and jstris. It
aims to satisfy three (related) goals:

1. Performance: On tetrio, for example, a game's general statistics (win/lose, average apm, etc.)
   are easy enough to view on the profile page at ch.tetr.io. However, to view the replay itself,
   the game has to be launched in order to simulate the actions taken by each player.
2. Simplicity (**NOT *NECESSARILY* IN GRAPHICS!**): Replays are often expressed in terms of inputs.
   This makes it phenomenally easy to not only replay the game, but to give metadata (for instance,
   when an all-spin occurs). It also makes it slightly difficult for anything apart from the game to
   know what is going on, as simulation of the game is required to read the replay. Viewtris
   attempts to express the game in terms of what you see, while also (WIP) preserving the metadata
   that an ordinary tetris game relies on.
3. Code-accessibility: Right now this library is just in rust, but I also want to make it accessible
   from python and some other popular languages (csharp, browser and node js, jvm-based, etc). In
   addition, I want to make certain common actions automatable, such as generating replay videos and
   censoring names

Secondary goals include customizability of the view, generality, and accessibility from web.

# Contributions

Replay data from these players has been used to test and develop the project. Without these people,
this project would essentially be meaningless:

* hahahaki
* awsum
* @vacuus (tetrio)
* zbrachinara (me)

You may have noticed this contribution from the commit history, but this project used to be called
`tetrio-tools` and the modules used to be called `bsr-tools` and `bsr-player`. Yuck! Thanks to
suggestions from @IFireGamer, the project has been renamed to `viewtris`, a much simpler name to say
and an easier name to remember. Just a better name in general! So thanks to him for the awesome
suggestion.
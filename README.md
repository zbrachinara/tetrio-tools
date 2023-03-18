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
3. Generality: Viewtris wouldn't have a use without games to generate replays from. I want to
   support as many block-stacking games as possible, which means being able to emulate and record a
   variety of mechanics, without intruding too much on the core format.

Secondary goals include customizability of the view and accessibility from web.

# Contributions

Replay data from these players has been used to test and develop the project:

* hahahaki
* awsum
* zbrachinara
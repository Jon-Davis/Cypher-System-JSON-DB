# Code

This is the code that is used to parse the CSRD into the json format.

The process is the relevant sections of the CSRD are copied into plain text files, then parsed using a combination of regex and state machines. The data is compiled into one large CSRD struct, which is then serialized into JSON.

--- 
All of the markdown files are licensed under the CSOL and Compatible with the Cypher System.

This product is an independent production and is not affiliated with Monte Cook Games,
LLC. It is published under the Cypher System Open License, found at
http://csol.montecookgames.com.

CYPHER SYSTEM and its logo are trademarks of Monte Cook Games, LLC in the U.S.A.
and other countries. All Monte Cook Games characters and character names, and the
distinctive likenesses thereof, are trademarks of Monte Cook Games, LLC.

--- 


# TODOS
Here is a list of TODO features I would like to add at some point.
1. Artifacts need to have option tables
2. Creatures need to have option tables (multiple probably)
3. Most objects probably need additional notes, such as those found in the margins 
4. Lesser creatures should probably be added in some form (offhanded mentions like bear is level 4 or whatever)
5. foci, types, cyphers, and creatures should probably be consistently uppercase, even if not in the CSOL
6. Renamed suggestions might be useful, for example when it says "Builds Robots" could be "Builds Golems" in the fantasy section.
7. Mundane equipment might be good to add. But might be hard to reconcile conflicting definitions.
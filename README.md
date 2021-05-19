
![](https://raw.githubusercontent.com/Havegum/Terrain-Generator/master/public/favicon.png)

# Agent-based border simulation
I originally started this because I wanted to try generating semi-realistic borders by simulating agents.

Possibly with genetic algorithms? Maybe with reinforcement learning? We'll see ... for now I'm just porting the thing over to Rust, and learning the language on the way.


## References and inspiration
### [Uncharted Atlas](https://github.com/mewo2/terrain)
The starting point for this project. Voronoi based map generation with hydraulic erosion.

### [Here be dragons](https://heredragonsabound.blogspot.com/2016/10/welcome.html)
Uses the same exact starting point as this project. Lots of well explained development, going through the pitfalls and successes!

### [ThingOnItsOwn](http://thingonitsown.blogspot.com/)
Posts about maps, evolving borders, simulating agents. Very cool!

### [Undiscovered Worlds](https://undiscoveredworlds.blogspot.com/)
Massive world generation project. Goes through lots of tricks to make the maps more believable.

### [Geologically reasonable maps](https://www.reddit.com/r/proceduralgeneration/comments/gi4hq4/geologically_reasonable_maps_seed_2/) by u/troyunrau.
More world maps. I'm not drawing world maps, but there's probably some helpful tips here.

### [Amit Patel's posts are a treasure trove](http://www-cs-students.stanford.edu/~amitp/game-programming/polygon-map-generation/)
[Lots of good stuff here ...](https://simblob.blogspot.com/2018/08/mapgen4-goals.html). Remember to check the appendices as well.


## How to try this out yourself
You will need to have [Node.js](https://nodejs.org) installed.

Additionally you'll need a bunch of [Rust stuff](https://www.rust-lang.org/tools/install).

When all is set up, you can navigate to this projects folder and run:
```bash
yarn install
yarn dev
```
If you don't have yarn, `npm install` and `npm run dev` should do the trick.

It should now be running and be available at [localhost:5000](http://localhost:5000).

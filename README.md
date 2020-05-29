# Noise & voronoi based terrain generation
*Fully based on [the fantasy map generator by mewo2](https://github.com/mewo2/terrain).*


# Agent-based border simulation
I originally stared this because I wanted to try generating semi-realistic borders by simulating agents.

Possibly with genetic algorithms? Maybe with reinforcement learning? We'll see ... for now I'm just porting the thing over to Rust, and learning the language on the way.


## Other interesting stuff
### [Geologically reasonable maps](https://www.reddit.com/r/proceduralgeneration/comments/gi4hq4/geologically_reasonable_maps_seed_2/) by u/troyunrau.
I'm not drawing world maps, but there's probably some helpful tips here.


### [Amit Patel's posts are a treasure trove](http://www-cs-students.stanford.edu/~amitp/game-programming/polygon-map-generation/)
Just [click around](https://simblob.blogspot.com/2018/08/mapgen4-goals.html) and there's a good chance you'll stumble upon neat insights about successful and not-so-successful attempts. The appendices are gold too!




## Get started
You will need to have [Node.js](https://nodejs.org) installed.

Additionally you'll need a bunch of [Rust stuff](https://www.rust-lang.org/tools/install). Specifically you'll need to be able to target wasm-unknown-unknown.

When all is set up, you can navigate to this projects folder and run:

```bash
yarn dev
```

It should now be running and be available at [localhost:5000](http://localhost:5000).

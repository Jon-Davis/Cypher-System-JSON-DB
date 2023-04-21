This product is an independent production and is not affiliated with Monte Cook Games,
LLC. It is published under the Cypher System Open License, found at
http://csol.montecookgames.com.

CYPHER SYSTEM and its logo are trademarks of Monte Cook Games, LLC in the U.S.A.
and other countries. All Monte Cook Games characters and character names, and the
distinctive likenesses thereof, are trademarks of Monte Cook Games, LLC.

CSRD.json is Compatible with the Cypher System.

CSRD.json is currently based on the Cypher System Reference Document 2023-02-10

The CSRD JSON Contains Types, Flavors, Descriptions, Foci, Abilities, Cyphers, Cypher Tables, Artifacts, Creatures, and NPCs.

```
struct CSRD_DB {
    types: Vec<Type>,
    flavors: Vec<Flavor>,
    descriptors: Vec<Descriptor>,
    foci: Vec<Focus>,
    abilities: Vec<Ability>,
    cyphers: Vec<Cypher>,
    cypher_tables: Vec<CypherTable>,
    artifacts: Vec<Artifact>,
    creatures: Vec<Creature>,
}
```

Abilities represent special abilities found in Types, Flavors, and Foci.

```
struct Ability {
    name: String,                       // The name of the ability
    cost: Option<usize>,                // The minimum point cost, if any
    pool: Vec<String>,                  // The pools this ability can use
    additional_cost: Option<String>,    // Other costs such as XP
    tier: Option<String>,               // General Tier: Low, Mid, High
    category: Vec<String>,              // Categories found in Chapter 9
    description: String,                // Description of the ability
    references: Vec<String>,            // Locations this ability pops up
}
```

AbilityRefs are used by Types, Flavors, and Foci to reference an ability

```
struct AbilityRef {
    name: String,       // The name of the ability
    tier: usize,        // What tier the ability is unlocked
    preselected: bool,  // Whether the ability was preselected or optional
}
```

BasicAbilities are abilities granted by Types and Descriptors, but are not found
in the Abilities section. Things like Starting Equipment and descriptor skills.
This struct also get's used generically whenever a name-description pair is needed.

```
struct BasicAbility {
    name: String,
    description: String,
}
```

Type is the same as a Cypher System Type

```
struct Type {
    name: String,                               // The name of the Type
    intrusions: Vec<BasicAbility>,              // Intrusion suggestions
    stat_pool: HashMap<String, usize>,          // Starting stat_pool
    special_abilities_per_tier: Vec<Amount>,    // Special Abilities unlocked at each tier
    abilities: Vec<BasicAbility>,               // Basic abilities like Starting Equipment and Effort
    special_abilities: Vec<AbilityRef>,         // Abilities found at each tier
}
```

Amount is used to signal how many abilities each type gains at each tier. For example
The Warrior get's 4 abilities at tier 1 and 2 abilities at tier 2.
```
struct Amount {
    tier: usize,
    special_abilities: usize,
}
```
Flavor is the same as a Cypher System Flavor
```
struct Flavor {
    name: String,                   // The name of the Flavor
    abilities: Vec<AbilityRef>,     // Abilities found at each tier
}
```
Descriptor is the same as a Cypher System Descriptor
```
struct Descriptor {
    name: String,                       // The name of the Descriptor
    description: String,                // The provided description
    characteristics: Vec<BasicAbility>, // Basic abilities such as skills and pool points
    links: Vec<String>,                 // Starting Adventure Links
}
```

Focus is the same as a Cypher System Focus
```
struct Focus {
    name: String,               // The name of the focus
    description: String,        // The provided description
    abilities: Vec<AbilityRef>, // Abilities at each tier
    intrusions: String,         // GM Intrusion suggestion
}
```

Cypher is a usable Cypher in the Cypher System. Note that everything was pulled from the CSRD and so kinds may be absent or different than in the core rulebook.
```
struct Cypher {
    name: String,               // The name of the cypher
    level_dice: Option<String>, // The dice used to determine the level
    level_mod: usize,           // The additional modification to the level
    effect: String,             // The effect of the cypher
    options: Vec<RollEntry>,    // A random roll table if applicable
    kinds: Vec<String>,         // MANIFEST, SUBTLE, FANTASTIC
}
```

RollEntry used for random tables
```
struct RollEntry {
    start: usize,       // starting range inclusive
    end: usize,         // ending range inclusive
    entry: String,      // name/description
}
```

CypherTable represents the random roll tables for cyphers found in the CSRD.
```
struct CypherTable {
    kind: String,               // MANIFEST, SUBTLE, FANTASTIC
    options: Vec<RollEntry>,    // Range and name of cypher
}
```

Artifacts represent Artifacts found in the CSRD.
```
struct Artifact {
    name: String,               // Name of the Artifact
    level_dice: Option<String>, // Dice used to determine level
    level_mod: usize,           // Additional modifications to the level
    form: String,               // The form of the artifact
    depletion: String,          // The depletion range
    effect: String,             // The description
}
```

Creatures represent the various creatures and npc's found in the CSRD
```
struct Creature {
    name: String,                   // The name of the creature
    kind: String,                   // Creature, NPC, or Super villain
    level: usize,                   // Level 1-10
    description: String,            // provided description
    motive: Option<String>,         // provided motive
    environment: Option<String>,    // environment if any
    health: Option<usize>,          // health
    damage: Option<String>,         // damage dealt,
    armor: usize,                   // armor, 0 if none
    movement: Option<String?,       // movement speed
    modifications: Vec<String>,     // list of modifications
    combat: Option<String>,         // combat options
    interactions: Option<String>,   // interactions
    uses: Option<String>,           // use if any
    loot: Option<String>,           // loot if any
    intrusions: Option<String>      // GM intrusions if any
}
```

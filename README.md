This product is an independent production and is not affiliated with Monte Cook Games,
LLC. It is published under the Cypher System Open License, found at
http://csol.montecookgames.com.

CYPHER SYSTEM and its logo are trademarks of Monte Cook Games, LLC in the U.S.A.
and other countries. All Monte Cook Games characters and character names, and the
distinctive likenesses thereof, are trademarks of Monte Cook Games, LLC.

CSDR.json is Compatible with the Cypher System.

The CSDR JSON Contains Types, Flavors, Descriptions, Foci, and Abilities

struct CSRD_DB {
    types: Vec<Type>,
    flavors: Vec<Flavor>,
    descriptors: Vec<Descriptor>,
    foci: Vec<Focus>,
    abilities: Vec<Ability>,
}

Abilities represent special abilities found in Types, Flavors, and Foci.

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

AbilityRefs are used by Types, Flavors, and Foci to reference an ability

struct AbilityRef {
    name: String,       // The name of the ability
    tier: usize,        // What tier the ability is unlocked
    preselected: bool,  // Whether the ability was preselected or optional
}

BasicAbilities are abilities granted by Types and Descriptors, but are not found
in the Abilities section. Things like Starting Equipment and descriptor skills.
This struct also get's used generically whenever a name-description pair is needed.

struct BasicAbility {
    name: String,
    description: String,
}

Type is the same as a Cypher System Type

struct Type {
    name: String,                               // The name of the Type
    intrusions: Vec<BasicAbility>,              // Intrusion suggestions
    stat_pool: HashMap<String, usize>,          // Starting stat_pool
    special_abilities_per_tier: Vec<Amount>,    // Special Abilities unlocked at each tier
    abilities: Vec<BasicAbility>,               // Basic abilities like Starting Equipment and Effort
    special_abilities: Vec<AbilityRef>,         // Abilities found at each tier
}

Used to signal how many abilities each type gains at each tier. For example
The Warrior get's 4 abilities at tier 1 and 2 abilities at tier 2.

struct Amount {
    tier: usize,
    special_abilities: usize,
}

Flavor is the same as a Cypher System Flavor

struct Flavor {
    name: String,                   // The name of the Flavor
    abilities: Vec<AbilityRef>,     // Abilities found at each tier
}

Descriptor is the same as a Cypher System Descriptor

struct Descriptor {
    name: String,                       // The name of the Descriptor
    description: String,                // The provided description
    characteristics: Vec<BasicAbility>, // Basic abilities such as skills and pool points
    links: Vec<String>,                 // Starting Adventure Links
}


Focus is the same as a Cypher System Focus

struct Focus {
    name: String,               // The name of the focus
    description: String,        // The provided description
    abilities: Vec<AbilityRef>, // Abilities at each tier
    intrusions: String,         // GM Intrusion suggestion
}

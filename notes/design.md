# Atelier Esri design notes

Goals for this demake:
- graph-style alchemy
    - will need to massively abbreviate traits
    - fill up nodes with the right items to unlock effects and other nodes
    - TODO: describe overall flow
- turn-based combat
    - no back row since there are only 3 party members
    - time cards
        - powerful attacks delay the next turn
        - some abilities load multiple damage or heal cards
    - TODO: describe overall flow
- simplified gathering
    - go to a gathering spot and press O
    - no need to select tools
    - no basket cap: you go home once you've stolen everything on the map, or you get bored
    - if you can gather, the spot is bright
    - if you don't have the tools, the spot is dark and disabled
- small hub town that gets a few more buildings as you progress

# characters

## party

early 20s

### Esriara "Esri" Oberhauser

- alchemist
- impulsive party girl: breaking into the abandoned atelier at the edge of town seemed like a good idea last night
- petty criminal, kleptomaniac, aspiring drug dealer, likely future arsonist (or i could just say "Atelier protagonist")
- not stupid but pretty lazy

### Alinalyn "Allie" Braunbeck

- all muscles
- weapon: sword as big as she is
- blonde in body and soul
- entirely too cheerful
- would never do a crime on purpose, constantly doing crimes by accident (or because Esri told her to)
- reminds Esri of her golden retriever

### Saelrana "Sae" Weiskopf

- glasses
- black and white color scheme
- perpetually half asleep
- failed alchemist: no natural talent, studied hard, didn't get very far, but has all the book learning
- natural top: makes Esri and Allie work
- party conscience (but exploiting Esri and Allie is fine)

## townies

### Esri's Parents

- lying to the Oberhausers about how Esri's "independent study" or "small business" is going is how the player checks the story so far
- "Oh, the mine that just opened up? I've been selling them shovels! It's going great!"
  - Actually blew the cave open herself
- "I've been learning a ton about commodities markets."
  - Because a story quest flooded the market with cheap staltium

### Thor Merchantsson

- Hannah's dad
- big guy
- overprotective, will not sell the party anything dangerous

### Hannah Merchantsdottir

- cheerful girl
- still in high school
- middleman between the party and her dad; definitely scamming both of them
- regularly tries to get Esri to give her booze/weed/mushrooms, which Esri will absolutely not do

### Lillian "Lil" Hammersmith

- tall, butch, fond of tea and crumpets
- town **blacksmith**

### Vivian "Viv" Hammersmith

- tall, butch, fond of tea and crumpets
- shame of the Hammersmith family for opening a **cafe**

### Zahnrad

- clockwork doll that runs the **observatory**
- quiet but tolerates other quiet people like Sae
- one half of the historic alchemist **Syzygy**, who split into halves after an internal debate about research methods

## antagonists

### Plum

- big bad
- actually not that big, or that bad, but has a plan so don't get in her way
- this dimunitive big-hat-wearing witch wants to finish Syzygy's research from three hundred years ago, by breaking Aufgang Point off at its base and turning it into a floating interdimensional ship
- does not care that this would make life difficult for the many people who have made their homes on Aufgang Point since
- in the post-game, hangs out with Zahnrad at the observatory

### Isabel "Izzy" Lichter

- Esri's high school crush (play it cool, Esri, you're over her)
- friendly in an aw-shucks kind of way
- party runs into her frequently in town
- excited about her new job, and Esri and Allie are happy for her, until they find out that Izzy's new job is **Plum's lieutenant**
- rapier in one hand, wand in the other (Plum's taught her a few spells)

# locations

## the town of Aufgang Point

- a hill with the observatory at the top
- multiple microclimates
- way more monsters than the surrounding area
- Syzygy webbed the hill with portals hundreds of years ago, so Aufgang Point is not what it seems, and walking in certain directions will put you in another dimension

## observatory

- the observatory can see things both in this world and in the others accessible through the portals
- Esri can go here to mark treasures and high-level gathering spots in other dimensions so that they'll show up for her when she's there

## blacksmith

- small and cramped

## cafe

- also small and cramped

## general store

- reconciling the Hammersmith sisters merges the blacksmith and cafe into **Hammersmith & Hammersmith General Goods**, which carries everything

# progression

## environments

1. town + hillside forest
    - puni
    - giant spiders
2. cavern
    - bats
    - snakes
    - boss: giant eurypterid
3. jungle
    - the party is now reasonably sure that this is not anywhere on Aufgang Point
    - pterodactyls
    - jungle cats
    - carnivorous plants
    - boss: volcanic elemental
4. rust desert
    - slags
    - antlions
    - miniboss: big slag
    - enviroment switch: abandoned high-tech factory
    - feral automatons
    - boss: Izzy
        - "Plum says this isn't safe. You guys need to leave."
        - "I don't care what Plum says. She sounds sketchy as hell."
        - "She's my ticket out of this no-horse town. And seriously, just turn around. Go home. You're not supposed to be here."
        - "Be smart about this, Izzy. There's three of us and one of you."
        - "Yeah, I see Allie… and two bystanders. And by the way, it's not just me. Lightning Sigil: Summon Neo-Scarabs!"
5. astral plane
    - phantasms
    - crystal… things
    - boss: Plum
    - enviroment switch: the astral rift closes when Plum is defeated, dumping you into a second phase *in the middle of town*

## weapon tiers

T3 is the first tier where Esri synthesizes her own equipment designs, but she will also be able to synthesize improved copies of T2 equipment, unlocking numbered effect slots not available on equipment from the shop. Given that there are only 3 playable characters, I'd like the player to be able to customize the party a bit by making T2 and T3 weapons overlap in power but with differing attacks and effects. This is kind of already a thing in real Atelier, but usually weapons of a given tier are significantly more powerful than weapons from a previous tier, and there's no point to keeping the old stuff around.

1. junk every character starts with
    - Esri: Sae's staff
        - "It's just a loaner, okay? I want it back eventually."
    - Allie: sword
        - "Had it since high school. Not my first, won't be my last. Gets the job done."
    - Sae: slingshot
        - "It's for self-defense. And keeping raccoons off my roof."
2. the party makes enough money to buy better gear from Lillian Hammersmith
    - Esri: The Esri Special (staff)
        - "Lil, you think you could put a core of this metal in one of your big oak staves?"
        - "Yeah, sure, Esri. You want a hiking stick you can hit people with?"
        - "Sort of. This is a transmutation catalyst alloy like the one alchemical cauldrons are made out of. I want a hiking stick I can hit *anything* with."
    - Allie: warhammer
        - "Hey, Aunt Lil!"
        - "What do you need, kiddo?"
        - "I wanna branch out a bit. Sword's good. Sword's fine. Whole family loves swords. Swords every day. What do you have that's absolutely *not* a sword?"
        - "I thought this day might come. It's only natural to want to experiment, Allie. How do you feel about really big hammers?"
    - Sae: crossbow
        - "I've seen you around. Peeping through the window a few times."
        - "Look, umm, I'm not really… the kind of person who… goes into places like this."
        - "You're not really the kind of person who buys the stuff in the window. That's flash. That's for looking like trouble and hoping there won't be trouble.  That's not you."
        - "It is definitely not me."
        - "So you need, let me think… you need something that assumes there's already trouble, and ends it as quickly as possible."
        - "I'm listening."
        - "You need range. You need power. And you don't have Allie's muscles or Esri's uncontrollable enthusiasm, so you need some sort of mechanical advantage. I have just the implement. It's called the Problem Solver."
3. Esri figures out how to synthesize equipment
    - Esri: Storm Spire (staff)
        - "Heyyyyy, Esri. What are you making?"
        - "New staff."
        - "What for?"
        - "Not sure. Feel like some of the materials we brought back from the jungle are talking to me. Like they want to be used."
        - "When was the last time you slept?"
        - "Can't sleep until I find out what this does…"
    - Allie: buzzglaive/diskos
        - "Allie! Look, I made you a glaive!"
        - "Aw, thanks, Esri! But you know, the use of the word 'glaive' to mean a three-bladed throwing weapon is from an old cult classic manga. Historically speaking, a 'glaive' is actually a kind of polearm."
        - "Oh, okay, yeah! I couldn't figure out which was the real thing, so I made it both."
        - "Both?"
        - "You hold the pole end, and the end with the three spinning blades goes in the enemy. Do you want me to, like, label those parts, or are you good?"
    - Sae: ultrabow
        - *(balance note: the T2 bow will be upgradable to wind splash damage for crowd control if you can max out its hardest numbered effect slot while synthesizing it yourself. likewise, the T3 bow can be synthesized to deal DEF bypass damage.)*
        - "Esri, have you been fucking with my crossbow? You know that's not a toy."
        - "Pffft, might as well be. Don't tell Lil I said that, but there's only so much you can do with steel, really."
        - "So what's this made of?"
        - "A little of this, a little of that… I mean, I have a list, I've been writing stuff down, I swear, but go on, pick it up."
        - "Oh, that's a lot lighter."
        - "Harmonium and spider silk, baby!"
        - "Wait, did you get over your thing about spiders?"
        - "Absolutely not. I'm just drunk enough to synthesize something beautiful from them and put the screaming off for a few hours."
4. Endgame weapons. Black with glowing bits. Totally unnecessary overkill. We may never actually put these in. I wanted to write the fiction anyway.
    - Esri: The Fulcrum (staff)
        - "That thing worries me."
        - "It's ominous, right? Not dangerous, not by itself, but it feels like something I shouldn't have. And this is me saying that."
        - "The fundamental forces of the universe are more or less balanced, Esri. They have to be, or we wouldn't exist. Gravity pulls us down, the floor keeps us up. But you put a thumb wrong on the new staff, and I think gravity goes sideways and the floor is lava."
        - "I'll be careful. We get through this, Sae. We do what we need to do. Then it goes in a safe under the floorboards, and we hope we never need it again."
        - "You really have grown since we started this, Esri. I'm proud of you. And that's the only reason I'm not running for it right now."
    - Allie: photonic scythe (double-ended, doubles physical attacks)
        - "I saw Death, once. The very first time I almost died. Did I tell you that?"
        - "You what? You saw Death?"
        - "You know my family has that whole 'live by the sword, die by the sword' deal. Maybe we have Death's attention already, and Death came by as a courtesy, or a reminder. It was a training accident with my dad. Blunt sword, but the blade snapped, got me in the lung."
        - "Holy shit, Allie. When was this?"
        - "I was fifteen. People get the black robe right, but the skeleton part? Nah. She's a woman, did you know that? Beautiful, and a little sad. She looked at me, and shook her head, like, 'not yet', and next thing I knew, I was on the dining room table, coughing up blood while my uncle stitched my chest back together."
        - "Your family is terrifying. I'm glad they're on our side. But why are you telling us this now?"
        - "People get the scythe part right too. Death carries a scythe. I saw it. Same shape as any farmer's. To fifteen-year-old me, it looked like the end of all things, and maybe it is. But you know what? Today I'm looking at this gorgeous glowing double-bladed slicing machine you made me, and I'm thinking, and it's crazy, but I'm thinking, with this… maybe I could take her. If I had to."
    - Sae: transmutation bow
        - "Sae, I wanted to say thanks. For everything. Alchemy is my life now. But, like, if I were the one teaching someone how to do it, and they were doing it way easier than I ever could, I'd be such a mess, and you've been so patient, I don't even know how you do it. Anyway, sorry, I'm fucking awful at having feelings, so I made you this."
        - "A longbow? Uh. Thanks, but I'm not sure I have the muscles to draw one of these things."
        - "It's… more longbow-shaped than it's a longbow. Give it a pull. You don't need to load it. But for the love of fuck don't let go suddenly, and don't point it at anything but the floor."
        - "Esri, what is this thing? My hands are tingling. My *teeth* are tingling. There're rainbows round the edges of everything."
        - "I wanted to make you something that'd help with your alchemy. I don't know if that's possible, but I had to try. And the most confident I've ever seen you is when you're grinning down the sights of something."
        - "When I'm what?"
        - "You're, like, practically feral. I've seen you literally licking your lips. It's honestly hot. So I crammed a bow full of variations on cauldron metal, and every other transmutation catalyst I could think of that'd fit your whole deal. A lot of twisty little bits, a lot of really poisonous stuff."
        - "Twisty bits and poisons; should I be insulted? Never mind. It sounds almost like an alchemical staff."
        - "Yeah, I guess, but it's a bow. I couldn't test the final version. My alchemy was barely enough to make it, and it's not the right kind to use it; it has to be yours. But I think if *you* fire an arrow from it, it won't be an arrow when it hits. It'll be whatever you need it to be. Acid. Darkness. Big ol' tungsten spikes."
        - "I… don't know what to say either. That's so much effort. I can feel how much power went into this. Nobody's ever done anything like this for me. Even if it doesn't work, thank you. Thank you for trying. And it's been fun teaching you, you know? You've come so far! My own attempt to learn alchemy turned out to not be totally pointless because I could pass it on to you."
        - "You're welcome!"
        - "Now let's go very, very, very far from the atelier so I can get the hang of this."

# cutscenes

## page operations

- set background
- play background effect
- set character and emotion in character slot (left, right)
- unset character
- display text with speaker name from character slot
- play character effect on character slot
- set item
- unset item
- play item effect

## meta operations

- execute page (generates log entry)
- conditional on plot counter value or range


# combat

Stats in the Atelier games almost always come out to HP, MP, ATK, DEF, SPD, resistances, and some sort of equipment cap. [Atelier Sophie 2](https://www.koeitecmoamerica.com/manual/sophie2/en/7200.html) is a good example.

- ATK: modifies damage dealt
- DEF: modifies damage received (combined with resistances)
- SPD: affects wait time between turns, and also critical hit chance

## resistances

Resistances can be negative, increasing that damage type. If we go with the Sophie 2 model, damage can be either physical or magical, and it has an optional elemental subtype. All of the applicable resistances should be applied: elemental (if applicable), damage type, and then DEF.

Sophie 1 and Firis don't have elemental subtypes for physical damage (calling it "impact damage"?) and do have "ultimate non-attribute damage" which bypasses all resistances. I'm tempted to include this. 

It's possible for resistance to be 100%, making something totally immune to damage of that type unless it's modified.

It's not clear how the chances of catching a status effect are calculated. Might be based on magic resistance, or on level.

## status effects

[Sophie 2](https://www.koeitecmoamerica.com/manual/sophie2/en/6200.html)

- specific stat or resistances up/down
- all stats (ATK/DEF/SPD anyway) or resistances up/down
  - pushing resistances to negative so something takes extra fire damage would be fun
- HP and/or MP regen
- Acceleration: shortens wait time but doesn't affect crits (unlike increasing SPD)
- damage boost to skill damage or item damage (I would also add normal attack damage, might be fun)
- Damage Reduction: not clear if this is the same as boosting DEF
- Poison: DoT
- Sleep: skip turn, take extra damage, may wake up if damaged
- Curse: reduce magic resistance, block HP recovery or do damage on HP recovery attempt
- Burns: take additional fire damage when attacked
- Frostbite: extended wait, reduce physical resistance
- Paralysis: from lightning effects? chance to skip turn
- Restraints: "Make it impossible to evade attacks", which is weird because Sophie 2 doesn't have a dodge mechanic?, increases chance of critical hits to the target
- Slow: (provisional) extended wait but unlike Frostbite leaves physical resistance alone
- add a damage type to normal attacks

## gauges

I like break mechanics so let's have a break gauge. Broken enemies (or party members) have dramatically lowered stats and resistances until the break wears off.

## skills

- Esri
  - can use all items
    - Esri should not be using attacks or skills until she's all out of items, except possibly in the presence of normal attack damage up/skill damage up buffs.
  - normal attack by swinging staff
    - magic damage (it's alchemizing the target more than it is hitting it)
  - Burn
    - fire magic damage
    - 2x damage if target has Frostbite
    - OR chance to Burns? if we go that route, Burn synergizes with *itself*
  - Shatter
    - ice magic damage
    - chance to Frostbite
- Allie
  - cannot use items
  - normal attack with whatever blade she has
    - physical damage
  - Bulwark
    - enemies doing physical damage should target Allie first
  - Smash
    - physical damage
    - stun damage
  - Blitz
    - physical lightning damage
    - if it crits, becomes a multi-target attack
    - chance to inflict Paralysis?
    - synergizes with Restraints and anything that increases crit chance
  - Overwatch
    - reaction skill, expires on Allie's next turn
    - large physical damage to any physical attacker, once
    - should be expensive
  - Cleave
    - physical damage
    - if target is affected by Burns, Frostbite, Paralysis, or Restraints, do additional magical fire, ice, lightning, or wind damage
- Sae
  - can use some items, maybe up to T2
  - normal attack with slingshot/bow
    - physical damage
  - Rapid Fire
    - six physical damage attacks
      - modify count based on Sae's level?
    - each does a small amount but it should add up to slightly more than a normal attack
    - random targets
  - Called Shot
    - physical damage
    - chance of random effect
      - Slow
      - Stun
      - Restraints
  - Acid Rain
    - 2x time card physical AoE wind damage
  - Corrosion
    - -DEF

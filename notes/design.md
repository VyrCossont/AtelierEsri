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

## further reduced goals for this demake

Alchemy cartridge only:
- you get some recipes
- you get some items
- you make the requested item
- any extra items you make (neutralizers and other intermediates, target items above the requested count) roll over to the next round
- after 5 rounds, you're done, you get a high score

# architecture

- multiple cartridges? (not on WASM-4)
    - title/loader
    - hub town
    - atelier interior
    - synthesis (maybe one per set of recipes if necessary)
    - gathering area (one per gathering area)
    - combat (one or more per gathering area or boss fight)
    - cutscene
- save format
    - item
        - ID
        - quantity
        - quality
        - item effect counters
        - trait flags

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
	- boss: ???
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
	- Allie: 
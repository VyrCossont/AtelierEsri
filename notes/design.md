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
- red and purple color scheme, lots of hair
- not stupid but pretty lazy
- weapon: staff

### Alinalyn "Allie" Braunbeck

- all muscles
- weapon: sword as big as she is
- blonde in body and soul
- yellow and blue color scheme
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
- weapon: crossbow

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

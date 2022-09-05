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

# architecture

- multiple cartridges?
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

### Hannah Merchantsdottir

- still in high school
- middleman between the party and her dad, who refuses to sell them anything dangerous; definitely scamming both of them
- regularly tries to get Esri to give her booze/weed/mushrooms, which Esri will absolutely not do

## antagonists

- TODO: copy from paper notes

# plot

- TODO: copy from paper notes

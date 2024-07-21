## Village? defense game design

Each day is split into 10 turns 

At dawn each day there is a chance of visitors, a merchant offering a trade, people seeking shelter from the monsters, a mercenary you can hire etc, travelling npcs that can offer you some powerup to help against the next attack. 
Player can choose whether to take what they offer or accept the trade etc. Have some currency gold coins or or crystals you can collect from the monsters or something. 
If you take in extra people, they can speed up building defenses but require extra food (Maybe should just ignore food and hunger to keep things simple).
During the day you do economy things? and build buildings and defenses. 
During the night the monsters attack. At dawn, the attack and ends the monsters retreat/remaining monsters on the field die.

5 days make a week, every fifth day there is a full moon when an extremely dangerous boss monster accompanies the wave and wrecks everything. Other waves aren't so dangerous and can be farmed for resources.
Each season is two weeks long. In winter the second week ends in an eclipse not a full moon, the final boss attacks and the night doesn't end until you either defeat them or are wiped out. 
Defeating the final boss during the eclipse wins the game.

* Summer: 6 turns day/4 turns night
* Autumn: 5 turns day/5 turns night
* Winter: 4 turns day/6 turns night
* Eclipse: Night continues until you defeat Dracula (or whoever)

## Special Vistors
Grant a one time bonus for the next night
* Necromancer - will cast a spell to increase or decrease the length of the next night
* Warlock - cast a hex to decrease monster stats
* Surgeon - cure one character's wounds immediately
* Priest - bless a character or trap to increase its stats
* Soothsayer - predict the number and type of enemy attackers

## Merchant
The next day after each boss attack on the full moon there's a guaranteed merchant that turns up with some good items.

## Resources
Might be simplest to go with just gold. Other options: food, wood, stone, metal, monster corpses, crystals.
* Could have a system where you can choose to either assign villagers to building or send them out to gather resources or farm.
* Could have buildings that generate certain resources for each turn during the day that they have villagers assigned to them.

## Buildings
The player can assign the people of the village to building structures and traps.
Each structure requires turns, resources and a number of workers to construct it. 
Building only happens during the day, if a structure isn't finished before nighttime the work continues the next day (if the in progress structure wasn't destroyed).
During the night the villagers hide inside buildings, if the building is destroyed they die.
There must be an unblocked path to every building (excepting traps and gates).
Buildings can be demolished, takes 1 turn per tile.

## Building types
* residence - allows a larger population, can build larger residences with more defense
* wall - blocks monsters can be broken down, can be reinforced
* doctor - speeds up healing 2x1
* gate - blocks monsters until it is broken down, doesn't block path to buildings
* tavern - mercenaries stay even if you don't pay them 2x1
* blacksmith - unlocks new types of traps and equipment 2x1
* guard tower - when manned, can attack a monster once per turn
* witch - brew potions that can be used at night 
* moat - water hazard that blocks monsters that can't swim or fly

## Traps
* Bear traps
* Landmines
* Crossbow and pressure plates
* Burning oil
* Crossbow tower (has to be manned by a worker who won't be available next day for building) can shoot in four directions

## Hunters + Mercenaries
During the night they can be controlled by the player to fight the monsters. They have a number of actions they can perform each turn. Maybe can also trigger traps or use certain structures in some ways.
Hunters fight monsters because its their duty or something.
Mercenaries charge gold per night, if you don't pay them they leave the village.
* Could be a building like a tavern you can build then mercenaries stay at the tavern and you can choose each night if you'll pay them to fight

## Combat Rules
Broadly similar to "Into the Breach" and "Xcom: EU" rules.

Each turn a unit (both player and computer controlled) can make a move and then an action. 
An action could be an attack or using a special ability
They can't make an action and then a move (unless they have a special ability that allows them to do so).

* Each unit has a movement characteristic that determines how many tiles they can move by rook move rules (no diagonal movement) unless they are flying in which case they can move in all eight directions. Each move takes 1 movement point.
* Two units can never occupy the same tile, including flying units. They also can't enter a building tile. They can enter a trapped tile.
* Flying units can move through any terrain without restriction.
* Units can have a swim ability that allows them to move through a water tile. They can't end a turn on a water tile however.
* Aquatic units can move like with the swim ability but can also end their turn on a water tile and take actions while on a water tile.
* Player controlled characters can move through buildings and gates but not walls, as long as they have enough movement to reach a tile on the other side of the building. They can't end their move on a building tile. Enemy movement is completely blocked by buildings.
* Towers can't move, but can perform actions.

## Actions
* Attack: attack a target with an equipped weapon. The target must be within the weapon's range and there can't be any other units or buildings between the attacker and the target.
* Throw: toss a potion or bomb up to three tiles in four directions.
* First Aid: Heal an adjacent unit for 1hp.
* Heal: Cure an adjacent units disease or poison.
* Recovery: Heal self for 1hp.
* Unweb: Available when webbed, removes web status.
* Block: Blocks the next attack against this unit.
* Kick: Knock an enemy back a tile

## Debuffs 
* Webbed: Can't move until an unweb action is performed.
* Poison: Lose 1hp per turn while poisoned.
* Disease: Lose 1 max hp and 1 move while diseased.

## Special Abilities (names subject to change)
* Run: take a second move instead of taking an action.
* Operator: can perform two actions if they don't move this turn.
* Tactician: can move after performing an action, if they hadn't already moved this turn.
* Intangible: can move through other units and buildings.
* Swim: can move through water tiles, but not end their move on a water tile.
* Aquatic: can move through and end their move on water tile.
* Flying: Can move through any terrain.
 
## Hitpoints
* Death is permanent
* To recover HP they have to skip a night of fighting and rest.

## Monster Types
* Bat - weak, can fly
* Troll - regenerates if not killed outright
* Merman - can swim
* Vampire - intelligent, can transform into a mist to pass barriers, mesmerise paralyzation attack
* Slime - can multiply
* Werewolf - quick, tough, high damage
* Skeleton - resistant to arrows and cross bow bolts, weak against blunt force
* Zombie - tough, anything it kills is raised from the dead and becomes another zombie
* Giant rat - cowardly, sneaky, causes disease
* Ghoul - diseased claws
* Mummy - very tough and strong but slow and weak against fire
* Skeleton archer - skeleton but with ranged attack
* Wizard - easy to kill but strong ranged attack against buildings
* Spider - creates webs that take a turn for characters to break out of, poisoned melee attack
* Revenant - armoured undead, big axe attack
* Thrall - human servant, can have similar abilities to player's mercenaries and hunters
* Ghost - Can pass through player units, walls and buildings. 
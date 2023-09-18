
# Skills 
* Change skills by /slash command
* Have skills that can gain experience based on usage

# Code
* Put code in a separate files based on functionality
* Ensure all calls to the db are localized to a single file/abstracted to a certain extent
* Have player and enemy actions take a `BattleInfo` and handle summation of the battle themselves

# Traits
* Traits that interact with things like hp ( vampire etc. )
* Allow infinite spending of traits on attribute boosts

# Zones & Areas
* Let users chance "zone" with different mobs & maybe mob levels

# Items
* Allow items to be equipped need a /slash command that accepts an item name or id
** In the short term we can just equip the best item in a slot
* Have enemies drop items
* Need a /slash command to list users items 
* Maybe a shop where you can gamble for items using gold
* Trading items between players
* Make items impact the combat probably at the `AttackModifier` and `DefenseModifier` level

# Mobs
* More mob variety
* Give mobs a defensive struct to make use of their resistances

# Combat
* Create a /slash command to let a user select a mob to fight this should probably cost some resource

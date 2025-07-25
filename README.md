# Convoy

## Description

A turn based strategy game based on advance wars

## Rules

### The Board

#### Tile Types

##### Empty

No special properties

##### Town

Provides an income bonus of 1

##### City

Provides an income bonus of 3

##### Border

Provides supplies and is a recruitment point

### Units

#### Recruiting

Units can be recruited at any recruitment tile under your control by paying their cost. Newly recruited units are
exhausted

#### Types

##### Convoy

###### Stats

| Speed | Power | Range | Cost |
|:-----:|:-----:|:-----:|:----:|
|   3   |  N/A  |  N/A  |  3   |

###### Special Ability

Convoys are a non-combat logistics unit that cannot attack or defend itself. Instead, if supplied, it supplies all units
within a 3 tile radius.

##### Infantry

###### Stats

| Speed | Power | Range | Cost |
|:-----:|:-----:|:-----:|:----:|
|   2   |   2   |   2   |  2   |

###### Special Ability

None

##### Artillery

###### Stats

| Speed | Power | Range | Cost |
|:-----:|:-----:|:-----:|:----:|
|   2   |   2   |  2-3  |  3   |

###### Special Ability

Artillery units cannot initiate a battle or move and act, but can support a battle without becoming exhausted.

##### Recon

###### Stats

| Speed | Power | Range | Cost |
|:-----:|:-----:|:-----:|:----:|
|   3   |   1   |   1   |  4   |

###### Special Ability

Recon units do not disband when unsupplied and can move to defend an

### Battles

A unit can initiate a battle against any enemy unit within its range; if the attacked unit is not exhausted it may
defend itself, exhausting it.

Other units may support the attack or defence if the targeted tile is within the supporting units attack range,
exhausting that unit; a supporting attacker may move into range to support, while defenders cannot.

The attacking side selects all attacking units in a battle before the defender

Once all involved units have been finalized, the side with the greatest power is the winner, with loser being destroyed;
in the case of a draw nothing happens.

### Turn Structure

A turn is make of three phases

#### Income

You gain money equal to your income

#### Command

You can give as many commands as your resources allow

##### Recruit

##### Move

##### Attack

#### Resupply

All unsupplied units are disbanded

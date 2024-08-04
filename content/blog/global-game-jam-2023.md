---
title: 'Participating in the Global Game Jam 2023'
description: 'Making a game with Godot engine in 48 hours'
date: '2023-02-10'
---

This year, I decided to participate in the Global Game Jam for the first time this past weekend. I discovered that there was a jam site at my university, so I decided to go and check it out. 48 hours to make a game was an interesting challenge, and I was excited to see what I could make in that time.

<!-- more -->

## [Opening Ceremony]

The opening ceremony started at 5:00 pm on Friday. I arrived at the site around 4:45 pm, and there were already a few people there. Since the global game jam is a worldwide event, there was an official opening ceremony that was streamed to all the sites. We watched the opening ceremony and then the site leaders, one of whom was one of my professors, gave a brief introduction to the event.

## [Brainstorming]

The theme was revealed at the end of the opening ceremony. This year's theme was "Roots". The nice thing about game jams is that you can interpret the theme however you want. Is the theme about the roots of a tree? Is it about the roots of a family? Is it about the roots of a number? The theme is open to interpretation, so you can end up with a wide variety of games.

During the introduction to the event, the site leaders suggested that we spend the first 30 minutes brainstorming ideas. I decided to do this, and I ended up with a few ideas. I decided to write them down on my tablet, so I could easily reference them later.

## [Choosing an Idea]

Once the brainstorming session was over, we were suggested to present our favorite ideas to all the other participants. This was a good way to get feedback on your ideas, as well as possibly finding a team to work with. I presented my idea, and a few other people had similar ideas, but with a few different twists. Most of us were connected to our own ideas, so we decided to work separately.

While I think that having time at the beginning of the jam to brainstorm ideas is a good idea, I think that it would be better to have the brainstorming time be quite a bit shorter. I think that 30 minutes is a bit too long, before presenting your ideas to the other participants. I think that 10 minutes might be a better amount of time, as it would allow you to come up with a few ideas while avoiding getting so connected to them that you don't want to abandon them.

My chosen idea was a game where you build a root system to help a plant grow. You navigate each root by directing it toward the nutrients in the soil with your mouse. You also have to avoid obstacles, such as rocks. I thought that this idea was interesting, and I thought that it would be fun to make. I also thought that it would be a good way to get more experience using Godot since it felt like the idea was not overly ambitious or complicated.

## [Setting Up the Project]

After brainstorming and attempting to form teams, it was time to start working on our games. Since I was working alone, I decided to start working on my game by setting up the project in Godot. I have been using Godot for a class this semester, so I understand the basics of how to use it, but I haven't used it for a game jam before. I decided to use Godot for my game because I wanted to get more experience using it. I also wanted to use a game engine that I was at least somewhat familiar with, so I could focus on making the game, rather than learning how to use a new game engine for the first time and making a game in 48 hours.

I chose to use Godot 4.0 which as of the time of writing this post, has just released its first release candidate. I decided to use the latest version of Godot because of how close it is to being released. I wanted to get some experience using the latest version of Godot, so I could be ready to use it when it is released. I feel that making a game in a game jam isn't critical enough to worry about perfect stability, so I decided to use the latest version of Godot and have access to the latest features.

I started by creating a new project in Godot. I decided to use the Mobile renderer since I knew that I wouldn't be creating anything that required the quality of the high-end Forward+ renderer. I also decided to use the Vulkan rendering backend, since it is one of the major advantages of Godot 4.0.

## [Starting the Game]

The first step was to set up the basic structure of the game. I decided to create a scene for the main menu, a scene for the game, and a scene for the root. I also created one scene for both the good and bad (nutrients and rocks) obstacles. I created a scene for the root because I wanted to be able to instantiate multiple roots in the game scene. I created a scene for the obstacles for the same reason.

Since this game jam occurred over three days, I think it is better if I explain the development process in two parts. The first part will be about the implementation, and the second part will be about issues that I ran into and how I fixed them.

## [Implementation]

### [Root Scene]

I decided to have the root handle its own movement in its own script so that it would always move in the direction of the mouse, as long as it was enabled. I also decided to have the root handle its own collision detection so that it would be able to detect when it collided with an obstacle. To render the root, I decided to use a Line2D node. I used a Line2D node because it is a simple way to render a line, and it is extremely easy to add points to the line.

To ensure the number of points in the line did not grow too quickly, I decided to only add a point to the line every 0.1 seconds using a timer node. I set up a signal for obstacle collision so that the root could notify the game scene when it collided with an obstacle.

### [Game Scene]

The game scene is responsible for spawning the root and the obstacles, as well as handling the game logic. I decided to spawn the root in the center of the screen 128px below the top. The obstacles are spawned at random positions all below the root. I decided to spawn the obstacles at random positions so that the game is slightly different each time you play it.

Spawning the obstacles was a very simple process. Since all of the obstacles are created at the beginning of the game, I just added a for loop to the \_ready() function to spawn the obstacles. I decided to spawn 30 obstacles, 10 of which are good and 20 of which are bad. I decided to spawn 30 obstacles because it felt like a balanced amount for a 1080p screen after testing a few different amounts.

The game scene also handles the creation of new branches of the root which are spawned when the root collides with a nutrient. When the root collides with a nutrient, it notifies the game scene. The game script then traverses the root system slowly and creates a new branch once the player clicks the mouse. The new branch is created by creating a new root scene at the position that the traversal is currently at, rotated 90 degrees towards the mouse. The game stores an array of the points in the root system up to the starting point of the system, which makes it much easier to traverse the system.

When the root collides with a rock or itself, the root shrinks by 24 points. Hitting a rock also causes the root system to lose 1 health. The game ends when the root system loses all five health points.

## [Issues and Fixes]

### [Root Movement]

The first major issue that I ran into was the root movement feeling too controlled. I wanted the root to move in the direction of the mouse, but I felt that just moving directly toward the mouse was too easy. I decided to add a small amount of randomness to the root movement so that it would be slightly more difficult to control.

My first attempt at adding randomness to the root movement was to add a random value to the direction of the root. I added a random value between -0.1 and 0.1 to the direction of the root. This didn't work very well because the value still averaged out to 0, and randomizing it every frame made it average out very quickly.

I then decided to add a random value to the direction of the root based on a 2d noise function. This worked much better because the noise value slowly moved around, and the root would move in a random direction for a while before changing direction again. It felt much more natural than the previous method.

### [Root Collision]

The next major issue that I ran into was root collision detection. Detecting when the root collided with an obstacle was very simple. I just checked if the point being added to the root was within the collision shape of an obstacle. If it was, I used a signal to notify the game scene that the root collided with an obstacle (good or bad). The more difficult part was detecting when the root collided with itself.

Since I was only adding a point to the root every 0.1 seconds, the line would jump around a lot. To fix this, I decided to always move the final point of the line to the actual position of the root. This made the line much smoother, but then caused some issues with my collision detection. I was looping through all of the points in the line and checking if they intersected with the last point and the actual root tip. Once I added the code to move the final point of the line to the actual position of the root, the collision detection would no longer work. I realized that the issue was that the last point of the line would always be the actual position of the root tip, so it was just the same point being checked twice. This meant that the collision detection would never detect a collision with the root. I fixed this by checking against the second to last point in the line instead of the last point. This fixed the issue, and the collision detection worked as expected.

## [Closing Ceremony]

After the game jam ended, we had a closing ceremony where we were able to play all of the games that were created at our site during the game jam. I really enjoyed playing the games other people created. I think that it was a great way to see how other people interpreted the theme and how they solved the problems that they ran into. We also had pizza and snacks at the closing ceremony, which was a nice way to end the game jam.

## [Final Thoughts]

I really enjoyed participating in the Global Game Jam this year. I think that it was a great way to get more experience using Godot. Even though I have participated in a few game jams before, this was the first time that I have attended an in-person game jam. I believe that it was a great experience, and I will definitely be participating in more game jams in the future!

If you are at all interested in game development, I highly recommend that you try participating in at least one game jam. It is a great way to learn new skills and meet new people. There are game jams happening all the time, so you should be able to find one that works with your schedule.

Source code for the game can be found on [GitHub](https://github.com/SquarePear/GGJ_2023).

The game page can be found on the [Global Game Jam website](https://v3.globalgamejam.org/2023/games/rooted-6-2).

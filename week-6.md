# Overview
Rest of book

## Lab Assignment
### Summary
From lab-assignment_1 INTEGRITY.md:

I tried to approach the assignment piece by piece and I believe a more top down approach would have been significantly better. One of the things I didn't realize about Rust was exactly how much refactoring of code was necessary, and I ended up spending longer on that than I did getting a (flawed) initial version of the program running. 

The other thing that I hadn't thought about was the "method first" trend that I noticed, particularly on iterators, in Rust. It can be difficult to keep track of what something like v.into_iter().rev().fold is actually doing on the backend. Not incredibly difficult but becuase I approached this like a C assignment there were a couple cases like that of twisted wires.

There was also the issue of understanding and setting up a rust "evironment." I wrote the majority of my code in main as I tried to get a trivial version of the assignment working, and once I did I tried running cargo test which did not go well. I think this is just a matter of experience and it wasn't a hard fix to port everything over to mod.rs but it was confusing at first.

Overall I think this assignment went fairly well for me, although trying to speed through things at the beginning definitely slowed me down at the end.

Again, I did end up using AI for debugging a specific error (as noted in INTEGRITY.md) and found that I needed to be implementing a Tuple Struct instead of a regular struct when it came to the declaration of DirectoryHandle, which seemed to be an issue that other students were running into.

## Book Assignment
### Summary
It's very funny that I ended up "programming" a multi-threaded http server as a large part of the higher level understanding of the project is directly portable to my cse-130 multi-threaded http server assignment. Being able to understand and implement what is essentially the exact same assignment in a different language a couple of weeks early has been very helpful for my understanding of the project.

That being said, this assignment was basically a "read a couple paragraphs and copy code" type of assignment, similar to the minigrep we implemented a couple weeks ago. It was interesting learning about thread pools and rust implementation of threads, certainly seems easier than in C, but I feel I'd have to work on the project enhancement ideas to feel I had a genuine understanding of all Rust has to offer when it comes to multi-threading.

## Reflection
There was a lot to do this week and it was very worthwile! Getting genuine experience with Rust, not via the pre-setup Rustlings assignments, was a good experience and it taught me that there's still a lot I have to learn about this language. In fact, if it were up to me I would request another lab assignment having to do with the initial stages of working with Twizzler so that I could have a better understanding of what exactly working with the OS would entail.
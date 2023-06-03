# todo

0. VSA HTML
- [X] Flatten
- [X] Remove empty
- [ ] Dedup
    - hard to efficiently reduce a join to a single node
- [ ] Place nodes in a reasonable starting position
    - some sort of bfs keeping track of position and depth?
- [ ] Label index of join edges
    - idk how
- [ ] Draw the lines more nicely

1. General algorithm
- start with a single node of unlearned with the goal
- click a node to run inverse semantics and add new unlearned nodes
- a way to extract a program with ??? for unlearned nodes
- a way to use forward semantics to assert human-created inverse semantics?? idk what this means, might be the same as
adding to vsa at a spot
    - like taking the inverse semantics for concat but instead of every combo just specify one except apply it to
    operators outside the language that may exist in python

2. other stuff
- a button to bottom up enumerate
- a way to manually add to bank in python (js? lua? rhai?)
- a way to automatically add a program to the vsa at a certain spot? is this useful?

# todo

0. VSA HTML
- [X] Flatten
- [X] Remove empty
- [X] Place nodes in a reasonable starting position
- [ ] Dedup
    - hard to efficiently reduce a join to a single node
    - use `VSA::contains` to check if siblings contain final AST
- [ ] Label index of join edges
    - idk how to do ui/ux for this
    - maybe different connection points instead of all at the center
- [ ] Draw the lines more nicely

Current process:
    1. compile VSA to HTML, preserving the tree structure
    2. Using the tree structure in the HTML, create an edge SVG node for each
       adjacency, and store an adjacency list
    3. Using the adjacency list, register onclick etc to drag the nodes and update the
       edges
    4. Place the nodes with dfs

1. General algorithm
- start with a single node of unlearned with the goal
- click a node to run inverse semantics and add new unlearned nodes
- a way to extract a program with ??? for unlearned nodes
- a way to use forward semantics to assert human-created inverse semantics?? idk what this means, might be the same as
adding to vsa at a spot
    - like taking the inverse semantics for concat but instead of every combo just specify one except apply it to
    operators outside the language that may exist in python
- start with e.g. "First Last" -> "FL"
    1. add VSA node `Execute(Python, "learn('F') + learn('L')"`)
    2. join node with 2 children which can be learned independently

2. other stuff
- a button to bottom up enumerate
- a way to manually add to bank in python (js? lua? rhai?)
- a way to automatically add a program to the vsa at a certain spot? is this useful?

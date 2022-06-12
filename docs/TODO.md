# Current work


# Current TODO
Next steps?

- Coils/OUTs

# Notes/Thoughts
I should keep ladder diagram nodes data as contact/coil, type, and _tag_
- Then a table of associated tags and memory?
- Or should it be _tag_ OR a direct memory address?

I should note potential improvements to the open source libs I'm using
- but prioritize other things
- Fe: Graph lib doesn't need performance, since our VMs will actually run bytecode or IL
- - Graph editing will mostly be from humans with interface IO speeds


# Future TODO
- Textual ladder parser?
- 2D ladder editor
- 3+ argument and/or combiners
- Function Blocks
- Ladder executor
- Propper IL output/generation

##Neighbor order
PROBLEM: I think processing ands is happening in the wrong order
  a & b & (c || d) & e is getting turned into: ((a & b) & ((c || d) & e)) instead of
                                               (((a & b) & (c || d)) & e)
This is "fixed" by restarting the visit on edit, however,
  I think that they may be logically equivalent?

I wonder if order of and/or should be based on some sort of Order of Operations?

What should be done to rebuild ladders with positional stability among builds?
  Maybe own visit/walker that sorts visit order between children based on a value?
  Should I ditch the graph library if I keep having to work around it?
- Sorted walkers (possibly based on node or edge weights and having them be comparable, or traits
- - This could allow for some nifty editing features, such as re-ordering things in an and/or,
but keeping them logically equivalent

# Done
- Clean
- Remove/Combine ladder and ast node types?
- Factor and and or combiners into one
- - Evaluate if removing nodes in combiners is best. Might be better to remove/add edges as needed
- Add type for our graph
- Do I need the "pools" when using a graph library?
- - Opting against for now

Removing pools, ids, and using cloneable node weights
- Works, with clones added generously
- Con - Added some Unknown/None variants and default... might need to remove these

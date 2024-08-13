# Garbage Collection
Garbage collection will be a generational gc that will try to reuse objects whenever possible. It should also be tunable to suit the needs of the user.


## How it works
1) All objects start off in the young generation space.
   * Once this space fills up, garbage collection takes place and objects that survive will be moved into the Survivor Space.
   * Objects that didn't survive will be moved to the Reuse Space. Here they wait to be reinitialized but if another collection of the Young Generation Space takes place, then the Reuse Space is cleared out.
2) Once in the Survivor Space, collection of the Survivor space won't take place until the Survivor Space is filled up. Unlike the Young Generation Space, objects will not be put in the reuse space.
   * If an object survived collection then they are moved to the Tenured Space which is much larger than the previous spaces.
3) Once in the Tenured Space, collection is likely not to occur.
4) The user is able to determine when garbage collection should occur. By default it occurs when each respective space runs out of space. However, the user may specify an interval in which GC will occur. This will cause collection to occur in this fashion
   * If a space runs out of space then collection will occur otherwise
   * Collect the Young Generation and Survivor Space on alternating intervals and the Tenured space every `nth` interval as specified by the user. 

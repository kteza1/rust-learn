Since we can’t allow one user of our chat service to block others, we need to isolate them somehow.

A common way is to create a separate thread for an each user so that blocking would take effect only in the context of a single thread. But while the concept is simple and such code is easy to write, each thread requires memory for its stack[6] and has an overhead of context switches — modern server CPUs usually have about 8 or 16 cores, and spawning many more threads requires an OS kernel scheduler to work hard to switch execution between them with an adequate speed. For this reason it’s hard to scale multithreading to many connections — in our case it’s barely practical (though certainly possible) to spawn several thousands of system threads, because we want to handle that many users without significant costs


So, instead we’ll use efficient I/O multiplexing system APIs that employ an event loop — that’s epoll on Linux[7] and kqueue on FreeBSD and OS X[8].

**These different APIs work similarly in a straightforward way: bytes are coming over the network, arriving at the sockets, and instead of waiting when the data becomes available for a read, we’re telling a socket to notify us when new data arrives.**

Instead of we manualy checking and waiting on 1000s of sockets, we'll write an event loop

```
loop {  //event loop
    select!{

    }

    // do other tasks
}
```

select block contains handling of network events which will be automaticaly selected when there are n/w events. Other wise we can happily continue doing other tasks

Interestingly enough, it works great not only for network communications but for disk I/O as well, as the event loop accepts all kinds of file handles (and sockets in the *nix world are just file handles too).

EDGE TRIGGERED VS LEVEL TRIGGERED EVENTS IN EVENT LOOP
---
In an edge-triggered system, if you want a notification to signal you when data is available to read, **you'll only get that notification when data was not available to read before, but now it is**. If you read some of the available data (so that remaining data is still available to read) you would not get another notification, and if you read all of the available data, then you would get another notification when data became available to read again. In a level-triggered system, you'd get that notification whenever data is available to read.



references: https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html
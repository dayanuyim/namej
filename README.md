An variant of `namei` that tranverses symlinks but displays the full path.

The flavor offered by `namej`:

```sh
$ namej /tmp/test/x
 * /tmp/test/x
   /tmp/test/x [l]
-> y
   /tmp/test/y [l]
-> ../test/z
   /tmp/test/z [l]
-> file
   /tmp/test/file [f]
```

compared to `namei`:
```sh
$ namei /tmp/test/x
f: /tmp/test/x
 d /
 d tmp
 d test
 l x -> y
   l y -> ../test/z
     d ..
     d test
     l z -> file
       - file
```

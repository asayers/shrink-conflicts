# Shrink git conflicts

`shrink-conflicts` is a program which tries to minimise diff3-style git
conflicts.

## Workflow

As I work on resolving a conflict, I slowly make all three parts (left,
right, and centre) more and more similar.  As I do this, I repeatedly pass
the conflict through `shrink-conflicts` to have it automatically simplified
(and possibly elliminated).  It's pretty nice.

`shrink-conflicts` reads data on stdin and prints it to stdout (ie. it's a
"filter").  This means you can easily use it from within your text editor:
just pipe the buffer through it.

## Credit

I got the idea from [git-mediate][].  `shrink-conflicts` is a re-implementation
of (part of) git-mediate's functionality.

[git-mediate]: https://github.com/Peaker/git-mediate

## Example

Here's lib.rs:

```rs
impl Conflict {
<<<<<<<
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
        // TODO: Should we be handling the common part too?
        self.left.is_empty() && self.right.is_empty()
    }
|||||||
    fn is_resolved(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }
=======
    fn is_resolved(&self) -> bool {
        self.left.is_empty() && self.common.is_empty() && self.right.is_empty()
    }
>>>>>>>
}
```

Running this file through `shrink-conflicts` will have no effect yet.
Let's copy the doc comment from the left part into the common and right parts.

```rs
impl Conflict {
<<<<<<<
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
        // TODO: Should we be handling the common part too?
        self.left.is_empty() && self.right.is_empty()
    }
|||||||
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }
=======
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
        self.left.is_empty() && self.common.is_empty() && self.right.is_empty()
    }
>>>>>>>
}
```

Now when we run this through `shrink-conflicts` it detects that all
three have a common prefix, and pulls it out:

```console
$ shrink-conflicts <lib.rs
impl Conflict {
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
<<<<<<<
        // TODO: Should we be handling the common part too?
        self.left.is_empty() && self.right.is_empty()
|||||||
        self.left.is_empty() && self.right.is_empty()
=======
        self.left.is_empty() && self.common.is_empty() && self.right.is_empty()
>>>>>>>
    }
}
```

Now, let's delete the TODO.

```rs
impl Conflict {
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
<<<<<<<
        self.left.is_empty() && self.right.is_empty()
|||||||
        self.left.is_empty() && self.right.is_empty()
=======
        self.left.is_empty() && self.common.is_empty() && self.right.is_empty()
>>>>>>>
    }
}
```

This time, when we run the file through `shrink-conflicts`, it detects that
the left and common parts are identical, and so selects the right part.

```console
$ shrink-conflicts <lib.rs
impl Conflict {
    /// Is this conflict resolved?
    fn is_resolved(&self) -> bool {
        self.left.is_empty() && self.common.is_empty() && self.right.is_empty()
    }
}
```

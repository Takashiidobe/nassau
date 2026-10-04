# <span id="section:0"></span>The `OS.Path` structure

---

#### Synopsis

<span id="OS_PATH:SIG:SPEC"></span>
<span id="Path:STR:SPEC"></span>

```sml
signature OS_PATH
structure Path : OS_PATH
```

The `OS.Path` structure provides support for manipulating the _syntax_ of file system paths independent of the underlying file system. It is purposely designed not to rely on any file system operations: none of the functions accesses the actual file system. There are two reasons for this design: many systems support multiple file systems that may have different semantics and applications may need to manipulate paths that do not exist in the underlying file system.

Before discussing the model of paths and the semantics of the individual operations, we need to define some terms:

- An _arc_ denotes a directory or file relative to the directory in which it is recorded. In a path string, arcs are separated by the arc separator character. This character is `#"/"` in Unix in Microsoft Windows, `#"\\"` is used. This character is `#"/"` in Unix in Microsoft Windows, both `#"\\"` and `#"/"` are allowed. For example, in Unix, the path `"abc/def"` contains two arcs: `"abc"` and `"def"`. There are two special arcs: [`parentArc`](os-path.md#SIG:OS_PATH.parentArc:VAL:SPEC) and [`currentArc`](os-path.md#SIG:OS_PATH.currentArc:VAL:SPEC). Under both Unix and Microsoft Windows, the [`parentArc`](os-path.md#SIG:OS_PATH.parentArc:VAL:SPEC) is `".."` and [`currentArc`](os-path.md#SIG:OS_PATH.currentArc:VAL:SPEC) is `"."`. An empty arc corresponds to an empty string.
  Although represented concretely as a string, an arc should be viewed as an abstraction in the context of the `OS.Path` structure, with a limited set of valid representations. In particular, a non-empty string `a` corresponds to _valid_ representation of an arc only if [`fromString`](os-path.md#SIG:OS_PATH.fromString:VAL:SPEC)` ``a` returns `{isAbs=false, vol="", arcs=[``a``]}`.

- A _path_ corresponds to a list of arcs, with an optional root, that denotes the path of directories leading to a file or directory in the file system hierarchy.

- An _absolute path_ has a root. Unix examples include `"/"` and `"/a/b"`; Microsoft Windows examples include `"\"`, `"\a\b"`, and `"A:\a\b"`.

- A _relative path_ is one without a root. Unix examples include `".."` and `"a/b"`; Microsoft Windows examples include `".."`, `"a\b"`, and `"A:a\b"`.

- A _canonical path_ contains no occurrences of the empty arc, no occurrences of the current arc unless the current arc is the only arc in the path, and contains parent arcs only at the beginning and only if the path is relative. Some examples of canonical paths, using Unix syntax, are as follows: `"."` `"/."`, `"/"`, `"a"`, `"a/b/c"`, `".."`, `"../a"`, `"../../a/b/c"`, and `"/a/b/c"`.

  > **System note \[WINDOWS\]:**
  >
  > In a Microsoft Windows implementation, canonical paths are entirely lowercase.

- A path has an associated _volume_. Under Unix, there is only one volume, whose name is `""`. Under Microsoft Windows, example volume names are `""`, `"A:"`, and `"C:"`.

In addition to operations for canonicalizing paths and computing relative paths, the [`Path`](os.md#SIG:OS.Path:STR:SPEC) structure supports path manipulations relative to three different views of a path:

1.  A navigation oriented view, where a path is broken down into its root and a non-empty list of arcs. A path is either absolute or relative. The root of a path specifies the volume to which the path is taken to be relative. For Unix, there is only the `""` volume.
2.  A directory/file view, where a path is broken down into a directory specifier and a filename.
3.  A base/extension view, where a path is broken down into a base filename and an extension. We make the assumption that the extension separator character is `#"."`. This works for Microsoft Windows, OS/2, and Unix; the Macintosh does not really have a notion of extension.

Our main design principle is that the functions should behave in a natural fashion when applied to _canonical_ paths. All functions, except [`concat`](os-path.md#SIG:OS_PATH.concat:VAL:SPEC), preserve canonical paths, _i.e._, if all arguments are canonical, then so is the result.

Note that although the model of path manipulation provided by the [`Path`](os.md#SIG:OS.Path:STR:SPEC) structure is operating system independent, the analysis of strings is not. In particular, any given implementation of the [`Path`](os.md#SIG:OS.Path:STR:SPEC) structure has an implicit notion of what the arc separator character is. Thus, on a Microsoft Windows system, [`Path`](os.md#SIG:OS.Path:STR:SPEC) will treat the string `"\\d\\e"` as representing an absolute path with two arcs, whereas on a Unix system, it will correspond to a relative path with one arc.

---

#### Interface

<span id="SIG:OS_PATH.Path:EXN:SPEC"></span>
<span id="SIG:OS_PATH.InvalidArc:EXN:SPEC"></span>
<span id="SIG:OS_PATH.parentArc:VAL:SPEC"></span>
<span id="SIG:OS_PATH.currentArc:VAL:SPEC"></span>
<span id="SIG:OS_PATH.fromString:VAL:SPEC"></span>
<span id="SIG:OS_PATH.toString:VAL:SPEC"></span>
<span id="SIG:OS_PATH.validVolume:VAL:SPEC"></span>
<span id="SIG:OS_PATH.getVolume:VAL:SPEC"></span>
<span id="SIG:OS_PATH.getParent:VAL:SPEC"></span>
<span id="SIG:OS_PATH.splitDirFile:VAL:SPEC"></span>
<span id="SIG:OS_PATH.joinDirFile:VAL:SPEC"></span>
<span id="SIG:OS_PATH.dir:VAL:SPEC"></span>
<span id="SIG:OS_PATH.file:VAL:SPEC"></span>
<span id="SIG:OS_PATH.splitBaseExt:VAL:SPEC"></span>
<span id="SIG:OS_PATH.joinBaseExt:VAL:SPEC"></span>
<span id="SIG:OS_PATH.base:VAL:SPEC"></span>
<span id="SIG:OS_PATH.ext:VAL:SPEC"></span>
<span id="SIG:OS_PATH.mkCanonical:VAL:SPEC"></span>
<span id="SIG:OS_PATH.isCanonical:VAL:SPEC"></span>
<span id="SIG:OS_PATH.mkAbsolute:VAL:SPEC"></span>
<span id="SIG:OS_PATH.mkRelative:VAL:SPEC"></span>
<span id="SIG:OS_PATH.isAbsolute:VAL:SPEC"></span>
<span id="SIG:OS_PATH.isRelative:VAL:SPEC"></span>
<span id="SIG:OS_PATH.isRoot:VAL:SPEC"></span>
<span id="SIG:OS_PATH.concat:VAL:SPEC"></span>
<span id="SIG:OS_PATH.fromUnixPath:VAL:SPEC"></span>
<span id="SIG:OS_PATH.toUnixPath:VAL:SPEC"></span>

```sml
exception Path
exception InvalidArc
val parentArc : string
val currentArc : string
val fromString : string -> {
isAbs : bool,
vol : string,
arcs : string list
}
val toString : {
isAbs : bool,
vol : string,
arcs : string list
} -> string
val validVolume : {isAbs : bool, vol : string} -> bool
val getVolume : string -> string
val getParent : string -> string
val splitDirFile : string -> {dir : string, file : string}
val joinDirFile : {dir : string, file : string} -> string
val dir : string -> string
val file : string -> string
val splitBaseExt : string -> {base : string, ext : string option
}
val joinBaseExt : {base : string, ext : string option} -> string
val base : string -> string
val ext : string -> string option
val mkCanonical : string -> string
val isCanonical : string -> bool
val mkAbsolute : {path : string, relativeTo : string} -> string
val mkRelative : {path : string, relativeTo : string} -> string
val isAbsolute : string -> bool
val isRelative : string -> bool
val isRoot : string -> bool
val concat : string * string -> string
val fromUnixPath : string -> string
val toUnixPath : string -> string
```

#### Description

<span id="SIG:OS_PATH.parentArc:VAL"></span>**`val`**` parentArc `**`:`**` string`  
The string denoting the parent directory (_e.g._, `".."` on Microsoft Windows and Unix).

<span id="SIG:OS_PATH.currentArc:VAL"></span>**`val`**` currentArc `**`:`**` string`  
The string denoting the current directory (_e.g._, `"."` on Microsoft Windows and Unix).

<span id="SIG:OS_PATH.fromString:VAL"></span>
`fromString ``path`` `  
returns the decomposition `{``isAbs``, ``vol``, ``arcs``}` of the path specified by `path`. `vol` is the volume name and `arcs` is the list of (possibly empty) arcs of the path. `isAbs` is `true` if the path is absolute. Under Unix, the volume name is always the empty string; under Microsoft Windows, in addition it can have the form `"A:"`, `"C:"`, etc.

Here are some examples for Unix paths:

---

`path`

`fromString ``path`

`""`

`{isAbs=false, vol="", arcs=[]}`

`"/"`

`{isAbs=true, vol="", arcs=[""]}`

`"//"`

`{isAbs=true, vol="", arcs=["", ""]}`

`"a"`

`{isAbs=false, vol="", arcs=["a"]}`

`"/a"`

`{isAbs=true, vol="", arcs=["a"]}`

`"//a"`

`{isAbs=true, vol="", arcs=["","a"]}`

`"a/"`

`{isAbs=false, vol="", arcs=["a", ""]}`

`"a//"`

`{isAbs=false, vol="", arcs=["a", "", ""]}`

`"a/b"`

`{isAbs=false, vol="", arcs=["a", "b"]}`

---


<span id="SIG:OS_PATH.toString:VAL"></span>
`toString {``isAbs``, ``vol``, ``arcs``} `  
makes a string out of a path represented as a list of arcs. `isAbs` specifies whether or not the path is absolute, and `vol` provides a corresponding volume. It returns `""` when applied to `{isAbs=false, vol="", arcs=[]}`. The exception [`Path`](os-path.md#SIG:OS_PATH.Path:EXN:SPEC) is raised if [`validVolume`](os-path.md#SIG:OS_PATH.validVolume:VAL:SPEC)`{``isAbs``, ``vol``}` is `false`, or if `isAbs` is `false` and `arcs` has an initial empty arc. The exception [`InvalidArc`](os-path.md#SIG:OS_PATH.InvalidArc:EXN:SPEC) is raised if any component in `arcs` is not a valid representation of an arc. The exception [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) is raised if the resulting string would have size greater than [`String.maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).

[`toString`](os-path.md#SIG:OS_PATH.toString:VAL:SPEC)`o`[`fromString`](os-path.md#SIG:OS_PATH.fromString:VAL:SPEC) is the identity. [`fromString`](os-path.md#SIG:OS_PATH.fromString:VAL:SPEC)`o`[`toString`](os-path.md#SIG:OS_PATH.toString:VAL:SPEC) is also the identity, provided no exception is raised and none of the strings in `arcs` contains an embedded arc separator character. In addition, [`isRelative`](os-path.md#SIG:OS_PATH.isRelative:VAL:SPEC)`(`[`toString`](os-path.md#SIG:OS_PATH.toString:VAL:SPEC)` {isAbs=false, vol, arcs})` evaluates to `true` when defined.

<span id="SIG:OS_PATH.validVolume:VAL"></span>
`validVolume {``isAbs``, ``vol``} `  
returns `true` if `vol` is a valid volume name for an absolute or relative path, respectively as `isAbs` is `true` or `false`. Under Unix, the only valid volume name is `""`. Under Microsoft Windows, the valid volume names have the form `"a:"`, `"A:"`, `"b:"`, `"B:"`, etc. and, if `isAbs`` = false`, also `""`. Under MacOS, `isAbs` can be `true` if and only if `vol` is `""`.

<span id="SIG:OS_PATH.getVolume:VAL"></span>
`getVolume ``path`` `  
returns the volume portion of the path `path`.

<span id="SIG:OS_PATH.getParent:VAL"></span>
`getParent ``path`` `  
returns a string denoting the parent directory of `path`. It holds that [`getParent`](os-path.md#SIG:OS_PATH.getParent:VAL:SPEC)` ``path`` = ``path` if and only if `path` is a root. If the last arc is empty or the parent arc, then [`getParent`](os-path.md#SIG:OS_PATH.getParent:VAL:SPEC) appends a parent arc. If the last arc is the current arc, then it is replaced with the parent arc. Note that if `path` is canonical, then the result of [`getParent`](os-path.md#SIG:OS_PATH.getParent:VAL:SPEC) will also be canonical.

Here are some examples for Unix paths:

---

`path`

`getParent ``path`

`"/"`

`"/"`

`"a"`

`"."`

`"a/"`

`"a/.."`

`"a///"`

`"a///.."`

`"a/b"`

`"a"`

`"a/b/"`

`"a/b/.."`

`".."`

`"../.."`

`"."`

`".."`

`""`

`".."`

---


<span id="SIG:OS_PATH.splitDirFile:VAL"></span>
`splitDirFile ``path`` `  
splits the string path `path` into its directory and file parts, where the file part is defined to be the last arc. The file will be `""`, if the last arc is `""`.

Here are some examples for Unix paths:

---

`path`

`splitDirFile ``path`

`""`

`{dir = "", file = ""}`

`"."`

`{dir = "", file = "."}`

`"b"`

`{dir = "", file = "b"}`

`"b/"`

`{dir = "b", file = ""}`

`"a/b"`

`{dir = "a", file = "b"}`

`"/a"`

`{dir = "/", file = "a"}`

---


<span id="SIG:OS_PATH.joinDirFile:VAL"></span>
`joinDirFile {``dir``, ``file``} `  
creates a whole path out of a directory and a file by extending the path `dir` with the arc `file`. If the string `file` does not correspond to an arc, raises [`InvalidArc`](os-path.md#SIG:OS_PATH.InvalidArc:EXN:SPEC). The exception [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) is raised if the resulting string would have size greater than [`String.maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).

<span id="SIG:OS_PATH.dir:VAL"></span>
`dir ``path`` `
` file ``path`` `  
return the directory and file parts of a path, respectively. They are equivalent to `#dir o `[`splitDirFile`](os-path.md#SIG:OS_PATH.splitDirFile:VAL:SPEC) and `#file o `[`splitDirFile`](os-path.md#SIG:OS_PATH.splitDirFile:VAL:SPEC), respectively, although they are probably more efficient.

<span id="SIG:OS_PATH.splitBaseExt:VAL"></span>
`splitBaseExt ``path`` `  
splits the path `path` into its base and extension parts. The extension is a non-empty sequence of characters following the right-most, non-initial, occurrence of `"."` in the last arc; [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned if the extension is not defined. The base part is everything to the left of the extension except the final `"."`. Note that if there is no extension, a terminating `"."` is included with the base part.

Here are some examples for Unix paths:

---

`path`

`splitBaseExt ``path`

`""`

`{base = "", ext = NONE}`

`".login"`

`{base = ".login", ext = NONE}`

`"/.login"`

`{base = "/.login", ext = NONE}`

`"a"`

`{base = "a", ext = NONE}`

`"a."`

`{base = "a.", ext = NONE}`

`"a.b"`

`{base = "a", ext = SOME "b"}`

`"a.b.c"`

`{base = "a.b", ext = SOME "c"}`

`".news/comp"`

`{base = ".news/comp", ext = NONE} `

---


<span id="SIG:OS_PATH.joinBaseExt:VAL"></span>
`joinBaseExt {``base``, ``ext``} `  
returns an arc composed of the base name and the extension (if different from [`NONE`](option.md#SIG:OPTION.option:TY:SPEC)). It is a left inverse of [`splitBaseExt`](os-path.md#SIG:OS_PATH.splitBaseExt:VAL:SPEC), _i.e._, [`joinBaseExt`](os-path.md#SIG:OS_PATH.joinBaseExt:VAL:SPEC)`o`[`splitBaseExt`](os-path.md#SIG:OS_PATH.splitBaseExt:VAL:SPEC) is the identity. The opposite does not hold, since the extension may be empty, or may contain extension separators. Note that although [`splitBaseExt`](os-path.md#SIG:OS_PATH.splitBaseExt:VAL:SPEC) will never return the extension [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`("")`, [`joinBaseExt`](os-path.md#SIG:OS_PATH.joinBaseExt:VAL:SPEC) treats this as equivalent to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). The exception [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) is raised if the resulting string would have size greater than [`String.maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).

<span id="SIG:OS_PATH.base:VAL"></span>
`base ``path`` `
` ext ``path`` `  
These return the base and extension parts of a path, respectively. They are equivalent to `#base o `[`splitBaseExt`](os-path.md#SIG:OS_PATH.splitBaseExt:VAL:SPEC) and `#ext o `[`splitBaseExt`](os-path.md#SIG:OS_PATH.splitBaseExt:VAL:SPEC), respectively, although they are probably more efficient.

<span id="SIG:OS_PATH.mkCanonical:VAL"></span>
`mkCanonical ``path`` `  
returns the canonical path equivalent to `path`. Redundant occurrences of the parent arc, the current arc, and the empty arc are removed. The canonical path will never be the empty string; the empty path is converted to the current directory path (`"."` under Unix and Microsoft Windows).

Note that the syntactic canonicalization provided by [`mkCanonical`](os-path.md#SIG:OS_PATH.mkCanonical:VAL:SPEC) may not preserve file system meaning in the presence of symbolic links (see [`concat`](os-path.md#SIG:OS_PATH.concat:VAL:SPEC)).

<span id="SIG:OS_PATH.isCanonical:VAL"></span>
`isCanonical ``path`` `  
returns `true` if `path` is a canonical path. It is equivalent to `(``path`` = `[`mkCanonical`](os-path.md#SIG:OS_PATH.mkCanonical:VAL:SPEC)` ``path``)`.

<span id="SIG:OS_PATH.mkAbsolute:VAL"></span>
`mkAbsolute {``path``, ``relativeTo``} `  
returns an absolute path that is equivalent to the path `path` relative to the absolute path `relativeTo`. If `path` is already absolute, it is returned unchanged. Otherwise, the function returns the canonical concatenation of `relativeTo` with `path`, _i.e._, [`mkCanonical`](os-path.md#SIG:OS_PATH.mkCanonical:VAL:SPEC)` (`[`concat`](os-path.md#SIG:OS_PATH.concat:VAL:SPEC)` (``abs``, ``p``))`. Thus, if `path` and `relativeTo` are canonical, the result will be canonical. If `relativeTo` is not absolute, or if the two paths refer to different volumes, then the [`Path`](os-path.md#SIG:OS_PATH.Path:EXN:SPEC) exception is raised. The exception [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) is raised if the resulting string would have size greater than [`String.maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).

<span id="SIG:OS_PATH.mkRelative:VAL"></span>
`mkRelative {``path``, ``relativeTo``} `  
returns a relative path `p` that, when taken relative to the canonical form of the absolute path `relativeTo`, is equivalent to the path `path`. If `path` is relative, it is returned unchanged. If `path` is absolute, the procedure for computing the relative path is to first compute the canonical form `abs` of `relativeTo`. If `path` and `abs` are equal, then the current arc is the result. Otherwise, the common prefix is stripped from `path` and `abs` giving `p'` and `abs'`. The resulting path is then formed by appending `p'` to a path consisting of one parent arc for each arc in `abs'`. Note that if both paths are canonical, then the result will be canonical.

If `relativeTo` is not absolute, or if `path` and `relativeTo` are both absolute but have different roots, the [`Path`](os-path.md#SIG:OS_PATH.Path:EXN:SPEC) exception is raised. The exception [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) is raised if the resulting string would have size greater than [`String.maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).

Here are some examples for Unix paths:

---

`path`

`relativeTo`

`mkRelative{``path``, ``relativeTo``}`

`"a/b"`

`"/c/d"`

`"a/b"`

`"/"`

`"/a/b/c"`

`"../../.."`

`"/a/b/"`

`"/a/c"`

`"../b/"`

`"/a/b"`

`"/a/c"`

`"../b"`

`"/a/b/"`

`"/a/c/"`

`"../b/"`

`"/a/b"`

`"/a/c/"`

`"../b"`

`"/"`

`"/"`

`"."`

`"/"`

`"/."`

`"."`

`"/"`

`"/.."`

`"."`

`"/a/b/../c"`

`"/a/d"`

`"../b/../c"`

`"/a/b"`

`"/c/d"`

`"../../a/b"`

`"/c/a/b"`

`"/c/d"`

`"../a/b"`

`"/c/d/a/b"`

`"/c/d"`

`"a/b"`

---


<span id="SIG:OS_PATH.isAbsolute:VAL"></span>
`isAbsolute ``path`` `
` isRelative ``path`` `  
These return `true` if `path` is, respectively, absolute or relative.

<span id="SIG:OS_PATH.isRoot:VAL"></span>
`isRoot ``path`` `  
returns `true` if `path` is a canonical specification of a root directory.

<span id="SIG:OS_PATH.concat:VAL"></span>
`concat (``path``, ``t``) `  
returns the path consisting of `path` followed by `t`. It raises the exception [`Path`](os-path.md#SIG:OS_PATH.Path:EXN:SPEC) if `t` is not a relative path or if `path` and `t` refer to different volumes. The exception [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) is raised if the resulting string would have size greater than [`String.maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).

A implementation of `concat` might be:

fun concat (p1, p2) = (case (fromString p1, fromString p2)
       of (\_, {isAbs=true, ...}) =\> raise Path
        \| ({isAbs, vol=v1, arcs=al1},
           {vol=v2, arcs=al2, ...}
          ) =\> if ((v2 = "") orelse (v1 = v2))
              then toString{
                  isAbs=isAbs, vol=v1,
                  arcs=concatArcs(al1, al2)
                }
              else raise Path
      (\* end case \*))

where `concatArcs` is like [`List.@`](list.md#SIG:LIST.@:VAL:SPEC), except that a trailing empty arc in the first argument is dropped. Note that [`concat`](os-path.md#SIG:OS_PATH.concat:VAL:SPEC) should not be confused with the concatenation of two strings.

[`concat`](os-path.md#SIG:OS_PATH.concat:VAL:SPEC) does not preserve canonical paths. For example, [`concat`](os-path.md#SIG:OS_PATH.concat:VAL:SPEC)`("a/b", "../c")` returns `"a/b/../c"`. The parent arc is not removed because `"a/b/../c"` and `"a/c"` may not be equivalent in the presence of symbolic links.

<span id="SIG:OS_PATH.fromUnixPath:VAL"></span>
`fromUnixPath ``s`` `  
converts the Unix-style path `s` to the path syntax of the host operating system. Slash characters are translated to the directory separators of the local system, as are parent arcs and current arcs. This function raises the [`InvalidArc`](os-path.md#SIG:OS_PATH.InvalidArc:EXN:SPEC) exception if any arc in the Unix path is invalid in the host system's path syntax (_e.g._, an arc that has a backslash character in it when the host system is Microsoft Windows).

Note that the syntax of Unix pathnames necessarily limits this function. It is not possible to specify paths that have a non-empty volume name or paths that have a slash in one of their arcs using this function.

<span id="SIG:OS_PATH.toUnixPath:VAL"></span>
`toUnixPath ``s`` `  
converts the path `s`, which is in the host operating system's syntax, to a Unix-style path. If the path `s` has a non-empty volume name, then the [`Path`](os-path.md#SIG:OS_PATH.Path:EXN:SPEC) exception is raised. Also, if any arc in the pathname contains the slash character, then the [`InvalidArc`](os-path.md#SIG:OS_PATH.InvalidArc:EXN:SPEC) exception is raised.

#### Examples

```repl
OS.Path.concat ("src", "main.sml");;
```

#### See Also

> [`OS`](os.md#OS:STR:SPEC), [`OS.FileSys`](os.md#SIG:OS.FileSys:STR:SPEC), [`OS.IO`](os.md#SIG:OS.IO:STR:SPEC), [`OS.Process`](os.md#SIG:OS.Process:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC)

#### Discussion

Syntactically, two paths can be checked for equality by applying string equality to canonical versions of the paths. Since volumes and individual arcs are just special classes of paths, an identical test for equality can be applied to these classes.

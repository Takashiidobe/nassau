# <span id="section:0"></span>The `TEXT` signature

---

#### Synopsis

<span id="TEXT:SIG:SPEC"></span>
<span id="Text:STR:SPEC"></span>
<span id="WideText:STR:SPEC"></span>

```sml
signature TEXT
structure Text :> TEXT
where type Char.char = Char.char
where type String.string = String.string
where type Substring.substring = Substring.substring
where type CharVector.vector = CharVector.vector
where type CharArray.array = CharArray.array
where type CharVectorSlice.slice = CharVectorSlice.slice
where type CharArraySlice.slice = CharArraySlice.slice
structure WideText :> TEXT (* OPTIONAL *)
where type Char.char = WideChar.char
where type String.string = WideString.string
where type Substring.substring = WideSubstring.substring
where type CharVector.vector = WideCharVector.vector
where type CharArray.array = WideCharArray.array
where type CharVectorSlice.slice = WideCharVectorSlice.slice
where type CharArraySlice.slice = WideCharArraySlice.slice
```

The `TEXT` signature collects together various text related structures based on the representation of the shared character type.

The [`TEXT`](text.md#TEXT:SIG:SPEC) signature is matched by two structures, the required [`Text`](text.md#Text:STR:SPEC) and the optional [`WideText`](text.md#WideText:STR:SPEC). The former implements strings based on the extended ASCII 8-bit characters. The latter provides strings of characters of some size greater than or equal to 8 bits.

---

#### Interface

```sml
structure Char : CHAR
structure String : STRING
structure Substring : SUBSTRING
structure CharVector : MONO_VECTOR
structure CharArray : MONO_ARRAY
structure CharVectorSlice : MONO_VECTOR_SLICE
structure CharArraySlice : MONO_ARRAY_SLICE
sharing type Char.char = String.char = Substring.char
= CharVector.elem = CharArray.elem = CharVectorSlice.elem
= CharArraySlice.elem
sharing type Char.string = String.string = Substring.string
= CharVector.vector = CharArray.vector
= CharVectorSlice.vector = CharArraySlice.vector
sharing type CharArray.array = CharArraySlice.array
sharing type CharVectorSlice.slice
= CharArraySlice.vector_slice
```

#### Examples

```repl
Text.String.explode "Nassau";;
```

#### See Also

> [`CHAR`](char.md#CHAR:SIG:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`STRING`](string.md#STRING:SIG:SPEC), [`SUBSTRING`](substring.md#SUBSTRING:SIG:SPEC)

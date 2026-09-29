/* Nassau's runtime library, linked into compiled programs and into the
 * compiler itself for the REPL's JIT. Values follow
 * docs/value-representation.md. */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef int64_t word;

#define TAG(n) ((((word)(n)) << 1) | 1)
#define UNTAG(w) ((w) >> 1)
#define IS_BOXED(w) (((w) & 1) == 0)
#define HEADER(length, kind) ((((word)(length)) << 8) | (kind))
#define LENGTH(header) ((header) >> 8)
#define KIND(header) ((header) & 0xff)
#define FIELD(block, index) (((word *)(block))[(index) + 1])

enum { KIND_RECORD = 0, KIND_CLOSURE = 1, KIND_STRING = 2, KIND_REAL = 3, KIND_REF = 4 };

#define CHUNK (1 << 20)

static char *heap_next;
static char *heap_end;

/* A block with room for `fields` words after its header, which the caller
 * fills in. Blocks are never freed until a collector exists. */
word *nassau_alloc(word fields) {
    size_t bytes = (size_t)(fields + 1) * sizeof(word);
    if (heap_next == NULL || (size_t)(heap_end - heap_next) < bytes) {
        size_t size = bytes > CHUNK ? bytes : CHUNK;
        heap_next = malloc(size);
        if (heap_next == NULL) {
            fputs("nassau: out of memory\n", stderr);
            exit(1);
        }
        heap_end = heap_next + size;
    }
    word *block = (word *)heap_next;
    heap_next += bytes;
    return block;
}

static word string_block(const char *bytes, size_t length) {
    word *block = nassau_alloc((word)(length / sizeof(word) + 1));
    block[0] = HEADER(length, KIND_STRING);
    memcpy(block + 1, bytes, length);
    ((char *)(block + 1))[length] = '\0';
    return (word)block;
}

static const char *string_bytes(word string) { return (const char *)((word *)string + 1); }

static size_t string_length(word string) { return (size_t)LENGTH(((word *)string)[0]); }

word nassau_print(word string) {
    fwrite(string_bytes(string), 1, string_length(string), stdout);
    fflush(stdout);
    return TAG(0);
}

word nassau_concat(word lhs, word rhs) {
    size_t left = string_length(lhs);
    size_t right = string_length(rhs);
    word *block = nassau_alloc((word)((left + right) / sizeof(word) + 1));
    block[0] = HEADER(left + right, KIND_STRING);
    memcpy(block + 1, string_bytes(lhs), left);
    memcpy((char *)(block + 1) + left, string_bytes(rhs), right);
    ((char *)(block + 1))[left + right] = '\0';
    return (word)block;
}

/* Int.toString: negative numbers use SML's `~`. */
word nassau_int_to_string(word integer) {
    char text[32];
    long long value = (long long)UNTAG(integer);
    int length = value < 0 ? snprintf(text, sizeof text, "~%lld", -value)
                           : snprintf(text, sizeof text, "%lld", value);
    return string_block(text, (size_t)length);
}

/* Structural equality. Immediates (ints, chars, nullary constructors) are
 * equal when their words are; blocks of the same kind and length compare
 * their contents. The last field of a record is compared by looping rather
 * than recursing, so a long list takes constant stack. */
static int equal(word lhs, word rhs) {
    for (;;) {
        if (lhs == rhs) {
            return 1;
        }
        if (!IS_BOXED(lhs) || !IS_BOXED(rhs)) {
            return 0;
        }
        word header = ((word *)lhs)[0];
        if (header != ((word *)rhs)[0]) {
            return 0;
        }
        switch (KIND(header)) {
        case KIND_STRING:
            return memcmp(string_bytes(lhs), string_bytes(rhs), (size_t)LENGTH(header)) == 0;
        case KIND_REAL: {
            double left, right;
            memcpy(&left, (word *)lhs + 1, sizeof left);
            memcpy(&right, (word *)rhs + 1, sizeof right);
            return left == right;
        }
        case KIND_RECORD: {
            word length = LENGTH(header);
            if (length == 0) {
                return 1;
            }
            for (word index = 0; index + 1 < length; index++) {
                if (!equal(FIELD(lhs, index), FIELD(rhs, index))) {
                    return 0;
                }
            }
            lhs = FIELD(lhs, length - 1);
            rhs = FIELD(rhs, length - 1);
            continue;
        }
        default:
            /* References are equal only when identical; functions have no
             * equality. */
            return 0;
        }
    }
}

/* Polymorphic structural equality, `=` on any equality type. */
word nassau_equal(word lhs, word rhs) { return TAG(equal(lhs, rhs)); }

/* The basis's exceptions, in the order of value::BUILTIN_EXCEPTIONS. An
 * exception's identity is a reference cell holding its name: each
 * evaluation of an `exception` declaration allocates a new one, and these
 * are allocated once, on first use. */
static const char *const builtin_names[] = {"Div",  "Overflow",  "Match", "Bind",
                                            "Fail", "Subscript", "Empty"};
static word builtin_identities[sizeof builtin_names / sizeof builtin_names[0]];

word nassau_exception(word index) {
    word slot = UNTAG(index);
    if (builtin_identities[slot] == 0) {
        const char *name = builtin_names[slot];
        word *identity = nassau_alloc(1);
        identity[0] = HEADER(1, KIND_REF);
        identity[1] = string_block(name, strlen(name));
        builtin_identities[slot] = (word)identity;
    }
    return builtin_identities[slot];
}

/* The exception being raised, and the position SML/NJ reports it at. A
 * function that raises records its exception here and returns 0, which is
 * no value; each caller then goes to its handler or returns 0 in turn. */
static word raised;
static const char *raised_at;

/* `where` is null when a handler passes an exception on: it keeps the
 * position it was first raised at. */
void nassau_raise_exception(word exception, const char *where) {
    raised = exception;
    if (where != NULL) {
        raised_at = where;
    }
}

/* The exception a handler catches. */
word nassau_caught(void) {
    word exception = raised;
    raised = 0;
    return exception;
}

/* Reports the exception no handler caught as SML/NJ does; returns the exit
 * status. */
int nassau_uncaught(void) {
    word exception = nassau_caught();
    word identity = FIELD(exception, 0);
    word name = FIELD(identity, 0);
    word argument = FIELD(exception, 1);
    fflush(stdout);
    fprintf(stderr, "/usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception %.*s with ",
            (int)string_length(name), string_bytes(name));
    if (!IS_BOXED(argument)) {
        fprintf(stderr, "%lld", (long long)UNTAG(argument));
    } else if (KIND(((word *)argument)[0]) == KIND_STRING) {
        fprintf(stderr, "\"%.*s\"", (int)string_length(argument), string_bytes(argument));
    } else {
        fputs("<unknown>", stderr);
    }
    fprintf(stderr, "\n raised at %s\n\n", raised_at);
    return 1;
}

/* Reports an uncaught exception as SML/NJ does and exits. `where` is the
 * position SML/NJ names as the raise point. */
void nassau_raise(const char *name, const char *where) {
    fflush(stdout);
    fprintf(stderr,
            "/usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception %s with 0\n"
            " raised at %s\n\n",
            name, where);
    exit(1);
}

/* Posix.Process.exit (Word8.fromInt status). */
void nassau_exit(word status) {
    fflush(stdout);
    exit((int)(UNTAG(status) & 0xff));
}

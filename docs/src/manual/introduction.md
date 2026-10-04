# <span id="section:0"></span>Introduction

These web pages contain the interface specifications for the modules of the **SML Basis Library**, which is a standard library for the 1997 Revision of SML**\[CITE\]**. The SML Basis Library provides interfaces and operations for basic types, such as integers and strings, support for input and output (I/O), interfaces to basic operating system interfaces, and support for standard datatypes, such as options and lists. The Library does _not_ attempt to define higher-level APIs, such as collection types or graphical user-interface components. These APIs are left for other libraries.

The SML Basis Library is also published as a book by _[Cambridge University Press](http://www.cup.org)_. In addition to the manual pages, the book also contains tutorial descriptions of programming techniques and idioms for effective use of the Library's interfaces.

The design philosophy of the SML Basis Library is to use the SML module system as an organizing tool. All type, exception, and value identifiers are bound in some module. A small number of these, which are called _pervasive identifiers_, are also bound at top-level (_i.e._, without qualification). In addition, the top-level environment defines overloading of some identifiers.

The components (_i.e._, signatures, structures, and functors) of the SML Basis Library are divided into _required_ and _optional_ components. Required components must be provided by all SML implementations, while optional components are just that. In some cases, support for one optional component entails providing support for others.

These webpages are organized as follows: the overview page describes the different kinds of interfaces provided by the SML Basis Library as well as which components are required and optional. The bulk of this document is a collection of _manual pages_, each of which describes an interface (_i.e._, signature) and its implementations.

### [SML Basis Library Overview](overview.md)

### [Top-level Environment](top-level-environment.md)

### [SML Basis Manual Pages](manual-pages.md)

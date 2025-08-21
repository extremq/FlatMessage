# Serialization Compatibility

FlatMessage provides comprehensive compatibility features that allow your data structures to evolve over time while maintaining interoperability between different versions. This chapter covers the key mechanisms for ensuring your serialized data can be safely shared between applications using different versions of the same message structures.

## Overview

When working with serialized data in production systems, you often need to handle scenarios where:

- **Newer applications** need to read data serialized by older versions
- **Older applications** need to read data serialized by newer versions
- **Different services** are running different versions of your data structures
- **Data migration** requires careful compatibility planning

FlatMessage addresses these challenges through several key features:
* **Version Control**: Every FlatMessage structure can be assigned a version number and specify which versions it's compatible with. This provides explicit control over backward and forward compatibility.

* **Flexible Field Management**: Fields can be marked as mandatory or optional, with support for default values when fields are missing. This enables safe addition and modification of structure fields.

* **Sealed vs Non-Sealed Types**: Enums, flags, and variants can be sealed (strict compatibility) or non-sealed (forward compatible), giving you control over how these types evolve.











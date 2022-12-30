# Common
## Overview


The Common pallet provides functions for:

 - Get\set\delete allow list.
 - Get\set\delete collection properties.
 - Get\set\delete collection property permissions.
 - Get\set\delete token property permissions.
 - Get\set\delete collection administrators.
 - Checking access permissions.

### Terminology
**Collection sponsor** - For the collection, you can set a sponsor, at whose expense it will
be possible to mint tokens.

**Allow list** - List of users who have the right to minting tokens.

**Collection properties** - Collection properties are simply key-value stores where various
metadata can be placed.

**Permissions on token properties** - For each property in the token can be set permission
to change, see [`PropertyPermission`].

**Collection administrator** - For a collection, you can set administrators who have the right
to most actions on the collection.

# Whitelist pallet
 
## Overview
 
The goal of this module is to control Security Assets possesion
and transfering by listing priviliged accounts (so called Investors) able to manage
Security Assets.
 
Whitelisted investors and their statuses (active or not) are controlled by
special roles which are: `RolesRoot`, `Admin` and `Manager`

### Terminology
- `RolesRoot` is able to assign and resign `Admin`
- `Admin` is the most priveleged role with plenty of control possibilities
- `Manager` is able to control investors statuses
## Interface
### Dispatchable Functions
- `add_admin` - Adding admin to Whitelist.
- `remove_admin` - Removing admin from Whitelist.
- `add_manager` - Adding manager to Whitelist.
- `remove_manager` - Removing manager from Whitelist.
- `add_investors` - Adding investor to Whitelist.
- `set_investor_status` - Set investor status(actice or not).
- `change_investor_address` - Admin change adress of investor in Whitelist.
- `change_my_address` - Investor change his own adress in Whitelist
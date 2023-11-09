### **About**
This package is used for pallets API testing
For now just only simple success cases are covered to ensure that API is working
Maybe the whole testing process will be moved here
### **Install**
```bash
npm install
```
### **Metadata and type definitions**
Before running tests you should download actual metadata from running node that you will be running tests on. Then you should generate type definitions with fresh metadata. Actually it is done automatically when you run one of the testing commands such as `test`, `pallettest` or `calltest`. But to make things work you should specify correct http endpoint in your `.env` file as in `.example.env`

Also you are able to do this manually. To get metadata run
```bash
npm run getmetadata
```
Then to generate type definitions run
```bash
npm build
```
### **Run tests**
**Before running tests**

Before run the tests you must specify correct endpoints in your `.env` file as in `.example.env`

**All tests**

To run all tests for all pallets
```bash
npm test
```
**Specific pallet**

You can also run tests for specific pallet using
```bash
npm run pallettest --pallet=palletName
```
> :warning: It is important to pass parameters with `=` sign as `--pallet=palletName` not `--pallet palletName`. Otherwise parameters could be parsed incorrectly	
Where palletName is name of pallet and must be equal to name of corresponding directory with tests which are  located under `src/`
For example if you want to run all tests for refungible pallet which are located in `src/refungible` you must run
```bash
npm run pallettest --pallet=refungible
```
**Specific call**

Tests for each extrinsic located on it's own file (some similar extrinsics like set_property and set_properties are grouped) named like `callName.test.js` under it's pallet directory. To run tests for specific use
```bash
npm run calltest --pallet=palletName --call=callName
```
Where palletName is name of folder as mentioned above and callName is camel-cased extrinsic name and equal to the first part of the corresponding filename.
For example if you want to run tests for create_item extrinsic of refungible pallet you must ensure that there is createItem.test.js file in `src/refungible` and run
```bash
npm run calltest --pallet=refungible --call=createItem
```



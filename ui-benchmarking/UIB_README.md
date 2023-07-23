# Adjustments for testing
In the files `ui-benchmark-localhost.js` and `ui-benchmark-dockerized.js`,
the amount of requested objects and gathering rounds can be adjusted. \
In lines;
```javascript
const ROUNDS = 3;
const OBJECTS_MAX_AMOUNT = 10000;
```
By default the values are set to 
```typescript
const ROUNDS = 3;
const OBJECTS_MAX_AMOUNT = 5000;

```

# How to run the ui-benchmark when the servers are running locally
1. Install python dependencies
```
pip3 install -r requirements.txt
```
2. Install node dependencies
```
npm install
```
3. Run the benchmark
```
node ./ui-benchmark-localhost.js
```
4. Run the python script that generated the graph from the csv files
```
python3 ./generate_graphs.py
```
The script will create two images representing the parsng results for two object types.\
The results (data files in `.csv` files) will be present in the directory (`./ui-benchmark_<DATE_OF_EXECUTION>`)

# How to run the ui-benchmark when the servers are running using Docker

1. Install python dependencies
```
pip3 install -r requirements.txt
```
2. Install node dependencies
```
npm install
```
3. Run the benchmark
```
node ./ui-benchmark-dockerized.js
```
The results (data files in `.csv` files) will be present in the directory (`./ui-benchmark_<DATE_OF_EXECUTION>`)

4. Run the python script that generated the graph from the csv files
```
python3 ./generate_graphs.py
```
The script will create two images representing the parsng results for two object types.\
The results (data files in `.csv` files) will be present in the directory (`./ui-benchmark_<DATE_OF_EXECUTION>`)

# Adjustments for testing
In the files `benchmark-localhost.py` and `benchmark-dockerized.py`,
you can adjust the amount of requested objects and gathering rounds.
```python
# for n in range (startValue, maxValue, step)
for n in range(10000, 1010000, 10000):
gathering_rounds = 10
```
By default the values are set to
```python
for n in range(100, 10000, 100):
gathering_rounds = 5
```
# How to run the server-benchmark when the servers are running locally
1. Install python dependencies
```
pip3 install -r requirements.txt
```
2. Run the benchmark
```
python3 ./benchmark-localhost.py
```
The results (graph images (extension `.png`), data used to plot the graphs (`.csv`) and execution logs (`benchmark-<DateTime>.log`) will be present in the directory (`./benchmark-<DateTime>`).

# How to run the server-benchmark when the servers are running using Docker

1. Install python dependencies
```
pip3 install -r requirements.txt
```
2. Run the benchmark
```
python3 ./benchmark-dockerized.py
```
The results (graph images (extension `.png`), data used to plot the graphs (`.csv`) and execution logs (`benchmark-<DateTime>.log`) will be present in the directory (`./benchmark-<DateTime>`).

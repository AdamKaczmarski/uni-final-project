import os
import requests
import json 
import matplotlib.pyplot as plt
import time
from datetime import datetime
import logging

#Global date tag that's used in the file names
DATE = datetime.today().strftime('%Y-%m-%d_%H_%M')

#If I would like to reduce the number of graphs or change something in the graphs I can reuse the data
def write_dict_to_csv(filename, dict_data):
    with open(filename, 'w') as f:
        f.write("amountOfRequestedObjects,parsingTime\n")
        for k,v in dict_data.items():
            f.write(f"{k},{v}\n")

#The function calls the triggerReqs endpoints on the specified server with required parameters.
#After sending the post request the script waits until results are received from the webserver.
#Those results are saved into the dict that's later used as data in the graph plotting.
def load_data(provider,label,url,format,tries,type):
    graph_data = dict()
    #Adding +1 because range doesn't include the ending number
    #Starting from one so we don't divide by 0 later
    for round in range(1,tries+1):
        print(f"ROUND {round} of {tries}")
        logging.info(f"ROUND {round} of {tries}")
        for n in range(0, 10000, 500):
        #for n in range(100, 300, 100): #For local testing
            payload = json.dumps({
                "amount": n,
                "providerUrl": provider,
                "objectFormat": format,
                "serverType": label,
                "objectType": type
                })
            headers = {
                    'Content-Type': 'application/json'
                    }
            response = requests.request("POST", url, headers=headers, data=payload)
            parsing_time = response.json()[f"{format.lower()}_parsing_time_micro_s"]/1000
            print(response.json())
            if n in graph_data:
                #Add the parsing time and average that
                graph_data[n] = (graph_data[n]+(parsing_time-graph_data[n]/round))
            else:
                #First round init the entry in the dict
                graph_data[n] = parsing_time 
    return graph_data

def main():
    #URLs are hardcoded for the easiness of benchmark execution
    requesters = {
            "rust": "http://localhost:8081/triggerReqs",
            "java": "http://localhost:8080/triggerReqs"
            }
    providers = {
            "rust": "http://localhost:8082",
            "java": "http://localhost:8079"
            }
    logging.info(requesters)
    logging.info(providers)
    #For each of the items run the benchmark (2 types, 2 formats, 2 providers to request data from) 
    for label, provider in providers.items():
        for type in ["NumericObject", "DummyObject"]:
            for format in ["JSON", "BSON"]:
                for lang, url in requesters.items():
                    logging.info(f"Sending to {lang} -- {url}")
                    logging.info(f"REQUEST: {lang}_{label}--{provider}_{format}")
                    gathering_rounds = 5
                    graph_data = load_data(provider, label, url, format, gathering_rounds,type)
                    write_dict_to_csv(f"./benchmark-{DATE}/{type}_{lang}_{format}_from_{label}-{DATE}.csv",graph_data)
                    if lang == "rust":
                        plt.plot(graph_data.keys(), graph_data.values(),label="Rust", color='#cf433c')
                    else:
                        plt.plot(graph_data.keys(), graph_data.values(),label="Java", color='#a87d32')
                    plt.title(f"{format.capitalize()} from {label.capitalize()}")
                    plt.xlabel(f"Number of objects\n Gathering rounds:{gathering_rounds}")
                    plt.ylabel("Average parsing time in ms")
                    plt.legend(loc="upper left")
                    plt.savefig(f"./benchmark-{DATE}/{type}_{format}_from_{label}-{DATE}.png", bbox_inches='tight')
                    logging.info("Sleeping for 15 seconds")
                    time.sleep(15)
                plt.close()
    logging.info("Finished benchmarking")
    print("Finished benchmarking")


if __name__ == "__main__":
    os.mkdir(f"./benchmark-{DATE}")
    logging.basicConfig(filename=f"./benchmark-{DATE}/benchmark-{DATE}.log", encoding='utf-8', format="%(asctime)s - %(levelname)s - %(message)s", datefmt="%d-%b-%y %H:%M:%S", level=logging.INFO)
    main()

import pandas as pd
import matplotlib.pyplot as plt
import glob

def main():
    for file in glob.glob("ui*/dummy*.csv"):
        with open(file,'r') as data_file:
            headers = ["amountOfRequestedObjects","parsingTime"]
            file_name=data_file.name.split("/")[1]
            title = file_name[:file_name.find('.')]
            df = pd.read_csv(data_file,usecols=headers)
            plt.plot(df.amountOfRequestedObjects,df.parsingTime,label=title)
    plt.title(f"TypeScript parsing times from both servers")
    plt.xlabel(f"Number of objects")
    plt.ylabel("Parsing time in ms")
    plt.legend(loc="upper left")
    plt.savefig(f"dummy_object.png", bbox_inches='tight')
    plt.close()
    for file in glob.glob("ui*/numeric*.csv"):
        with open(file,'r') as data_file:
            headers = ["amountOfRequestedObjects","parsingTime"]
            file_name=data_file.name.split("/")[1]
            title = file_name[:file_name.find('.')]
            df = pd.read_csv(data_file,usecols=headers)
            plt.plot(df.amountOfRequestedObjects,df.parsingTime,label=title)
    plt.title(f"TypeScript parsing times from both servers")
    plt.xlabel(f"Number of objects")
    plt.ylabel("Parsing time in ms")
    plt.legend(loc="upper left")
    plt.savefig(f"numeric_object.png", bbox_inches='tight')
    plt.close()

if __name__ == "__main__":
    main()

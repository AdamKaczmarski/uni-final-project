import puppeteer from 'puppeteer';
import fs from 'fs';
import path from 'path';

//3 rounds that calculate the average parsing time for each 100 of objects for more precise data
const ROUNDS = 3;
//The IDs of the buttons that the synthetic is clicking to trigger the parsing benchmark
const scenarios = ["numeric_java_json", "numeric_java_bson", "numeric_rust_json", "numeric_rust_bson","dummy_java_json", "dummy_java_bson", "dummy_rust_json", "dummy_rust_bson"];
//The max amount of objects that the typescript-client has to obtain and parse
const OBJECTS_MAX_AMOUNT = 5000;
// const OBJECTS_MAX_AMOUNT =200;

const benchmarkParsing = async (scenario, amnt) => {
    //timeout set to 0 in case of long waits for parsing results
    const browser = await puppeteer.launch({timeout:0,protocolTimeout:0});
    const page = await browser.newPage();
    page.setDefaultTimeout(0);
    page.setDefaultNavigationTimeout(0);
    //When apache2 serves the static files
    await page.goto('http://localhost:80');
    //I found that setting the viewport and making a screenshot prevents some UI issues that the client meets when opening the browser and loading the page
    page.setViewport({width:1080,height:1920}); //For screenshots
    await page.screenshot({path:"./test.png"}) //Debugging what was put into the input

    // Type into objects amount input
    await page.type('#objects_amount_input', amnt);
    //Search for the button and click it
    const searchResultSelector = '#' + scenario;
    await page.waitForSelector(searchResultSelector);
    await page.click(searchResultSelector);
    //Wait until the parsing time results show up at the bottom of the bage
    const textSelector = await page.waitForSelector(
        '#parsing_time'
    );
    const parsingTime = await textSelector.evaluate(el => el.textContent);

    await browser.close(); return +parsingTime; // + is a simple JS trick to convert a string to number
};
//Add new value to the average
const addToAverage = (oldAvg, newVal, rounds) => {
    return Math.round(oldAvg + ((newVal - oldAvg) / rounds), 0)

}
//Writing a map to a CSV file
const writeMapToCSV = (data, fileName) => {
    try {
        console.log(data)
        fs.writeFileSync(fileName, "amountOfRequestedObjects,parsingTime");
        data.forEach((parsingTime, amnt) => {  //value, key because JavaScript :))
            fs.appendFileSync(fileName, "\n" + amnt + "," + parsingTime);
        });
    } catch (err) {
        console.log(err)
    }
}
console.log("Starting the UI benchmark");
const date = new Date().toISOString().slice(0,16).replace('T','_').replace(':','-');
const pathDir = path.join("ui-benchmark_"+date);
//Create a directory that will hold the csv files
fs.mkdir(pathDir,(err)=>console.log(err));
//This loop executes over every scenario and for each 100 util OBJECTS_MAX_AMOUNT
//Every iteration opens a browser, types in the number of objects, clicks the button and waits for the parsing time that appears at the bottom of the page
for (let scenario of scenarios) {
    console.log("Running scenario ",scenario)
    const map = new Map();
    for (let i = 1; i <= ROUNDS; i++) {
        console.log(`Round: ${i}`)
        for (let amnt = 100; amnt <= OBJECTS_MAX_AMOUNT; amnt += 100) {
            let parsing_time = await benchmarkParsing(scenario, ''.concat(amnt));
            if (map.has(amnt)) {
                map.set(amnt, addToAverage(map.get(amnt), parsing_time, i))
            } else {
                map.set(amnt, parsing_time);
            }
        }
    }
    //Writing the results to the CSV generate the graphs using python
    writeMapToCSV(map, pathDir+"/"+scenario + ".csv");
}
console.log("UI Benchmark finished")


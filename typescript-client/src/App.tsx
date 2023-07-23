import './App.css'
import { BSON } from 'bson';
import { useRef, useEffect, useState } from 'react';
interface Coordinate {
    timestamp: number;
    latitude: number
    longitude: number
}
interface NumericObject {
    id: String;
    index: number;
    numbr: number;
    coordinates: Coordinate[]
}

interface DummyFriend {
    id: number;
    name: String;
}
interface DummyObject {
    _id: String;
    index: number;
    guid: String;
    isActive: boolean;
    balance: number;
    picture: String;
    age: number;
    eyeColor: String;
    name: String;
    gender: String;
    company: String;
    email: String;
    phone: String;
    address: String;
    about: String;
    registered: String;
    latitude: number;
    longitude: number;
    tags: String[];
    friends: DummyFriend[];
    greeting: String;
    favoriteFruit: String;
}
const App = () => {
    const [parsingTime, setParsingTime] = useState<number>(0);
    const [OBJS_AMNT, setObjectAmount] = useState<number>(0);
    const objectAmountRef = useRef(null);
    // let java_server_local = "/java";
    // let rust_server_local = "/rust";
    //For the development mode (when files are nothosted by apache2)
    let java_server_local = "http://localhost:8080";
    let rust_server_local = "http://localhost:8081";
    //Added UseEffect that calls health checks to java and rust server to establish connecion first
    useEffect(() => {
        fetch(java_server_local + '/healthCheck');
        fetch(rust_server_local + '/health_check');
    }, []);

    //Obtain the OBJS_AMNT of BSON objects, the type is specified by the parameter assigned on the button
    const getBSONObjectRust = async (objectType: string) => {
        console.log("############RUST - BSON")
        let totalParse = 0;
        for (let i = 0; i < OBJS_AMNT; i++) {
            if (i % 1000 === 0) {
                console.log("i: " + i)
            }
            const response = await fetch(`${rust_server_local}/BSONObject/${objectType}`)
            const t1 = Date.now();
            //Get the byte array from the response body
            const arb = await response.blob().then(blob => blob.arrayBuffer());
            if (objectType == "1") {
                //Parsing the response to the object
                const bson: NumericObject = BSON.deserialize(new Uint8Array(arb)) as NumericObject;
            } else if (objectType == "2") {
                //Parsing the response to the object
                const bson: DummyObject = BSON.deserialize(new Uint8Array(arb)) as DummyObject;
            }
            totalParse += Date.now() - t1;
        }
        console.log(`total time to parse ${OBJS_AMNT} objects is ${totalParse}`)
        setParsingTime(totalParse)
    }
    //Obtain the OBJS_AMNT of BSON objects, the type is specified by the parameter assigned on the button
    const getBSONObjectJava = async (objectType: string) => {
        console.log("############JAVA - BSON")
        let totalParse = 0;
        for (let i = 0; i < OBJS_AMNT; i++) {
            if (i % 1000 === 0) {
                console.log("i: " + i)
            }
            const response = await fetch(`${java_server_local}/BSONObject/${objectType}`)
            const t1 = Date.now();
            //Get the byte array from the response body
            const arb = await response.blob().then(blob => blob.arrayBuffer());
            if (objectType == "1") {
                //Parsing the response to the object
                const bson: NumericObject = BSON.deserialize(new Uint8Array(arb)) as NumericObject;
            } else if (objectType == "2") {
                //Parsing the response to the object
                const bson: DummyObject = BSON.deserialize(new Uint8Array(arb)) as DummyObject;
            }
            totalParse += Date.now() - t1;
        }
        const t2 = Date.now();
        console.log(`total time to parse ${OBJS_AMNT} objects is ${totalParse}`)
        setParsingTime(totalParse)
    }

    //Obtain the OBJS_AMNT of JSON objects, the type is specified by the parameter assigned on the button
    const getJSONObjectRust = async (objectType: string) => {
        console.log("############RUST - JSON")
        let totalParse = 0;
        for (let i = 0; i < OBJS_AMNT; i++) {
            if (i % 1000 === 0) {
                console.log("i: " + i)
            }
            const response = await fetch(`${rust_server_local}/JSONObject/${objectType}`);
            const t1 = Date.now();
            if (objectType == "1") {
                //Parse the JSON body
                const json: NumericObject = await response.json();
            } else if (objectType == "2") {
                //Parse the JSON body
                const json: DummyObject = await response.json();
            }
            totalParse += Date.now() - t1;
        }
        console.log(`total time to parse ${OBJS_AMNT} objects is ${totalParse}`)
        setParsingTime(totalParse)
    }
    //Obtain the OBJS_AMNT of JSON objects, the type is specified by the parameter assigned on the button
    const getJSONObjectJava = async (objectType: string) => {
        console.log("############JAVA - JSON")
        let totalParse = 0;
        const t1 = Date.now();
        for (let i = 0; i < OBJS_AMNT; i++) {
            if (i % 1000 === 0) {
                console.log("i: " + i)
            }
            const response = await fetch(`${java_server_local}/JSONObject/${objectType}`);
            const t1 = Date.now();
            if (objectType == "1") {
                //Parse the JSON body
                const json: NumericObject = await response.json();
            } else if (objectType == "2") {
                //Parse the JSON body
                const json: DummyObject = await response.json();
            }
            totalParse += Date.now() - t1;
        }
        console.log(`total time to parse ${OBJS_AMNT} objects is ${totalParse}`)
        setParsingTime(totalParse)
    }
    /**
     * The UI presents the user with an input to specify the amount of objects that the typescript-client should obtain.
     * The the user has to press one of the button to request the desired object from a certain server.
     * The combined parsing time from a single button action will be shown below all the buttons
    **/
    return (
        <div className="App">

            <div className="card">
                <h2>Amount of objects to request</h2>
                <input id="objects_amount_input" defaultValue={0} min={0} ref={objectAmountRef} type="number" onChange={(ev) => setObjectAmount(+ev.target.value)} />
            </div>
            <div className="card" style={{ marginRight: '1px solid #eee' }}>
                <button id="numeric_rust_json" style={{ margin: "10px", backgroundColor: "rgba(224, 126, 101,0.5)" }} onClick={() => getJSONObjectRust("1")}>
                    RustJSON NumericObject
                </button>
                <button id="numeric_java_json" style={{ margin: "10px", backgroundColor: "rgba(224, 167, 101,0.5)" }} onClick={() => getJSONObjectJava("1")}>
                    JavaJSON NumericObject
                </button>
                <br />
                <button id="numeric_rust_bson" style={{ margin: "10px", backgroundColor: "rgba(224, 126, 101,0.5)" }} onClick={() => getBSONObjectRust("1")}>
                    RustBSON NumericObject
                </button>
                <button id="numeric_java_bson" style={{ margin: "10px", backgroundColor: "rgba(224, 167, 101,0.5)" }} onClick={() => getBSONObjectJava("1")}>
                    JavaBSON NumericObject
                </button>
            </div>
            <div className="card">
                <button id="dummy_rust_json" style={{ margin: "10px", backgroundColor: "rgba(224, 126, 101,0.5)" }} onClick={() => getJSONObjectRust("2")}>
                    RustJSON DummyObject
                </button>
                <button id="dummy_java_json" style={{ margin: "10px", backgroundColor: "rgba(224, 167, 101,0.5)" }} onClick={() => getJSONObjectJava("2")}>
                    JavaJSON DummyObject
                </button>
                <br />
                <button id="dummy_rust_bson" style={{ margin: "10px", backgroundColor: "rgba(224, 126, 101,0.5)" }} onClick={() => getBSONObjectRust("2")}>
                    RustBSON DummyObject
                </button>
                <button id="dummy_java_bson" style={{ margin: "10px", backgroundColor: "rgba(224, 167, 101,0.5)" }} onClick={() => getBSONObjectJava("2")}>
                    JavaBSON DummyObject
                </button>
                {parsingTime != 0 ? <p>Parsing time: <span id="parsing_time">{parsingTime}</span> ms</p> : null}
            </div>
        </div>
    )
}

export default App

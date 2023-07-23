import { BSON, EJSON, ObjectId } from 'bson';
import axios from 'axios';
//java-server
/*console.log("############JAVA")
axios.get('http://localhost:8080/BSONObject/1')
    .then(r => {
        let buff = Buffer.from(r.data, "base64");
        console.log(buff);
        const oc = BSON.deserialize(buff);
        console.log(EJSON.stringify(oc));
    })
    .catch(err => console.log(err));*/
    const OBJS_AMNT = 5000;
const bson_obtain = async () => {

    //rust-server
    let totalParse = 0;
    console.log("############BSON")
    for (let i = 0; i < OBJS_AMNT; i++) {
        if (i % 1000 === 0) {
            console.log("i: " + i)
        }
        // const response = await axios.get('http://localhost:8080/BSONObject')
        const response2 = await fetch('http://localhost:8080/BSONObject')
        const t1 = Date.now();
        const arb = await response2.blob().then(blob => blob.arrayBuffer());
        const bson = BSON.deserialize(arb);
        totalParse += Date.now() - t1;
        // const response = await axios.get('http://localhost:8080/BSONObject')//.then((r)=>r.blob()).then(blob=>blob.arrayBuffer()
        // const t1 = Date.now();
        // const buff = Buffer.from(response.data);
        //     console.log(buff)
        // const bson = BSON.deserialize(Uint8Array.from(buff));
        // totalParse += Date.now() - t1;
    }
    console.log(`total time to parse ${OBJS_AMNT} objects is ${totalParse}`)
}
const json_obtain = async () => {
    console.log("############JSON")
    let totalParse = 0;
    for (let i = 0; i < OBJS_AMNT; i++) {
        if (i % 1000 === 0) {
            console.log("i: " + i)
        }
        const response = await fetch('http://localhost:8080/JSONObject')//.then((r)=>r.blob()).then(blob=>blob.arrayBuffer()
        const t1 = Date.now();
        const json = await response.json();
        totalParse += Date.now() - t1;
        /*axios.get<DummyObject>('http://localhost:8081/JSONObject')
            .then(r => {
                let t: DummyObject = r.data;
                objs.push(t)

            })
            .catch(err => console.log(err));*/
    }
    console.log(`total time to parse ${OBJS_AMNT} objects is ${totalParse}`)
}
await bson_obtain();
await json_obtain();
// axios.get('http://localhost:8081/BSONObject')
//     .then(r => {
//         let buff = Buffer.from(r.data, "base64");
//         console.log(buff);
//         const oc = BSON.deserialize(buff);
//         console.log(EJSON.stringify(oc));
//     })
//     .catch(err => console.log(err));

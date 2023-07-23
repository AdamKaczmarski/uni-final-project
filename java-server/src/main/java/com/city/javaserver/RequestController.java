package com.city.javaserver;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.undercouch.bson4jackson.BsonFactory;
import org.bson.RawBsonDocument;
import org.bson.json.JsonObject;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.core.io.ByteArrayResource;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.client.RestTemplate;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.time.Duration;
import java.time.Instant;
import java.util.HashMap;

import static org.springframework.http.MediaType.APPLICATION_JSON;
import static org.springframework.http.MediaType.APPLICATION_OCTET_STREAM;

@RestController
public class RequestController {
    private final static Logger LOGGER = LoggerFactory.getLogger(RequestController.class);
    private final byte[] numObjBsonBytes;
    private final String numObjJSON;
    private final String dummyObjJSON;
    private final byte[] dummyObjBsonBytes;

    public RequestController() throws IOException {
        //Creating test objects
        DummyObject dummyObject = new DummyObject("63dcdee3a473b6e562063a35", 0, "8c514ba0-f517-46ea-9dc3-dfd64c689bcf", false, 2166.53, "http://placehold.it/32x32", 29, "blue", "JosefaGrant", "female", "ZENTIA", "josefagrant@zentia.com", "+1(805)529-2872", "758HubbardStreet,Roderfield,Massachusetts,6249", "InsitvoluptatedolorLoremnostrudquisLoremauteet.Ametcillumnullacommodoullamcoidelitquiadipisicingfugiatreprehenderitconsequatut.Deseruntsitexcepteurutsuntmagnaesse.Culpanisiquiexcepteurinirureveniamculpatempor.Estipsumdeseruntlaboreesse.Proidentnoneasintincididunttemporeiusmodinnulladolore.\r\n", "2018-02-17T12:43:54-00:00", -62.176008, 123.106292, new String[]{"minim", "Lorem", "ea", "incididunt", "voluptate", "laboris", "ex"}, new DummyFriend[]{new DummyFriend(0, "KittyBrock"), new DummyFriend(1, "MirandaMoody"), new DummyFriend(2, "GeorgetteSherman")}, "Hello,JosefaGrant!Youhave10unreadmessages.", "apple");
        NumericObject numericObject = new NumericObject("5ef42a23d-4a77-49c5-be8a-7ced8479f915", 0, 36, new Coordinate[]{new Coordinate(8828480041L, -71.228786, -97.911745), new Coordinate(9299061322L, 29.996248, -1.727763), new Coordinate(9052787749L, -46.622638, 141.685921), new Coordinate(3491739584L, -59.283697, 35.835898), new Coordinate(1196235471L, -59.368215, -167.022009), new Coordinate(7014816742L, -35.010705, -75.416994), new Coordinate(3190810328L, 22.557889, -108.35877), new Coordinate(1756812663L, -12.775935, 66.956942), new Coordinate(9237021137L, 60.138851, -27.953068), new Coordinate(8377575119L, 70.411835, 25.353941)});
        //Jackson mapper
        ObjectMapper mapper = new ObjectMapper();
        //bson4jackson mapper
        ObjectMapper bsonMapper = new ObjectMapper(new BsonFactory());
        //Serialize the object to BSON document ByteArray
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        bsonMapper.writeValue(baos, numericObject);
        numObjBsonBytes = baos.toByteArray();
        baos.flush();
        baos = new ByteArrayOutputStream();
        bsonMapper.writeValue(baos, dummyObject);
        dummyObjBsonBytes = baos.toByteArray();
        baos.flush();
        //Convert objects to JSON
        numObjJSON = mapper.writeValueAsString(numericObject);
        dummyObjJSON = mapper.writeValueAsString(dummyObject);
    }

    /**
     * @param objectType a number indicating which object should be returned (This takes 2 two values, 1 for numericObject, 2 for dummyObject)
     * @return JSON Object
     */
    @GetMapping("/JSONObject/{objectType}")
    public ResponseEntity<String> getJSONObject(@PathVariable int objectType) {
        //Return the same value that's serialized once during the initialization
        if (objectType == 1) {
            return ResponseEntity.ok().contentType(APPLICATION_JSON).body(numObjJSON);
        } else if (objectType == 2) {
            return ResponseEntity.ok().contentType(APPLICATION_JSON).body(dummyObjJSON);
        } else {
            return ResponseEntity.badRequest().body("Wrong objectType, the allowed values are either 1 for NumericObject or 2 for DummyObject");
        }
    }

    /**
     * This function was made to showcase the amount of bytes JSON occupies in the literature review chapter
     *
     * @return JSON String
     */
    @GetMapping("/JSONbytes")
    public ResponseEntity<?> JSONbytes() {
        JsonObject jo2 = new JsonObject("{\"id\":123,\"active\":false,\"name\":\"Adam\",\"balance\":123.23,\"friendIDs\":[324,234,434]}");
        return ResponseEntity.ok().contentType(APPLICATION_JSON).body(jo2.getJson());
    }

    /**
     * @param objectType a number indicating which object should be returned (This takes 2 two values, 1 for numericObject, 2 for dummyObject)
     * @return byte array of the requested BSON object
     */
    @GetMapping("/BSONObject/{objectType}")
    public ResponseEntity<?> getBSONObject(@PathVariable int objectType) {
        //Return the same value that's serialized once during the initialization
        if (objectType == 1) {
            return ResponseEntity.ok().contentType(APPLICATION_OCTET_STREAM).body(numObjBsonBytes);
        } else if (objectType == 2) {
            return ResponseEntity.ok().contentType(APPLICATION_OCTET_STREAM).body(dummyObjBsonBytes);
        } else {
            return ResponseEntity.badRequest().body("Wrong objectType, the allowed values are either 1 for NumericObject or 2 for DummyObject");
        }
    }

    /**
     * For the front-end to establish connection with the server (helped when setting up apache httpd)
     *
     * @return ResponseEntity with status code
     */
    @GetMapping("/healthCheck")
    public ResponseEntity<?> healthCheck() {
        return ResponseEntity.ok().build();
    }

    /**
     * The function is used for measuring the parsing time of a specified object serialized in a specified format
     * @param sendObjectsDTO Object containing parameters for the benchmarking
     * @return A JSON string with the results of the benchmark
     * @throws URISyntaxException
     * @throws IOException
     */
    @PostMapping("/triggerReqs")
    public ResponseEntity<?> triggerReqs(@RequestBody SendObjectsDTO sendObjectsDTO) throws URISyntaxException, IOException {
        LOGGER.info(sendObjectsDTO.toString());
        //The JSON containing results of the benchmark
        HashMap<String, Long> results = new HashMap<>();
        results.put("amount", (long) sendObjectsDTO.getAmount());
        results.put(sendObjectsDTO.getServerType(), 1L);
        RestTemplate restTemplate = new RestTemplate();
        //Define URLs the server providers and define the objectType the function should request
        URI uriBSON;
        URI uriJSON;
        if (sendObjectsDTO.getObjectType().equals(ObjectType.NumericObject)) {
            uriBSON = new URI(sendObjectsDTO.getProviderUrl() + "/BSONObject/1");
            uriJSON = new URI(sendObjectsDTO.getProviderUrl() + "/JSONObject/1");
        } else if (sendObjectsDTO.getObjectType().equals(ObjectType.DummyObject)) {
            uriBSON = new URI(sendObjectsDTO.getProviderUrl() + "/BSONObject/2");
            uriJSON = new URI(sendObjectsDTO.getProviderUrl() + "/JSONObject/2");
        } else {
            return ResponseEntity.badRequest().body("Wrong objectType, please choose 1 for NumericObject or 2 for DummyObject");
        }

        ObjectMapper bsonMapper = new ObjectMapper(new BsonFactory());
        ObjectMapper jsonMapper = new ObjectMapper();

        long totalTime = 0L;
        long parsingTime = 0L;
        if (sendObjectsDTO.getObjectFormat().equals(ObjectFormat.BSON)) {
            LOGGER.info("############### BSON ##################");
            Instant start = Instant.now();
            //Request as many object as specified in the request
            for (int i = 0; i < sendObjectsDTO.getAmount(); i++) {
                ResponseEntity<ByteArrayResource> response = restTemplate.getForEntity(uriBSON, ByteArrayResource.class);
                if (response.getBody() != null) {
                    //Time only the parsing time
                    Instant startParsing = Instant.now();
                    //  RawBsonDocument rawBsonDocument = new RawBsonDocument(response.getBody().getByteArray());
                    //  bson4jackson && jackson is slower than Mongo 's RawDocument/Document
                    // RawBsonDocument doesn't allow us to use the fields of th send object (when using rawBsonDocument.getField()
                    // whereas using bson4jackson instantiates the object based on the BSON data
                    if (sendObjectsDTO.getObjectType().equals(ObjectType.DummyObject)) {
                        DummyObject dummyObj = bsonMapper.readValue(response.getBody().getByteArray(), DummyObject.class);
                    } else if (sendObjectsDTO.getObjectType().equals(ObjectType.NumericObject)) {
                        NumericObject numObj = bsonMapper.readValue(response.getBody().getByteArray(), NumericObject.class);
                    }
                    Instant finishParsing = Instant.now();
                    parsingTime += Duration.between(startParsing, finishParsing).toNanos();
                }
            }
            Instant finish = Instant.now();
            totalTime = Duration.between(start, finish).toMillis();
            results.put("bson_total_time_ms", totalTime);
            results.put("bson_parsing_time_micro_s", parsingTime / 1000);
            LOGGER.info(String.format("It took %dms to parse %d objects", parsingTime / 1000000, sendObjectsDTO.getAmount()));
            LOGGER.info(String.format("It took %dms to obtain and parse %d objects", totalTime, sendObjectsDTO.getAmount()));
            LOGGER.info("#######################################");
            return ResponseEntity.ok(jsonMapper.writeValueAsString(results));
        }
        if (sendObjectsDTO.getObjectFormat().equals(ObjectFormat.JSON)) {
            LOGGER.info("############### JSON ##################");
            Instant start = Instant.now();
            for (int i = 0; i < sendObjectsDTO.getAmount(); i++) {
                ResponseEntity<String> response = restTemplate.getForEntity(uriJSON, String.class);
                if (response.getBody() != null) {
                    Instant startParsing = Instant.now();
                    if (sendObjectsDTO.getObjectType().equals(ObjectType.DummyObject)) {
                        DummyObject numObj = jsonMapper.readValue(response.getBody(), DummyObject.class);
                    } else if (sendObjectsDTO.getObjectType().equals(ObjectType.NumericObject)) {
                        NumericObject numObj = jsonMapper.readValue(response.getBody(), NumericObject.class);
                    }
                    Instant finishParsing = Instant.now();
                    parsingTime += Duration.between(startParsing, finishParsing).toNanos();
                }
            }
            Instant finish = Instant.now();
            totalTime = Duration.between(start, finish).toMillis();
            results.put("json_total_time_ms", totalTime);
            results.put("json_parsing_time_micro_s", parsingTime / 1000);
            LOGGER.info(String.format("It took %dms to parse %d objects", parsingTime / 1000000, sendObjectsDTO.getAmount()));
            LOGGER.info(String.format("It took %dms to obtain and parse %d objects", totalTime, sendObjectsDTO.getAmount()));
            LOGGER.info("#######################################");
            return ResponseEntity.ok(jsonMapper.writeValueAsString(results));
        }
        return ResponseEntity.badRequest().build();
    }

    /*
    This function is not used in the project, it was developed for some testing, but it was decided
    that the benchmark should be made using the triggerReqs()
     */
/*    @PostMapping("/triggerReqsTime/{time}")
    public ResponseEntity<?> triggerReqsTime(@PathVariable long time, @RequestBody SendObjectsDTO sendObjectsDTO) throws URISyntaxException, IOException {
        System.out.println(sendObjectsDTO);
        HashMap<String, Long> results = new HashMap<>();
        results.put("time", time);
        results.put(sendObjectsDTO.getServerType(), 1L);
        RestTemplate restTemplate = new RestTemplate();
        URI uriBSON;
        URI uriJSON;
        if (sendObjectsDTO.getObjectType().equals(ObjectType.NumericObject)) {
            uriBSON = new URI(sendObjectsDTO.getProviderUrl() + "/BSONObject/1");
            uriJSON = new URI(sendObjectsDTO.getProviderUrl() + "/JSONObject/1");
        } else if (sendObjectsDTO.getObjectType().equals(ObjectType.DummyObject)) {
            uriBSON = new URI(sendObjectsDTO.getProviderUrl() + "/BSONObject/2");
            uriJSON = new URI(sendObjectsDTO.getProviderUrl() + "/JSONObject/2");
        } else {
            return ResponseEntity.badRequest().body("Wrong objectType, please choose 1 for NumericObject or 2 for DummyObject");
        }
        ObjectMapper bsonMapper = new ObjectMapper(new BsonFactory());
        ObjectMapper jsonMapper = new ObjectMapper();

        long parsingTime = 0L;
        int counter = 0;
        if (sendObjectsDTO.getObjectFormat().equals(ObjectFormat.BSON)) {
            LOGGER.info("############### BSON ##################");
            long cutoff = Instant.now().plusMillis(time).toEpochMilli();
            while (Instant.now().toEpochMilli() < cutoff) {
                ResponseEntity<ByteArrayResource> response = restTemplate.getForEntity(uriBSON, ByteArrayResource.class);
                Instant startParsing = Instant.now();
//                RawBsonDocument rawBsonDocument = new RawBsonDocument(response.getBody().getByteArray());
                //bson4jackson && jackson is slower than Mongo's RawDocument/Document
                //gonna use bson4jackson because we want to get the objects not Documents
                if (sendObjectsDTO.getObjectType().equals(ObjectType.DummyObject)) {
                    DummyObject dummyObj = bsonMapper.readValue(response.getBody().getByteArray(), DummyObject.class);
                } else if (sendObjectsDTO.getObjectType().equals(ObjectType.NumericObject)) {
                    NumericObject numObj = bsonMapper.readValue(response.getBody().getByteArray(), NumericObject.class);
                }
                Instant finishParsing = Instant.now();
                counter += 1;
                parsingTime += Duration.between(startParsing, finishParsing).toNanos();
            }
            results.put("bson_parsing_time_ms", parsingTime / 1000000);
            results.put("amount_objects", (long) counter);
            LOGGER.info(String.format("Parsed %d objects in %d", counter, time));
            LOGGER.info("#######################################");
            return ResponseEntity.ok(jsonMapper.writeValueAsString(results));
        }
        if (sendObjectsDTO.getObjectFormat().equals(ObjectFormat.JSON)) {
            LOGGER.info("############### JSON ##################");
            long cutoff = Instant.now().plusMillis(time).toEpochMilli();
            while (Instant.now().toEpochMilli() < cutoff) {
                ResponseEntity<String> response = restTemplate.getForEntity(uriJSON, String.class);
                Instant startParsing = Instant.now();
//                Document doc = Document.parse(response.getBody());
                //jackson is slower than Mongo's RawDocument/Document
                if (sendObjectsDTO.getObjectType().equals(ObjectType.DummyObject)) {
                    DummyObject numObj = jsonMapper.readValue(response.getBody(), DummyObject.class);
                } else if (sendObjectsDTO.getObjectType().equals(ObjectType.NumericObject)) {
                    NumericObject numObj = jsonMapper.readValue(response.getBody(), NumericObject.class);
                }
                Instant finishParsing = Instant.now();
                counter += 1;
                parsingTime += Duration.between(startParsing, finishParsing).toNanos();
            }
            results.put("json_parsing_time_ms", parsingTime / 1000000);
            results.put("amount_objects", (long) counter);
            LOGGER.info(String.format("Parsed %d objects in %d", counter, time));
            LOGGER.info("#######################################");
            return ResponseEntity.ok(jsonMapper.writeValueAsString(results));
        }
        return ResponseEntity.badRequest().build();
    }*/
}
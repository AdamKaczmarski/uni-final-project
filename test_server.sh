#/usr/bin/bash
echo "########## -- JAVA JSON"
 ab -c 10 -n 10000000 http://localhost:8080/JSONObject | grep -E 'Time taken|Requests per second|Transfer rate'  
echo "########## -- JAVA BSON"
 ab -c 10 -n 10000000 http://localhost:8080/BSONObject | grep -E 'Time taken|Requests per second|Transfer rate'  
echo "########## -- RUST JSON"
 ab -c 10 -n 10000000 http://localhost:8081/JSONObject | grep -E 'Time taken|Requests per second|Transfer rate'  
echo "########## -- RUST BSON"
 ab -c 10 -n 10000000 http://localhost:8081/BSONObject | grep -E 'Time taken|Requests per second|Transfer rate'  

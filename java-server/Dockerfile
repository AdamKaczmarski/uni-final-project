FROM eclipse-temurin:17-alpine AS builder
WORKDIR /app
COPY . .
RUN apk update
RUN ./mvnw dependency:resolve
RUN ./mvnw package -Pdocker


FROM eclipse-temurin:17-jre-alpine
WORKDIR /app
RUN apk update
COPY --from=builder /app/target/*.jar ./java-server.jar
ENTRYPOINT ["java","-Xms1024m","-Xmx1024m","-XX:+UseG1GC","-Dlogging.file.path=/var/log","-Xlog:gc:/var/log/spring.log","-jar","./java-server.jar"]


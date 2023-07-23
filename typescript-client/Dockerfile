FROM node:18-alpine AS builder
WORKDIR /ui
COPY . . 
RUN npm install
RUN npm run build

FROM ubuntu/apache2:2.4-22.10_beta
COPY ./apache_config_docker /etc/apache2
COPY --from=builder /ui/dist /var/www/html



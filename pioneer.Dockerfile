FROM node:14 as builder

WORKDIR /joystream
COPY . /joystream

RUN yarn
RUN NODE_ENV=production yarn workspace @joystream/types build
RUN NODE_ENV=production yarn workspace pioneer build

FROM ubuntu:18.04

RUN apt-get update && apt-get -y install nginx

COPY --from=builder /joystream/pioneer/packages/apps/build /var/www/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

FROM node:14 as builder

WORKDIR /joystream
COPY . /joystream
RUN rm -fr /joystream/pioneer
# Replaced by "integration-tests" on Olympia
RUN rm -fr /joystream/tests/network-tests

ARG TYPEGEN_WS_URI

# Do not set NODE_ENV=production until after running yarn install
# to ensure dev dependencies are installed.
RUN yarn --frozen-lockfile

# @joystream/types are built during postinstall
RUN yarn workspace storage-node build
RUN yarn workspace query-node-root build

ENTRYPOINT [ "yarn" ]

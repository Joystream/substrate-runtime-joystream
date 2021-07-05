FROM node:14 as builder

WORKDIR /joystream
COPY . /joystream
RUN  rm -fr /joystream/pioneer

# Do not set NODE_ENV=production until after running yarn install
# to ensure dev dependencies are installed.
RUN yarn --forzen-lockfile

RUN yarn workspace @joystream/types build
RUN yarn workspace query-node-root build
RUN yarn workspace storage-node build

# Second stage to reduce image size, enable it when
# all packages have correctly identified what is a devDependency and what is not.
# It will reduce the image size by about 500MB (down from 2.2GB to 1.7GB)

# # Remove files that are not needed after build.
# # We will re-fetch only dependencies needed for running the apps.
# RUN rm -fr node_modules/
# RUN rm -fr .git/

# FROM node:12
# WORKDIR /joystream
# COPY --from=builder /joystream/ /joystream/

# # Skip installing devDependencies, since we have already built the packages.
# ENV NODE_ENV=production
# RUN yarn install --forzen-lockfile --production

ENTRYPOINT [ "yarn" ]

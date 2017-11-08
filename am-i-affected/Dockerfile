
FROM node AS builder
WORKDIR /src

# Restore dependencies
COPY ./package.json .
COPY ./yarn.lock .
RUN yarn install

# Build
COPY . .
RUN yarn run build

FROM nginx
COPY --from=builder /src/build/ /usr/share/nginx/html/

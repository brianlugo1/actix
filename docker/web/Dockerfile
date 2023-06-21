
#-----------------------------------------------------------------------#
# Docker Container to run the Actix application!                        #
#                                                                       #
# Run the two following commands to build a docker image from this      #
#   Dockerfile and run single a docker container                        #
#                                                                       #
#   - `docker build -t test .; docker run -p 3001:80 test;`             #
#       - builds an image from this Dockerfile, specifing               #
#           the name of `test` with the `-t` flag and the               #
#           directory `.` to find this Dockerfile                       #
#       - runs the built image `test` to create a docker container,     #
#           specifing the port to connect on the local machine          #
#           `3001` to the port in the docker container `80`             #
#-----------------------------------------------------------------------#

FROM rust
COPY ../../../actix /usr/src/app
WORKDIR /usr/src/app
RUN cargo install --path .
EXPOSE 3001
CMD [ "actix" ]
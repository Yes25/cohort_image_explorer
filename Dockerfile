# FROM scratch
FROM debian

ADD "./image_server/target/release/image_server" "/app/image_server"
ADD "./frontend-cohort-image-explorer/dist" "/app/dist"

CMD ["/app/image_server"]

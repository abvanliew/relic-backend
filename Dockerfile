FROM mongo:8
COPY ./data/mongo/backup /data/backup
# RUN mongod --sysinfo
# RUN nohup bash -c "mongod" >&/dev/null

# && \
#     ps -ef && \
#     mongorestore --nsInclude="relic.*" /data/backup/main

# FROM mongo:8

# Start MongoDB, restore the backup, shut it down
RUN mkdir -p /data/db \
 && mongod --bind_ip 127.0.0.1 --fork --logpath /tmp/mongod.log \
 && until mongosh --quiet --eval "db.adminCommand({ ping: 1 })" >/dev/null 2>&1; do sleep 1; done \
 && mongorestore --nsInclude="relic.*" /data/backup/main \
 && mongod --shutdown

# Stage 2: Final runtime image
FROM mongo:8

COPY --from=builder /data/db /data/db

FROM mongo:8
COPY ./data/mongo/backup /data/backup
# RUN mongod --sysinfo
# RUN nohup bash -c "mongod" >&/dev/null

# && \
#     ps -ef && \
#     mongorestore --nsInclude="relic.*" /data/backup/main
# create new mongo pod
podman kube play --replace ./kube/mongo.yaml

# start podman
podman machine start

# start mongodb
podman start mongo-db

podman exec -it mongo-db bash

# backcommand when in pod shell
mongodump --db relic --out /data/backup/`date +"%Y-%m-%d"`
mongorestore --db relic /data/backup/`date +"%Y-%m-%d"`/relic

# quick exec commands
# podman exec mongo-db bash -c 'mongodump --db relic --out /data/backup/`date +"%Y-%m-%d"` && echo `date +"%Y-%m-%d"`'
podman exec mongo-db bash -c 'mongodump --db relic --out /data/backup/main'
podman exec mongo-db bash -c 'mongoexport -d relic -c skills -o /data/backup/skills.json'

# podman exec mongo-db bash -c 'mongorestore --db relic /data/backup/`date +"%Y-%m-%d"`/relic' 
# podman exec mongo-db bash -c 'mongorestore --db relic /data/backup/2024-12-18/relic'
podman exec mongo-db bash -c 'mongorestore --db relic /data/backup/main/relic'
podman exec mongo-db bash -c 'mongorestore --drop --db relic /data/backup/main/relic'


mongoexport -d relic -c skills -o /data/backup/skills.json
mongoimport --db=relic --collection=skills --file=/data/backup/upsert.json --mode upsert

mongodump --db relic --out /data/backup/`date +"%Y-%m-%d"`
mongorestore --db relic /data/backup/`date +"%Y-%m-%d"`/relic

podman exec mongo-db bash -c 'mongodump --db relic --out /data/backup/`date +"%Y-%m-%d"`'
podman exec mongo-db bash -c 'mongorestore --db relic /data/backup/`date +"%Y-%m-%d"`/relic'